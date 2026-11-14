use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

struct TestDir(PathBuf);

impl TestDir {
    fn new() -> Self {
        let id = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("tuffpython-cli-test-{}-{id}", std::process::id()));
        fs::create_dir(&path).expect("create test directory");
        Self(path)
    }

    fn file(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove test directory");
    }
}

fn run_compile(input: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_tpy"))
        .arg("compile")
        .arg(input)
        .arg(output)
        .output()
        .expect("run tpy")
}

#[test]
fn short_builtin_names_work_in_all_languages() {
    let dir = TestDir::new();
    let examples = [
        (
            "french",
            "#!fr\nnombre = 7\nimprimez(max([2, 7]), min([2, 7]), long(\"abc\"), \"nombre\" en locales())\n",
        ),
        (
            "spanish",
            "#!es\nnumero = 7\nimprimir(max([2, 7]), min([2, 7]), largo(\"abc\"), \"numero\" en locales())\n",
        ),
        (
            "hindi",
            "#!hi\nnumber = 7\nmax([2, 7]), min([2, 7]), lambai(\"abc\"), \"number\" mein sthaniya() likho\n",
        ),
        (
            "marathi",
            "#!mr\nnumber = 7\nmax([2, 7]), min([2, 7]), lambai(\"abc\"), \"number\" madhye sthaniya() chhapa\n",
        ),
        (
            "chinese",
            "#!zh\nnumber = 7\ndayin(max([2, 7]), min([2, 7]), changdu(\"abc\"), \"number\" zai bendi())\n",
        ),
    ];

    for (language, source) in examples {
        let input = dir.file(&format!("{language}-short-builtins.tpy"));
        let output = dir.file(&format!("{language}-short-builtins.py"));
        fs::write(&input, source).expect("write source");
        let compile = run_compile(&input, &output);

        assert!(compile.status.success(), "{language}");
        assert!(compile.stderr.is_empty(), "{language}");
        let execution = Command::new("python")
            .arg(&output)
            .output()
            .expect("run generated Python");
        assert!(
            execution.status.success(),
            "{language}: {}",
            String::from_utf8_lossy(&execution.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&execution.stdout)
                .lines()
                .collect::<Vec<_>>(),
            ["7 2 3 True"],
            "{language}"
        );
    }
}

#[test]
fn symbolic_assignments_are_preserved_in_all_languages() {
    let dir = TestDir::new();
    for code in ["fr", "es", "hi", "mr", "zh"] {
        let input = dir.file(&format!("symbolic-assignment-{code}.tpy"));
        let output = dir.file(&format!("symbolic-assignment-{code}.py"));
        fs::write(
            &input,
            format!("#!{code}\nnumber = 0\nnumber += 1\nnumber *= 2\nnumber -= 1\nprint(number)\n"),
        )
        .expect("write input");

        let compile = run_compile(&input, &output);

        assert!(compile.status.success(), "{code}");
        assert!(compile.stderr.is_empty(), "{code}");
        let generated = fs::read_to_string(&output).expect("read generated Python");
        assert!(
            generated.contains("number = 0\nnumber += 1\nnumber *= 2\nnumber -= 1\nprint(number)"),
            "{code}: {generated}"
        );
        let execution = Command::new("python")
            .arg(&output)
            .output()
            .expect("run generated Python");
        assert!(
            execution.status.success(),
            "{code}: {}",
            String::from_utf8_lossy(&execution.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&execution.stdout).trim(),
            "1",
            "{code}"
        );
    }
}

