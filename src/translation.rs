use reqwest::{StatusCode, blocking::Client};
use serde_json::Value;
use std::{error::Error, fmt, time::Duration};

const TRANSLATION_ENDPOINT: &str = "https://translate.googleapis.com/translate_a/single";

#[derive(Debug)]
pub enum TranslationError {
    InvalidLanguage { field: &'static str, value: String },
    ClientBuild(reqwest::Error),
    Request(reqwest::Error),
    ResponseRead(reqwest::Error),
    ServiceRejected { status: StatusCode, message: String },
    InvalidResponse(String),
}

impl fmt::Display for TranslationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLanguage { field, value } => {
                write!(formatter, "Invalid {field} language code '{value}'")
            }
            Self::ClientBuild(source) => {
                write!(
                    formatter,
                    "Could not initialize the translation client: {source}"
                )
            }
            Self::Request(source) if source.is_timeout() => write!(
                formatter,
                "The translation request timed out: {source}. Try again later."
            ),
            Self::Request(source) => write!(
                formatter,
                "Could not connect to Google Translate: {source}. \
                 Check your internet connection and try again."
            ),
            Self::ResponseRead(source) => {
                write!(
                    formatter,
                    "Could not read the translation response: {source}"
                )
            }
            Self::ServiceRejected { status, message }
                if *status == StatusCode::TOO_MANY_REQUESTS || *status == StatusCode::FORBIDDEN =>
            {
                write!(
                    formatter,
                    "Google Translate refused the request (HTTP {status}): {message}. \
                     It may be rate-limiting requests; try again later."
                )
            }
            Self::ServiceRejected { status, message } => write!(
                formatter,
                "Google Translate returned HTTP {status}: {message}"
            ),
            Self::InvalidResponse(message) => {
                write!(
                    formatter,
                    "Google Translate returned an invalid response: {message}"
                )
            }
        }
    }
}

impl Error for TranslationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ClientBuild(source) | Self::Request(source) | Self::ResponseRead(source) => {
                Some(source)
            }
            _ => None,
        }
    }
}

/// Translates text through Google's public translation endpoint without
/// requiring an API key or environment configuration.
///
/// Language codes must be supported by Google Translate. An empty source
/// code or `auto` requests language detection. Text is sent to Google over
/// HTTPS. This endpoint is undocumented and may change or be rate-limited.
pub fn translate(
    text: &str,
    source_language: &str,
    target_language: &str,
) -> Result<String, TranslationError> {
    translate_at(TRANSLATION_ENDPOINT, text, source_language, target_language)
}

fn validate_language(
    field: &'static str,
    language: &str,
    allow_auto: bool,
) -> Result<(), TranslationError> {
    let normalized = language.trim();
    if normalized.is_empty() && allow_auto {
        return Ok(());
    }
    if normalized.is_empty() || (!allow_auto && normalized.eq_ignore_ascii_case("auto")) {
        return Err(TranslationError::InvalidLanguage {
            field,
            value: language.to_string(),
        });
    }
    Ok(())
}

fn translate_at(
    endpoint: &str,
    text: &str,
    source_language: &str,
    target_language: &str,
) -> Result<String, TranslationError> {
    if text.is_empty() {
        return Ok(String::new());
    }

    validate_language("source", source_language, true)?;
    validate_language("target", target_language, false)?;

    let client = Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(TranslationError::ClientBuild)?;
    let source_language = if source_language.trim().is_empty()
        || source_language.trim().eq_ignore_ascii_case("auto")
    {
        "auto"
    } else {
        source_language.trim()
    };

    let response = client
        .post(endpoint)
        .query(&[
            ("client", "at"),
            ("sl", source_language),
            ("tl", target_language.trim()),
            ("dt", "t"),
            ("ie", "UTF-8"),
        ])
        .form(&[("q", text)])
        .send()
        .map_err(TranslationError::Request)?;
    let status = response.status();
    let body = response.text().map_err(TranslationError::ResponseRead)?;

    if !status.is_success() {
        let message = serde_json::from_str::<Value>(&body)
            .ok()
            .and_then(|value| {
                value
                    .get("error")
                    .and_then(Value::as_str)
                    .map(str::to_string)
            })
            .filter(|message| !message.is_empty())
            .unwrap_or_else(|| {
                let message = body.trim();
                if message.is_empty() {
                    "no response details".to_string()
                } else {
                    message.chars().take(512).collect()
                }
            });
        return Err(TranslationError::ServiceRejected { status, message });
    }

    let response: Value = serde_json::from_str(&body)
        .map_err(|error| TranslationError::InvalidResponse(format!("expected JSON ({error})")))?;
    let segments = response.get(0).and_then(Value::as_array).ok_or_else(|| {
        TranslationError::InvalidResponse("missing translated text segments".to_string())
    })?;
    let translated = segments
        .iter()
        .filter_map(|segment| segment.get(0).and_then(Value::as_str))
        .collect::<String>();

    if translated.is_empty() {
        return Err(TranslationError::InvalidResponse(
            "translation contained no text".to_string(),
        ));
    }

    Ok(translated)
}

