#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    WordObj(String),
    StringLit(String),
    NumLit(String),
    Symbol(char),
    NewLine,
    Tab,
    Whitespace(String),
}

pub type Tok = Token;

pub struct Lexer {
    feed: Vec<char>,
    idx: usize,
    tokens: Vec<Tok>,
}

impl Lexer {
    pub fn new(feed: &str) -> Lexer {
        Lexer {
            feed: feed.chars().collect(),
            idx: 0,
            tokens: Vec::new(),
        }
    }

    fn peek(&self, distance: usize) -> Option<char> {
        self.feed.get(self.idx + distance).copied()
    }

    fn lex_wordobj(&mut self) -> String {
        let mut buf = String::new();

        while let Some(c) = self.peek(0) {
            if c.is_alphanumeric() || c == '_' {
                buf.push(c);
                self.idx += 1;
            } else {
                break;
            }
        }

        buf
    }

    // Preserves the complete original string literal.
    // Supports single, double and triple quotes.
    fn lex_strlit(&mut self) -> String {
        let quote = self.feed[self.idx];

        let triple = self.peek(1) == Some(quote) && self.peek(2) == Some(quote);

        let delimiter_len = if triple { 3 } else { 1 };

        let delimiter: String = std::iter::repeat_n(quote, delimiter_len).collect();

        let mut buf = delimiter.clone();

        self.idx += delimiter_len;

        let mut escaped = false;

        while let Some(c) = self.peek(0) {
            // Check for closing delimiter.
            if !escaped && c == quote {
                let matches_delimiter = (0..delimiter_len).all(|i| self.peek(i) == Some(quote));

                if matches_delimiter {
                    for _ in 0..delimiter_len {
                        buf.push(quote);
                        self.idx += 1;
                    }

                    return buf;
                }
            }

            buf.push(c);
            self.idx += 1;

            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            }
        }

        // Unterminated strings are passed through.
        // Python can report the syntax error.
        buf
    }

    fn lex_numlit(&mut self) -> String {
        let mut buf = String::new();
        let mut has_dot = false;

        while let Some(c) = self.peek(0) {
            if c.is_ascii_digit() || c == '_' {
                buf.push(c);
                self.idx += 1;
            } else if c.is_ascii_alphabetic() {
                // Supports prefixes, exponents and imaginary suffixes.
                buf.push(c);
                self.idx += 1;

                if (c == 'e' || c == 'E') && matches!(self.peek(0), Some('+' | '-')) {
                    buf.push(self.peek(0).unwrap());
                    self.idx += 1;
                }
            } else if c == '.' {
                // Don't consume the first dot of a range.
                if has_dot
                    || self.peek(1) == Some('.')
                    || !self.peek(1).is_some_and(|n| n.is_ascii_digit())
                {
                    break;
                }

                has_dot = true;
                buf.push('.');
                self.idx += 1;
            } else {
                break;
            }
        }

        buf
    }

    pub fn lex(&mut self) -> &Vec<Tok> {
        while self.idx < self.feed.len() {
            let c = self.feed[self.idx];

            if c == '\n' {
                self.idx += 1;
                self.tokens.push(Tok::NewLine);
            } else if c == '\t' {
                self.idx += 1;
                self.tokens.push(Tok::Tab);
            } else if c == '\r' {
                self.idx += 1;
            } else if c == ' ' {
                let mut spaces = String::new();

                while self.peek(0) == Some(' ') {
                    spaces.push(' ');
                    self.idx += 1;
                }

                self.tokens.push(Tok::Whitespace(spaces));
            } else if c == '#' {
                // Leave the newline for the main lexer loop.
                while self.idx < self.feed.len() && self.feed[self.idx] != '\n' {
                    self.idx += 1;
                }
            } else if c.is_alphabetic() || c == '_' {
                let word = self.lex_wordobj();
                self.tokens.push(Tok::WordObj(word));
            } else if c == '"' || c == '\'' {
                let string = self.lex_strlit();
                self.tokens.push(Tok::StringLit(string));
            } else if c.is_ascii_digit() {
                let number = self.lex_numlit();
                self.tokens.push(Tok::NumLit(number));
            } else {
                self.tokens.push(Tok::Symbol(c));
                self.idx += 1;
            }
        }

        &self.tokens
    }
}