#[test]
fn all_language_examples_compile_and_produce_matching_output() {
    let examples_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .join("tests");
    let expected = [
        ("01_greeting.tpy", vec!["Hello"]),
        ("02_values.tpy", vec!["5"]),
        ("03_conditions.tpy", vec!["yes"]),
        ("04_loops.tpy", vec!["0", "1", "2"]),
        ("05_functions_aliases.tpy", vec!["Hello Ada"]),
        ("06_short_builtins.tpy", vec!["7", "2", "3", "True"]),
        ("07_classes.tpy", vec!["Hello"]),
        ("08_import_aliases.tpy", vec!["3.0"]),
        ("09_while_exceptions.tpy", vec!["handled", "2"]),
    ];
    let dir = TestDir::new();

    for language in ["french", "spanish", "hindi", "marathi", "chinese"] {
        let language_dir = examples_root.join(language);
        let mut files = fs::read_dir(&language_dir)
            .expect("read language examples")
            .map(|entry| entry.expect("read example entry").path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "tpy"))
            .collect::<Vec<_>>();
        files.sort();
        assert_eq!(files.len(), expected.len(), "{language} example count");

        for (index, input) in files.iter().enumerate() {
            let name = input
                .file_name()
                .expect("example filename")
                .to_string_lossy();
            assert_eq!(name, expected[index].0, "{language} example ordering");
            let output = dir.file(&format!("{language}-{name}.py"));
            let compile = run_compile(input, &output);
            assert!(
                compile.status.success(),
                "{language}/{name}: {}",
                String::from_utf8_lossy(&compile.stderr)
            );
            assert!(
                compile.stderr.is_empty(),
                "{language}/{name}: {}",
                String::from_utf8_lossy(&compile.stderr)
            );

            let execution = Command::new("python")
                .arg(&output)
                .output()
                .expect("run generated Python");
            assert!(
                execution.status.success(),
                "{language}/{name}: {}",
                String::from_utf8_lossy(&execution.stderr)
            );
            let stdout = String::from_utf8_lossy(&execution.stdout);
            let actual = stdout.lines().collect::<Vec<_>>();
            assert_eq!(actual, expected[index].1, "{language}/{name}");
        }
    }
}

#[test]
fn compiles_uppercase_french_directive() {
    let dir = TestDir::new();
    let input = dir.file("hello.fr.tpy");
    let output = dir.file("hello.py");
    fs::write(&input, "#!FR\nimprimez(\"Bonjour\")\n").expect("write input");

    let result = run_compile(&input, &output);

    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    assert_eq!(
        fs::read_to_string(output).expect("read output"),
        "\nprint(\"Bonjour\")\n"
    );
}

#[test]
fn compiles_and_runs_spanish_source() {
    let dir = TestDir::new();
    let input = dir.file("hola.tpy");
    let output = dir.file("hola.py");
    fs::write(
        &input,
        "#!ES\nfuncion saludo(nombre):\n    retorna \"Hola \" + nombre\nimprimir(saludo(\"Ada\"))\nsi cierto entonces:\n    imprimir(largo(\"ok\"))\nsi falso entonces:\n    imprimir(\"mal\")\no si cierto entonces:\n    imprimir(\"rama\")\nsino:\n    imprimir(\"mal\")\npara item en rango(2):\n    imprimir(item)\n",
    )
    .expect("write input");

    let result = run_compile(&input, &output);

    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    let generated = fs::read_to_string(&output).expect("read output");
    assert!(generated.contains("def saludo(nombre):"), "{generated}");
    assert!(generated.contains("if True:"), "{generated}");
    let execution = Command::new("python")
        .arg(&output)
        .output()
        .expect("run generated Python");
    assert!(execution.status.success());
    assert_eq!(
        String::from_utf8_lossy(&execution.stdout)
            .lines()
            .collect::<Vec<_>>(),
        ["Hola Ada", "2", "rama", "0", "1"]
    );
}

#[test]
fn romanized_marathi_and_chinese_conditionals_and_keywords_run() {
    let dir = TestDir::new();
    for (code, source) in [
        (
            "mr",
            "#!mr\njar nahi khota kiwa khara tar:\n    chhapa(\"yes\")\nnahitar jar khota tar:\n    chhapa(\"bad\")\nnahitar:\n    chhapa(\"bad\")\n",
        ),
        (
            "zh",
            "#!zh\nruguo bu jia huo zhen na:\n    dayin(\"yes\")\nfouze ruguo jia na:\n    dayin(\"bad\")\nfouze:\n    dayin(\"bad\")\n",
        ),
    ] {
        let input = dir.file(&format!("{code}-keywords.tpy"));
        let output = dir.file(&format!("{code}-keywords.py"));
        fs::write(&input, source).expect("write input");

        let compile = run_compile(&input, &output);

        assert!(compile.status.success(), "{code}");
        assert!(compile.stderr.is_empty(), "{code}");
        let execution = Command::new("python")
            .arg(&output)
            .output()
            .expect("run generated Python");
        assert!(
            execution.status.success(),
            "{code}: {}",
            String::from_utf8_lossy(&execution.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&execution.stdout).trim(),
            "yes",
            "{code}"
        );
    }
}

