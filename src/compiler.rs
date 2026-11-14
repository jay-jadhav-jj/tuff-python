use crate::{filehandling, fr_processor, language_names::Language, lexer};

impl Language {
    pub fn from_source(source: &str) -> Result<Self, String> {
        for line in source.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() || (trimmed.starts_with('#') && !trimmed.starts_with("#!")) {
                continue;
            }

            if let Some(directive) = trimmed.strip_prefix("#!") {
                let code = directive.split_whitespace().next().unwrap_or("");
                return Self::from_code(code);
            }

            return Err(
                "Missing language directive before source code; expected a leading '#!fr', '#!es', '#!hi', '#!mr', or '#!zh' directive"
                    .to_string(),
            );
        }

        Err(
            "Missing language directive; expected a leading '#!fr', '#!es', '#!hi', '#!mr', or '#!zh' directive"
                .to_string(),
        )
    }
}

pub struct CompileOutput {
    pub code: String,
    pub alias_errors: Vec<crate::alias::AliasError>,
}

fn compile_source(source: &str, language: Language) -> CompileOutput {
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.lex().clone();
    let mut processor = match language {
        Language::French => fr_processor::new(tokens),
        Language::Spanish => crate::es_processor::new(tokens),
        Language::Hindi => crate::hi_processor::new(tokens),
        Language::Marathi => crate::mr_processor::new(tokens),
        Language::Chinese => crate::zh_processor::new(tokens),
    };
    let code = processor.process();

    CompileOutput {
        code,
        alias_errors: processor.errors().to_vec(),
    }
}

pub fn compile_source_with_diagnostics(source: &str) -> Result<CompileOutput, String> {
    let language = Language::from_source(source)?;
    Ok(compile_source(source, language))
}

pub fn compile_file_with_diagnostics(path: &str) -> Result<CompileOutput, String> {
    let source = filehandling::read_file(path)?;
    compile_source_with_diagnostics(&source)
}
