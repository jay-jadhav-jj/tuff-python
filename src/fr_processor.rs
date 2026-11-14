use crate::{language_names::Language, lexer::Tok, processor::Processor};

pub fn keyword_python_name(word: &str) -> Option<&'static str> {
    match word.to_ascii_lowercase().as_str() {
        "si" => Some("if"),
        "sinon" => Some("else"),
        "sinon_si" | "ou_si" => Some("elif"),
        "pour" => Some("for"),
        "en" => Some("in"),
        "tantque" => Some("while"),
        "classe" => Some("class"),
        "fonction" => Some("def"),
        "retournez" => Some("return"),
        "interrompez" => Some("break"),
        "continuez" => Some("continue"),
        "imprimez" => Some("print"),
        "gamme" => Some("range"),
        "longueur" => Some("len"),
        "et" => Some("and"),
        "ou" => Some("or"),
        "non" => Some("not"),
        "vrai" => Some("True"),
        "faux" => Some("False"),
        "aucun" => Some("None"),
        "selon" => Some("match"),
        "cas" => Some("case"),
        "importez" => Some("import"),
        "depuis" => Some("from"),
        "comme" => Some("as"),
        "affirme" => Some("assert"),
        "asynchrone" | "async" => Some("async"),
        "attendez" => Some("await"),
        "effacez" => Some("del"),
        "finalement" => Some("finally"),
        "sauf" => Some("except"),
        "avec" => Some("with"),
        "essayez" => Some("try"),
        "levez" => Some("raise"),
        "passe" => Some("pass"),
        "est" => Some("is"),
        "global" => Some("global"),
        "nonlocal" => Some("nonlocal"),
        "lambda" => Some("lambda"),
        "type" => Some("type"),
        "rend" => Some("yield"),
        _ => None,
    }
}

pub fn new(tokens: Vec<Tok>) -> Processor {
    Processor::new(tokens, Language::French)
}