#[test]
fn compiles_and_runs_romanized_hindi_source() {
    let dir = TestDir::new();
    let input = dir.file("namaste.tpy");
    let output = dir.file("namaste.py");
    fs::write(
        &input,
        "#!HI\nnaam() karya:\n    \"Ram\" lautao\n\"Namaste \" + naam() likho\nnums = [1, 2, 3]\nnums mein har n:\n    n likho\nagar sach to:\n    lambai(\"ok\") likho\nwarna agar nahi sach to:\n    \"galat\" likho\nwarna:\n    \"warna\" likho\nmath se sqrt lao\nsqrt(4) likho\n",
    )
    .expect("write input");

    let result = run_compile(&input, &output);

    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    let generated = fs::read_to_string(&output).expect("read output");
    assert!(generated.contains("def naam():"), "{generated}");
    assert!(generated.contains("if True:"), "{generated}");
    assert!(generated.contains("for n in nums:"), "{generated}");
    assert!(generated.contains("from math import sqrt"), "{generated}");
    let execution = Command::new("python")
        .arg(&output)
        .output()
        .expect("run generated Python");
    assert!(execution.status.success());
    assert_eq!(
        String::from_utf8_lossy(&execution.stdout)
            .lines()
            .collect::<Vec<_>>(),
        ["Namaste Ram", "1", "2", "3", "2", "2.0"]
    );
}

#[test]
fn romanized_hindi_reorders_class_and_exception_phrases() {
    let dir = TestDir::new();
    let input = dir.file("hindi-phrases.tpy");
    let output = dir.file("hindi-phrases.py");
    fs::write(
        &input,
        "#!hi\nVyakti kaksha:\n    chhodo\nkoshish:\n    uthao ValueError(\"bad\")\nValueError pakdo:\n    \"pakda\" likho\n",
    )
    .expect("write input");

    let result = run_compile(&input, &output);

    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    let generated = fs::read_to_string(&output).expect("read output");
    assert!(generated.contains("class Vyakti:"), "{generated}");
    assert!(generated.contains("except ValueError:"), "{generated}");
    let execution = Command::new("python")
        .arg(&output)
        .output()
        .expect("run generated Python");
    assert!(execution.status.success());
    assert_eq!(
        String::from_utf8_lossy(&execution.stdout)
            .lines()
            .collect::<Vec<_>>(),
        ["pakda"]
    );
}

#[test]
fn localized_import_shorthands_reorder_words() {
    let dir = TestDir::new();
    for (code, import_line, expected) in [
        ("es", "importar sqrt de math", "from math import sqrt"),
        ("hi", "math se sqrt lao", "from math import sqrt"),
        ("mr", "math pasun sqrt aayat", "from math import sqrt"),
        ("zh", "cong math daoru sqrt", "from math import sqrt"),
    ] {
        let input = dir.file(&format!("{code}.tpy"));
        let output = dir.file(&format!("{code}.py"));
        fs::write(&input, format!("#!{code}\n{import_line}\n")).expect("write input");

        let result = run_compile(&input, &output);

        assert!(result.status.success());
        assert!(result.stderr.is_empty());
        let generated = fs::read_to_string(output).expect("read output");
        assert!(generated.contains(expected), "{generated}");
    }
}

#[test]
fn translated_builtin_names_are_reserved_as_alias_targets() {
    let dir = TestDir::new();
    for (code, declaration, builtin) in [
        ("es", "funcion saludar{imprimir}():", "imprimir"),
        ("mr", "namaskar{chhapa}() karya:", "chhapa"),
        ("zh", "dingyi zhaohu{dayin}():", "dayin"),
    ] {
        let input = dir.file(&format!("reserved-alias-{code}.tpy"));
        let output = dir.file(&format!("reserved-alias-{code}.py"));
        fs::write(&input, format!("#!{code}\n{declaration}\n")).expect("write input");

        let result = run_compile(&input, &output);

        assert!(result.status.success(), "{code}");
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert!(stderr.contains("alias"), "{code}: {stderr}");
        assert!(stderr.contains(builtin), "{code}: {stderr}");
    }
}

