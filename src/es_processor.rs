use crate::{language_names::Language, lexer::Tok, processor::Processor};

const KEYWORDS: &[(&str, &str)] = &[
    ("False", "falso"),
    ("None", "nada"),
    ("True", "cierto"),
    ("and", "y"),
    ("as", "como"),
    ("assert", "asegura"),
    ("async", "async"),
    ("await", "espera"),
    ("break", "rompe"),
    ("class", "clase"),
    ("continue", "sigue"),
    ("def", "funcion"),
    ("del", "borra"),
    ("elif", "sino_si"),
    ("else", "sino"),
    ("except", "excepto"),
    ("finally", "fin"),
    ("for", "para"),
    ("from", "desde"),
    ("global", "global"),
    ("if", "si"),
    ("import", "importar"),
    ("in", "en"),
    ("is", "es"),
    ("lambda", "lambda"),
    ("nonlocal", "nonlocal"),
    ("not", "no"),
    ("or", "o"),
    ("pass", "pasa"),
    ("raise", "lanza"),
    ("return", "retorna"),
    ("try", "intenta"),
    ("while", "mientras"),
    ("with", "con"),
    ("yield", "cede"),
    ("match", "coincide"),
    ("case", "caso"),
    ("type", "type"),
];

pub fn keyword_python_name(word: &str) -> Option<&'static str> {
    KEYWORDS
        .iter()
        .find(|(_, translated)| word.eq_ignore_ascii_case(translated))
        .map(|(python, _)| *python)
}

pub fn is_keyword_name(word: &str) -> bool {
    KEYWORDS
        .iter()
        .any(|(_, translated)| word.eq_ignore_ascii_case(translated))
}

pub fn new(tokens: Vec<Tok>) -> Processor {
    Processor::new(tokens, Language::Spanish)
}
