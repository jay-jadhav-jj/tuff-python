use std::{
    fmt,
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

use crate::{compiler, filehandling, language_names::Language};

const CONNECTION_CHECK_TEXT: &str = "tpy translation connection check";
const TRACEBACK_MARKER: &[u8] = b"Traceback (most recent call last):";
static NEXT_TEMP_SCRIPT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct RunError {
    pub message: String,
    pub exit_code: i32,
}

struct RunContext<'a> {
    input_path: &'a Path,
    source: &'a str,
    target_language: &'a str,
    translation_available: bool,
}

impl RunError {
    fn compile(message: String) -> Self {
        Self {
            message,
            exit_code: 2,
        }
    }

    fn execution(message: String) -> Self {
        Self {
            message,
            exit_code: 1,
        }
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

pub fn run_file<E, F>(input: &str, translate: F) -> Result<ExitStatus, RunError>
where
    E: fmt::Display,
    F: FnMut(&str, &str, &str) -> Result<String, E>,
{
    run_file_with_io(input, translate, io::stdout(), io::stderr())
}

fn run_file_with_io<E, F, W, S>(
    input: &str,
    mut translate: F,
    stdout: W,
    mut stderr: S,
) -> Result<ExitStatus, RunError>
where
    E: fmt::Display,
    F: FnMut(&str, &str, &str) -> Result<String, E>,
    W: Write + Send + 'static,
    S: Write,
{
    let input_path = Path::new(input);
    let source = filehandling::read_file(input_path).map_err(RunError::compile)?;
    let language = Language::from_source(&source).map_err(RunError::compile)?;
    let target_language = language.translation_code();

    let translation_available = match translate(CONNECTION_CHECK_TEXT, "en", target_language) {
        Ok(_) => true,
        Err(error) => {
            writeln!(
                stderr,
                "Could not connect to Google Translate; runtime errors will be shown in English: {error}"
            )
            .map_err(|error| RunError::execution(format!("Failed to write to stderr: {error}")))?;
            false
        }
    };

    let compiled = compiler::compile_source_with_diagnostics(&source).map_err(RunError::compile)?;
    for error in &compiled.alias_errors {
        writeln!(stderr, "alias error: {error}")
            .map_err(|error| RunError::execution(format!("Failed to write to stderr: {error}")))?;
    }

    let script_path = write_temp_script(input_path, &compiled.code)?;
    let execution = execute_python(
        &script_path,
        RunContext {
            input_path,
            source: &source,
            target_language,
            translation_available,
        },
        translate,
        stdout,
        &mut stderr,
    );
    let cleanup = fs::remove_file(&script_path).map_err(|error| {
        RunError::execution(format!(
            "Failed to remove temporary script '{}': {error}",
            script_path.display()
        ))
    });

    match (execution, cleanup) {
        (Ok(status), Ok(())) => Ok(status),
        (Err(error), Ok(())) => Err(error),
        (Ok(_), Err(error)) | (Err(_), Err(error)) => Err(error),
    }
}

fn write_temp_script(input_path: &Path, code: &str) -> Result<PathBuf, RunError> {
    let parent = input_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    for _ in 0..16 {
        let id = NEXT_TEMP_SCRIPT.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(".tpy-run-{}-{id}.py", std::process::id()));
        let mut file = match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(RunError::execution(format!(
                    "Failed to create temporary script '{}': {error}",
                    path.display()
                )));
            }
        };

        if let Err(error) = file.write_all(code.as_bytes()) {
            let cleanup = fs::remove_file(&path);
            let detail = match cleanup {
                Ok(()) => String::new(),
                Err(cleanup_error) => {
                    format!("; failed to remove partial script: {cleanup_error}")
                }
            };
            return Err(RunError::execution(format!(
                "Failed to write temporary script '{}': {error}{detail}",
                path.display()
            )));
        }

        return Ok(path);
    }

    Err(RunError::execution(
        "Could not create a unique temporary script file".to_string(),
    ))
}