#[test]
fn missing_input_reports_file_error_without_directive_advice() {
    let dir = TestDir::new();
    let input = dir.file("missing.fr.tpy");
    let output = dir.file("output.py");

    let result = run_compile(&input, &output);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(2));
    assert!(stderr.contains("File not found:"), "{stderr}");
    assert!(!stderr.contains("language directive"), "{stderr}");
    assert!(!stderr.contains("Usage:"), "{stderr}");
    assert!(!output.exists());
}

#[test]
fn source_without_directive_reports_missing_directive() {
    let dir = TestDir::new();
    let input = dir.file("no-directive.tpy");
    let output = dir.file("output.py");
    let source = "imprimez(\"Bonjour\")\n";
    fs::write(&input, source).expect("write input");

    let result = run_compile(&input, &output);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(2));
    assert!(stderr.contains("Missing language directive"), "{stderr}");
    assert!(!output.exists());
}

#[test]
fn language_directive_after_code_reports_missing_leading_directive() {
    let dir = TestDir::new();
    let input = dir.file("late-directive.tpy");
    let output = dir.file("output.py");
    let source = "print('Python source')\n#!fr\nimprimez('Not translated')\n";
    fs::write(&input, source).expect("write input");

    let result = run_compile(&input, &output);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(2));
    assert!(
        stderr.contains("Missing language directive before source code"),
        "{stderr}"
    );
    assert!(!output.exists());
}

#[test]
fn unsupported_language_directive_reports_supported_codes() {
    let dir = TestDir::new();
    let input = dir.file("unsupported.tpy");
    let output = dir.file("output.py");
    let source = "#!zz\nprint('Hola')\n";
    fs::write(&input, source).expect("write input");

    let result = run_compile(&input, &output);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(2));
    assert!(
        stderr.contains("Unsupported language code 'zz'"),
        "{stderr}"
    );
    assert!(
        stderr.contains("Supported language codes: fr, es, hi, mr, zh"),
        "{stderr}"
    );
    assert!(!output.exists());
}

#[test]
fn malformed_language_directive_reports_missing_language_code() {
    let dir = TestDir::new();
    let input = dir.file("malformed-directive.tpy");
    let output = dir.file("output.py");
    let source = "#!\nprint('Python sees this unchanged')\n";
    fs::write(&input, source).expect("write input");

    let result = run_compile(&input, &output);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(2));
    assert!(
        stderr.contains("Language directive '#!' is missing a language code"),
        "{stderr}"
    );
    assert!(!output.exists());
}

#[test]
fn empty_source_reports_missing_directive() {
    let dir = TestDir::new();
    let input = dir.file("empty.tpy");
    let output = dir.file("output.py");
    fs::write(&input, "").expect("write input");

    let result = run_compile(&input, &output);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(2));
    assert!(stderr.contains("Missing language directive"), "{stderr}");
    assert!(!output.exists());
}

#[test]
fn invalid_python_syntax_is_still_written_for_python_to_report() {
    let dir = TestDir::new();
    let input = dir.file("invalid-python.tpy");
    let output = dir.file("invalid-python.py");
    fs::write(&input, "#!fr\nimprimez(\n").expect("write input");

    let result = run_compile(&input, &output);

    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    assert!(
        fs::read_to_string(output)
            .expect("read output")
            .contains("print(")
    );
}

#[test]
fn output_write_failure_names_the_output_path() {
    let dir = TestDir::new();
    let input = dir.file("source.tpy");
    let output = dir.file("output-directory");
    fs::write(&input, "#!fr\nprint('ok')\n").expect("write input");
    fs::create_dir(&output).expect("create output directory");

    let result = run_compile(&input, &output);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(1));
    assert!(stderr.contains("Failed to write"), "{stderr}");
    assert!(
        stderr.contains(output.to_string_lossy().as_ref()),
        "{stderr}"
    );
    assert!(!stderr.contains("Usage:"), "{stderr}");
}

#[test]
fn invalid_arguments_show_usage() {
    let result = Command::new(env!("CARGO_BIN_EXE_tpy"))
        .arg("c")
        .output()
        .expect("run tpy");
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(2));
    assert!(stderr.contains("Usage:"), "{stderr}");
}
