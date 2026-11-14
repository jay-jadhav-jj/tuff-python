use crate::{
    language_names::Language,
    lexer::{Tok, Token},
    processor::Processor,
};

const KEYWORDS: &[(&str, &str)] = &[
    ("False", "jia"),
    ("None", "wu"),
    ("True", "zhen"),
    ("and", "he"),
    ("as", "zuowei"),
    ("assert", "queren"),
    ("async", "async"),
    ("await", "dengdai"),
    ("break", "tupo"),
    ("class", "lei"),
    ("continue", "jixu"),
    ("def", "dingyi"),
    ("del", "shanchu"),
    ("elif", "fouze_ruguo"),
    ("else", "fouze"),
    ("except", "chuwai"),
    ("finally", "zuihou"),
    ("for", "mei_ge"),
    ("from", "cong"),
    ("global", "global"),
    ("if", "ruguo"),
    ("import", "daoru"),
    ("in", "zai"),
    ("is", "shi"),
    ("lambda", "lambda"),
    ("nonlocal", "nonlocal"),
    ("not", "bu"),
    ("or", "huo"),
    ("pass", "tiaoguo"),
    ("raise", "paichu"),
    ("return", "fanhui"),
    ("try", "changshi"),
    ("while", "dang"),
    ("with", "yong"),
    ("yield", "shuchu"),
    ("match", "pipei"),
    ("case", "qingkuang"),
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

pub fn process_word_order(
    processor: &mut Processor,
    start: usize,
    end: usize,
    output: &mut String,
) -> bool {
    let first = processor.skip_whitespace(start, end);
    let trimmed_end = processor.trim_range(start, end).1;
    let Some(last) = trimmed_end.checked_sub(1) else {
        return false;
    };

    if processor.is_word(first, "cong")
        && let Some(import_word) =
            (first + 1..trimmed_end).find(|&index| processor.is_word(index, "daoru"))
    {
        output.push_str("from ");
        let (module_start, module_end) = processor.trim_range(first + 1, import_word);
        processor.process_range(module_start, module_end, output);
        output.push_str(" import ");
        let (names_start, names_end) = processor.trim_range(import_word + 1, trimmed_end);
        processor.process_range(names_start, names_end, output);
        return true;
    }

    if matches!(processor.source.get(last), Some(Token::Symbol(':')))
        && processor.is_word(first, "mei_ge")
    {
        let item_start = processor.skip_whitespace(first + 1, last);
        if let Some(zai) = (item_start..last).find(|&index| processor.is_word(index, "zai")) {
            let (item_start, item_end) = processor.trim_range(item_start, zai);
            let (sequence_start, sequence_end) = processor.trim_range(zai + 1, last);
            if item_start < item_end && sequence_start < sequence_end {
                processor.process_range(start, first, output);
                output.push_str("for ");
                processor.process_range(item_start, item_end, output);
                output.push_str(" in ");
                processor.process_range(sequence_start, sequence_end, output);
                output.push(':');
                return true;
            }
        }
    }

    false
}

pub fn new(tokens: Vec<Tok>) -> Processor {
    Processor::new(tokens, Language::Chinese)
}