fn execute_python<E, F, W, S>(
    script_path: &Path,
    context: RunContext<'_>,
    mut translate: F,
    mut stdout: W,
    stderr: &mut S,
) -> Result<ExitStatus, RunError>
where
    E: fmt::Display,
    F: FnMut(&str, &str, &str) -> Result<String, E>,
    W: Write + Send + 'static,
    S: Write,
{
    let RunContext {
        input_path,
        source,
        target_language,
        translation_available,
    } = context;
    let script_path = fs::canonicalize(script_path).map_err(|error| {
        RunError::execution(format!(
            "Failed to resolve temporary script '{}': {error}",
            script_path.display()
        ))
    })?;
    let mut child = Command::new("python")
        .arg(&script_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| RunError::execution(format!("Failed to start Python: {error}")))?;

    let child_stdout = child
        .stdout
        .take()
        .ok_or_else(|| RunError::execution("Python stdout was not captured".to_string()))?;
    let child_stderr = child
        .stderr
        .take()
        .ok_or_else(|| RunError::execution("Python stderr was not captured".to_string()))?;

    let stdout_thread = thread::spawn(move || {
        let mut child_stdout = child_stdout;
        io::copy(&mut child_stdout, &mut stdout)?;
        stdout.flush()
    });
    let stderr_thread = thread::spawn(move || {
        let mut child_stderr = child_stderr;
        let mut output = Vec::new();
        child_stderr.read_to_end(&mut output)?;
        Ok::<_, io::Error>(output)
    });

    let status = child.wait();
    let stdout_result = stdout_thread
        .join()
        .map_err(|_| RunError::execution("The stdout forwarding thread panicked".to_string()))?
        .map_err(|error| RunError::execution(format!("Failed to forward Python stdout: {error}")));
    let stderr_output = stderr_thread
        .join()
        .map_err(|_| RunError::execution("The stderr capture thread panicked".to_string()))?
        .map_err(|error| RunError::execution(format!("Failed to capture Python stderr: {error}")));

    let status = status
        .map_err(|error| RunError::execution(format!("Failed to wait for Python: {error}")))?;
    stdout_result?;
    let stderr_output = stderr_output?;

    let stderr_output = if !status.success() {
        let mapped = map_traceback_to_source(&stderr_output, &script_path, input_path, source);
        if translation_available {
            match translate_exception_line(&mapped, target_language, &mut translate) {
                Ok(Some(translated)) => translated,
                Ok(None) => mapped,
                Err(error) => {
                    writeln!(
                        stderr,
                        "Could not connect to Google Translate; this runtime error will be shown in English: {error}"
                    )
                    .map_err(|error| {
                        RunError::execution(format!("Failed to write to stderr: {error}"))
                    })?;
                    mapped
                }
            }
        } else {
            mapped
        }
    } else {
        stderr_output
    };

    stderr
        .write_all(&stderr_output)
        .and_then(|()| stderr.flush())
        .map_err(|error| {
            RunError::execution(format!("Failed to forward Python stderr: {error}"))
        })?;

    Ok(status)
}

fn map_traceback_to_source(
    stderr: &[u8],
    script_path: &Path,
    input_path: &Path,
    source: &str,
) -> Vec<u8> {
    let script_path = script_path.to_string_lossy();
    let input_path = input_path.to_string_lossy();
    let source_lines = source.lines().collect::<Vec<_>>();
    let text = String::from_utf8_lossy(stderr);
    let mut mapped = String::with_capacity(text.len());
    let mut pending_source_line = None;

    for line in text.split_inclusive('\n') {
        let content = line.trim_end_matches(['\r', '\n']);
        let line_ending = &line[content.len()..];

        if let Some(source_line) = traceback_source_line(content, &script_path) {
            mapped.push_str(&line.replace(script_path.as_ref(), input_path.as_ref()));
            pending_source_line = Some(source_line);
        } else if let Some(source_line) = pending_source_line.take() {
            if content.starts_with("    ")
                && !content.trim().is_empty()
                && let Some(original_line) = source_lines.get(source_line.saturating_sub(1))
            {
                mapped.push_str("    ");
                mapped.push_str(original_line.trim());
                mapped.push_str(line_ending);
            } else {
                mapped.push_str(line);
            }
        } else {
            mapped.push_str(line);
        }
    }

    mapped.into_bytes()
}

fn traceback_source_line(line: &str, script_path: &str) -> Option<usize> {
    let rest = line.trim_start().strip_prefix("File \"")?;
    let (path, details) = rest.split_once("\", line ")?;
    if path != script_path {
        return None;
    }
    let digits = details
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>();
    digits.parse().ok()
}

fn translate_exception_line<E, F>(
    stderr: &[u8],
    target_language: &str,
    translate: &mut F,
) -> Result<Option<Vec<u8>>, String>
where
    E: fmt::Display,
    F: FnMut(&str, &str, &str) -> Result<String, E>,
{
    if !stderr
        .windows(TRACEBACK_MARKER.len())
        .any(|window| window == TRACEBACK_MARKER)
    {
        return Ok(None);
    }

    let Some(last_content_index) = stderr
        .iter()
        .rposition(|byte| *byte != b'\n' && *byte != b'\r')
    else {
        return Ok(None);
    };
    let last_content_end = last_content_index + 1;
    let line_start = stderr[..last_content_end]
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(0, |index| index + 1);
    if line_start == last_content_end {
        return Ok(None);
    }

    let exception = String::from_utf8_lossy(&stderr[line_start..last_content_end]);
    let translated =
        translate(exception.trim(), "en", target_language).map_err(|error| error.to_string())?;
    if translated.is_empty() {
        return Err("Google Translate returned an empty error translation".to_string());
    }

    let mut output = Vec::with_capacity(stderr.len() + translated.len());
    output.extend_from_slice(&stderr[..line_start]);
    output.extend_from_slice(translated.as_bytes());
    output.extend_from_slice(&stderr[last_content_end..]);
    Ok(Some(output))
}

