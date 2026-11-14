use crate::{
    language_names::Language,
    lexer::{Tok, Token},
    processor::Processor,
};

const KEYWORDS: &[(&str, &str)] = &[
    ("False", "jhooth"),
    ("None", "shunya"),
    ("True", "sach"),
    ("and", "aur"),
    ("as", "jaisa"),
    ("assert", "pakka"),
    ("async", "async"),
    ("await", "ruk"),
    ("break", "todo"),
    ("class", "kaksha"),
    ("continue", "jari"),
    ("def", "karya"),
    ("del", "hatao"),
    ("elif", "warna_agar"),
    ("else", "warna"),
    ("except", "pakdo"),
    ("finally", "akhir"),
    ("for", "har"),
    ("from", "se"),
    ("global", "global"),
    ("if", "agar"),
    ("import", "lao"),
    ("in", "mein"),
    ("is", "hai"),
    ("lambda", "lambda"),
    ("nonlocal", "nonlocal"),
    ("not", "nahi"),
    ("or", "ya"),
    ("pass", "chhodo"),
    ("raise", "uthao"),
    ("return", "lautao"),
    ("try", "koshish"),
    ("while", "jabtak"),
    ("with", "saath"),
    ("yield", "do"),
    ("match", "milao"),
    ("case", "mamla"),
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
    let last = match trimmed_end.checked_sub(1) {
        Some(index) => index,
        None => return false,
    };
    let terminal = processor.terminal_word(start, end);

    if processor.is_word(last, "lao")
        && let Some(separator) = (first..last).find(|&index| processor.is_word(index, "se"))
    {
        output.push_str("from ");
        let (module_start, module_end) = processor.trim_range(first, separator);
        processor.process_range(module_start, module_end, output);
        output.push_str(" import ");
        let (import_start, import_end) = processor.trim_range(separator + 1, last);
        processor.process_range(import_start, import_end, output);
        return true;
    }

    if matches!(processor.source.get(last), Some(Token::Symbol(':'))) {
        let keyword = processor.skip_whitespace_backward(last, first);
        if processor.is_word(keyword, "karya") || processor.is_word(keyword, "kaksha") {
            let translated = if processor.is_word(keyword, "karya") {
                "def "
            } else {
                "class "
            };
            processor.process_range(start, first, output);
            output.push_str(translated);
            let (declaration_start, declaration_end) = processor.trim_range(first, keyword);
            processor.process_range(declaration_start, declaration_end, output);
            output.push(':');
            return true;
        }

        if let Some(mein) = (first..last).find(|&index| processor.is_word(index, "mein")) {
            let item = processor.skip_whitespace(mein + 1, last);
            if processor.is_word(item, "har") {
                let item_start = processor.skip_whitespace(item + 1, last);
                let item_end = processor.trim_range(item_start, last).1;
                if item_start < item_end {
                    processor.process_range(start, first, output);
                    output.push_str("for ");
                    processor.process_range(item_start, item_end, output);
                    output.push_str(" in ");
                    let (sequence_start, sequence_end) = processor.trim_range(first, mein);
                    processor.process_range(sequence_start, sequence_end, output);
                    output.push(':');
                    return true;
                }
            }
        }

        let exception = processor.skip_whitespace_backward(last, first);
        if processor.is_word(exception, "pakdo") {
            let (exception_start, exception_end) = processor.trim_range(first, exception);
            processor.process_range(start, first, output);
            output.push_str("except ");
            processor.process_range(exception_start, exception_end, output);
            output.push(':');
            return true;
        }
    }

    if let Some(verb) = terminal {
        let (expression_start, expression_end) = processor.trim_range(first, verb);
        if expression_start < expression_end {
            let output_verb = if processor.is_word(verb, "likho") {
                Some("print(")
            } else if processor.is_word(verb, "lautao") {
                Some("return ")
            } else {
                None
            };
            if let Some(output_verb) = output_verb {
                processor.process_range(start, first, output);
                output.push_str(output_verb);
                processor.process_range(expression_start, expression_end, output);
                if output_verb == "print(" {
                    output.push(')');
                }
                return true;
            }
        }
    }

    false
}

pub fn new(tokens: Vec<Tok>) -> Processor {
    Processor::new(tokens, Language::Hindi)
}
