use crate::{
    alias::{AliasError, AliasRegistry},
    language_names::Language,
    lexer::*,
};

pub struct Processor {
    pub(crate) source: Vec<Tok>,
    idx: usize,
    language: Language,
    aliases: AliasRegistry,
    errors: Vec<AliasError>,
}

impl Processor {
    pub fn new(lex_res: Vec<Tok>, language: Language) -> Processor {
        Processor {
            source: lex_res,
            idx: 0,
            language,
            aliases: AliasRegistry::new(),
            errors: Vec::new(),
        }
    }

    pub fn errors(&self) -> &[AliasError] {
        &self.errors
    }

    fn translate_keyword(language: Language, word: &str) -> Option<&'static str> {
        if let Some(python_name) = crate::builtin_translations::python_name(language, word) {
            return Some(python_name);
        }

        let python_name = match language {
            Language::French => crate::fr_processor::keyword_python_name(word),
            Language::Spanish => crate::es_processor::keyword_python_name(word),
            Language::Hindi => crate::hi_processor::keyword_python_name(word),
            Language::Marathi => crate::mr_processor::keyword_python_name(word),
            Language::Chinese => crate::zh_processor::keyword_python_name(word),
        };
        if let Some(python_name) = python_name {
            return Some(python_name);
        }
        None
    }

    pub(crate) fn is_whitespace(tok: &Tok) -> bool {
        matches!(tok, Tok::Whitespace(_) | Tok::Tab)
    }

    pub(crate) fn skip_whitespace(&self, mut idx: usize, end: usize) -> usize {
        while idx < end && Self::is_whitespace(&self.source[idx]) {
            idx += 1;
        }

        idx
    }

    pub(crate) fn trim_range(&self, mut start: usize, mut end: usize) -> (usize, usize) {
        while start < end && Self::is_whitespace(&self.source[start]) {
            start += 1;
        }

        while end > start && Self::is_whitespace(&self.source[end - 1]) {
            end -= 1;
        }

        (start, end)
    }

    /// Detects:
    ///
    /// french_name{original_name}
    ///
    /// Whitespace inside the braces is permitted.
    ///
    /// Returns the alias, original name and position after
    /// the closing brace.
    fn find_alias(&self, start: usize, end: usize) -> Option<(String, String, usize)> {
        if start >= end {
            return None;
        }

        let alias = match &self.source[start] {
            Tok::WordObj(word) => word.clone(),
            _ => return None,
        };

        let open = self.skip_whitespace(start + 1, end);

        if !matches!(self.source.get(open), Some(Tok::Symbol('{'))) {
            return None;
        }

        let original_idx = self.skip_whitespace(open + 1, end);

        let original = match self.source.get(original_idx) {
            Some(Tok::WordObj(word)) => word.clone(),
            _ => return None,
        };

        let close = self.skip_whitespace(original_idx + 1, end);

        if !matches!(self.source.get(close), Some(Tok::Symbol('}'))) {
            return None;
        }

        Some((alias, original, close + 1))
    }

    fn translate_identifier(&self, word: &str) -> String {
        // Aliases have priority over keyword translations.
        if let Some(original) = self.aliases.resolve(word) {
            return original.to_string();
        }

        if let Some(translated) = Self::translate_keyword(self.language, word) {
            return translated.to_string();
        }

        word.to_string()
    }

    pub(crate) fn is_word(&self, index: usize, expected: &str) -> bool {
        matches!(
            self.source.get(index),
            Some(Tok::WordObj(word)) if word.eq_ignore_ascii_case(expected)
        )
    }

    fn process_line(&mut self, start: usize, end: usize, output: &mut String) {
        let reordered = match self.language {
            Language::Hindi => crate::hi_processor::process_word_order(self, start, end, output),
            Language::Marathi => crate::mr_processor::process_word_order(self, start, end, output),
            Language::Chinese => crate::zh_processor::process_word_order(self, start, end, output),
            Language::French | Language::Spanish => false,
        };
        if reordered {
            return;
        }

        let first = self.skip_whitespace(start, end);
        let (elif_first, elif_second) = self.language.elif_words();
        let starts_elif_phrase = self.is_word(first, elif_first)
            && self.is_word(self.skip_whitespace(first + 1, end), elif_second);

        if starts_elif_phrase {
            self.process_range(start, first, output);
            output.push_str("elif");
        }

        let body_start = if starts_elif_phrase {
            self.skip_whitespace(first + 1, end) + 1
        } else {
            start
        };
        let first_word = if starts_elif_phrase {
            self.language.elif_words().1
        } else {
            match self.source.get(first) {
                Some(Tok::WordObj(word)) => word.as_str(),
                _ => "",
            }
        };
        let is_conditional = Self::translate_keyword(self.language, first_word)
            .is_some_and(|translated| translated == "if" || translated == "elif");

        if is_conditional {
            let mut depth = 0isize;
            let alors = (body_start..end).find(|&index| {
                match self.source.get(index) {
                    Some(Tok::Symbol('(' | '[' | '{')) => depth += 1,
                    Some(Tok::Symbol(')' | ']' | '}')) => depth -= 1,
                    Some(Tok::WordObj(word))
                        if depth == 0 && word.eq_ignore_ascii_case(self.language.then_word()) =>
                    {
                        return true;
                    }
                    _ => {}
                }

                false
            });

            if let Some(alors) = alors {
                let trailing = self.skip_whitespace(alors + 1, end);
                let has_colon = matches!(self.source.get(trailing), Some(Tok::Symbol(':')))
                    && self.skip_whitespace(trailing + 1, end) == end;
                let marker_is_terminal = has_colon || self.skip_whitespace(alors + 1, end) == end;

                if marker_is_terminal {
                    let mut condition_end = alors;
                    while condition_end > body_start
                        && Self::is_whitespace(&self.source[condition_end - 1])
                    {
                        condition_end -= 1;
                    }
                    self.process_range(body_start, condition_end, output);
                    output.push(':');
                    return;
                }
            }
        }

        self.process_range(body_start, end, output);
    }

    pub(crate) fn terminal_word(&self, start: usize, end: usize) -> Option<usize> {
        let last = self.trim_range(start, end).1.checked_sub(1)?;
        match self.source.get(last) {
            Some(Tok::WordObj(_)) => Some(last),
            _ => None,
        }
    }

    pub(crate) fn skip_whitespace_backward(&self, mut index: usize, start: usize) -> usize {
        while index > start && Self::is_whitespace(&self.source[index - 1]) {
            index -= 1;
        }
        index.saturating_sub(1)
    }

    /// Processes a token range, registering aliases as encountered.
    pub(crate) fn process_range(&mut self, start: usize, end: usize, output: &mut String) {
        let mut i = start;

        while i < end {
            // Check for an alias declaration.
            if let Some((alias, original, next)) = self.find_alias(i, end) {
                match self.aliases.register(&alias, &original) {
                    Ok(()) => {
                        let resolved = self
                            .aliases
                            .resolve(&alias)
                            .unwrap_or(&original)
                            .to_string();

                        output.push_str(&resolved);
                    }

                    Err(error) => {
                        self.errors.push(error);

                        // Emit the alias as an ordinary identifier.
                        output.push_str(
                            Self::translate_keyword(self.language, &alias).unwrap_or(&alias),
                        );
                    }
                }

                i = next;
                continue;
            }

            match &self.source[i] {
                Tok::WordObj(word) => {
                    let translated = self.translate_identifier(word);
                    output.push_str(&translated);
                }

                Tok::StringLit(value) => {
                    // Strings are copied exactly as lexed.
                    output.push_str(value);
                }

                Tok::NumLit(value) => {
                    output.push_str(value);
                }

                Tok::Symbol(symbol) => {
                    output.push(*symbol);
                }

                Tok::NewLine => {
                    output.push('\n');
                }

                Tok::Tab => {
                    output.push('\t');
                }

                Tok::Whitespace(spaces) => {
                    output.push_str(spaces);
                }
            }

            i += 1;
        }
    }

    /// Handles:
    ///
    /// importez function{original} de module{original_module}
    ///
    /// Produces:
    ///
    /// from original_module import original
    fn process_import(&mut self, output: &mut String) {
        let start = self.idx + 1;
        let mut end = start;

        // Find the end of the import statement.
        while end < self.source.len() && !matches!(&self.source[end], Tok::NewLine) {
            end += 1;
        }

        // Locate the language-specific import separator.
        let separator = (start..end).find(|&i| {
            matches!(
                &self.source[i],
                Tok::WordObj(word)
                    if word.eq_ignore_ascii_case(self.language.import_source_separator())
            )
        });

        if let Some(separator) = separator {
            let mut imported = String::new();
            let mut module = String::new();

            let (import_start, import_end) = self.trim_range(start, separator);

            let (module_start, module_end) = self.trim_range(separator + 1, end);

            // Register imported aliases first.
            self.process_range(import_start, import_end, &mut imported);

            // Then register module aliases.
            self.process_range(module_start, module_end, &mut module);

            output.push_str("from ");
            output.push_str(&module);
            output.push_str(" import ");
            output.push_str(&imported);
        } else {
            // Ordinary import.
            let (start, end) = self.trim_range(start, end);

            output.push_str("import ");

            self.process_range(start, end, output);
        }

        // Leave the newline for the main processing loop.
        self.idx = end;
    }

    pub fn process(&mut self) -> String {
        let mut output = String::new();

        while self.idx < self.source.len() {
            if matches!(self.source[self.idx], Tok::NewLine) {
                output.push('\n');
                self.idx += 1;
                continue;
            }

            let start = self.idx;
            let mut end = start;
            while end < self.source.len() && !matches!(self.source[end], Tok::NewLine) {
                end += 1;
            }
            let first = self.skip_whitespace(start, end);

            if self.is_word(first, self.language.import_word()) {
                self.process_range(start, first, &mut output);
                self.idx = first;
                self.process_import(&mut output);
            } else {
                self.process_line(start, end, &mut output);
                self.idx = end;
            }

            if self.idx < self.source.len() && matches!(self.source[self.idx], Tok::NewLine) {
                output.push('\n');
                self.idx += 1;
            }
        }

        output
    }
}
