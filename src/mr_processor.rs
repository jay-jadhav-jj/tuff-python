use crate::{
    language_names::Language,
    lexer::{Tok, Token},
    processor::Processor,
};

const KEYWORDS: &[(&str, &str)] = &[
    ("False", "khota"),
    ("None", "shunya"),
    ("True", "khara"),
    ("and", "ani"),
    ("as", "mhanun"),
    ("assert", "siddha"),
    ("async", "async"),
    ("await", "thamba"),
    ("break", "baher"),
    ("class", "varga"),
    ("continue", "pudhe"),
    ("def", "karya"),
    ("del", "kadha"),
    ("elif", "nahitar_jar"),
    ("else", "nahitar"),
    ("except", "pakda"),
    ("finally", "shevati"),
    ("for", "pratyek"),
    ("from", "pasun"),
    ("global", "global"),
    ("if", "jar"),
    ("import", "aayat"),
    ("in", "madhye"),
    ("is", "aahe"),
    ("lambda", "lambda"),
    ("nonlocal", "nonlocal"),
    ("not", "nahi"),
    ("or", "kiwa"),
    ("pass", "sodun_dya"),
    ("raise", "uchla"),
    ("return", "parat"),
    ("try", "prayatna"),
    ("while", "joparyant"),
    ("with", "sobat"),
    ("yield", "dya"),
    ("match", "julva"),
    ("case", "vakalp"),
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

    if processor.is_word(last, "aayat")
        && let Some(separator) = (first..last).find(|&index| processor.is_word(index, "pasun"))
    {
        output.push_str("from ");
        let (module_start, module_end) = processor.trim_range(first, separator);
        processor.process_range(module_start, module_end, output);
        output.push_str(" import ");
        let (names_start, names_end) = processor.trim_range(separator + 1, last);
        processor.process_range(names_start, names_end, output);
        return true;
    }

    if processor.is_word(first, "pasun")
        && let Some(import_word) =
            (first + 1..trimmed_end).find(|&index| processor.is_word(index, "aayat"))
    {
        output.push_str("from ");
        let (module_start, module_end) = processor.trim_range(first + 1, import_word);
        processor.process_range(module_start, module_end, output);
        output.push_str(" import ");
        let (names_start, names_end) = processor.trim_range(import_word + 1, trimmed_end);
        processor.process_range(names_start, names_end, output);
        return true;
    }

    if matches!(processor.source.get(last), Some(Token::Symbol(':'))) {
        let keyword = processor.skip_whitespace_backward(last, first);
        if processor.is_word(keyword, "karya") || processor.is_word(keyword, "varga") {
            processor.process_range(start, first, output);
            output.push_str(if processor.is_word(keyword, "karya") {
                "def "
            } else {
                "class "
            });
            let (declaration_start, declaration_end) = processor.trim_range(first, keyword);
            processor.process_range(declaration_start, declaration_end, output);
            output.push(':');
            return true;
        }

        if let Some(madhye) = (first..last).find(|&index| processor.is_word(index, "madhye")) {
            let item = processor.skip_whitespace(madhye + 1, last);
            if processor.is_word(item, "pratyek") {
                let item_start = processor.skip_whitespace(item + 1, last);
                let item_end = processor.trim_range(item_start, last).1;
                if item_start < item_end {
                    processor.process_range(start, first, output);
                    output.push_str("for ");
                    processor.process_range(item_start, item_end, output);
                    output.push_str(" in ");
                    let (sequence_start, sequence_end) = processor.trim_range(first, madhye);
                    processor.process_range(sequence_start, sequence_end, output);
                    output.push(':');
                    return true;
                }
            }
        }

        let exception = processor.skip_whitespace_backward(last, first);
        if processor.is_word(exception, "pakda") {
            processor.process_range(start, first, output);
            output.push_str("except ");
            let (exception_start, exception_end) = processor.trim_range(first, exception);
            processor.process_range(exception_start, exception_end, output);
            output.push(':');
            return true;
        }
    }

    if let Some(verb) = processor.terminal_word(start, end) {
        let (expression_start, expression_end) = processor.trim_range(first, verb);
        if expression_start < expression_end {
            let output_verb = if processor.is_word(verb, "chhapa") {
                Some("print(")
            } else if processor.is_word(verb, "parat") {
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
    Processor::new(tokens, Language::Marathi)
}
