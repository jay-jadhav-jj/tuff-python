#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    French,
    Spanish,
    Hindi,
    Marathi,
    Chinese,
}

impl Language {
    pub fn from_code(code: &str) -> Result<Self, String> {
        match code.to_ascii_lowercase().as_str() {
            "fr" => Ok(Self::French),
            "es" => Ok(Self::Spanish),
            "hi" => Ok(Self::Hindi),
            "mr" => Ok(Self::Marathi),
            "zh" => Ok(Self::Chinese),
            "" => Err("Language directive '#!' is missing a language code".to_string()),
            _ => Err(format!(
                "Unsupported language code '{}'. Supported language codes: fr, es, hi, mr, zh",
                code
            )),
        }
    }

    pub fn translation_code(self) -> &'static str {
        match self {
            Self::French => "fr",
            Self::Spanish => "es",
            Self::Hindi => "hi",
            Self::Marathi => "mr",
            Self::Chinese => "zh",
        }
    }

    pub fn import_word(self) -> &'static str {
        match self {
            Self::French => "importez",
            Self::Spanish => "importar",
            Self::Hindi => "lao",
            Self::Marathi => "aayat",
            Self::Chinese => "daoru",
        }
    }

    pub fn import_source_separator(self) -> &'static str {
        match self {
            Self::French | Self::Spanish => "de",
            Self::Hindi => "se",
            Self::Marathi => "pasun",
            Self::Chinese => "cong",
        }
    }

    pub fn then_word(self) -> &'static str {
        match self {
            Self::French => "alors",
            Self::Spanish => "entonces",
            Self::Hindi => "to",
            Self::Marathi => "tar",
            Self::Chinese => "na",
        }
    }

    pub fn elif_words(self) -> (&'static str, &'static str) {
        match self {
            Self::French => ("ou", "si"),
            Self::Spanish => ("o", "si"),
            Self::Hindi => ("warna", "agar"),
            Self::Marathi => ("nahitar", "jar"),
            Self::Chinese => ("fouze", "ruguo"),
        }
    }
}

pub fn is_keyword_name(word: &str) -> bool {
    crate::es_processor::is_keyword_name(word)
        || crate::hi_processor::is_keyword_name(word)
        || crate::mr_processor::is_keyword_name(word)
        || crate::zh_processor::is_keyword_name(word)
}
