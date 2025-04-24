use super::Language;
use crate::RequestId;

/// The raw text of message.
///
/// The raw message is the message received from the speech recognition service.
pub type RawMessage = String;

/// Recognizer events.
///
/// The events are used to notify the user of the progress of the speech recognition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The session started.
    SessionStarted(RequestId),

    /// The session ended.
    SessionEnded(RequestId),

    /// The speech recognition started.
    StartDetected(RequestId, Offset),
    /// The speech recognition ended.
    EndDetected(RequestId, Offset),

    TranslationSynthesis(RequestId, Vec<i16>),

    Translating(RequestId, String, Offset, Duration, RawMessage),
    Translated(RequestId, String, Offset, Duration, RawMessage),

    /// UnMatch event.
    /// This event is triggered when the speech recognition does not match any text.
    NoMatch(RequestId, Offset, Duration, RawMessage),
    //Cancelled(RequestId, Offset, crate::Error),
}

/// The offset of the speech recognition.
///
/// The offset is the time in milliseconds from the start of the conversation.
pub type Offset = u64;

/// The duration of the speech recognition.
///
/// The duration is the time in milliseconds of the speech recognition.
pub type Duration = u64;

/// The confidence of the speech recognition.
///
/// The confidence is the confidence of the speech recognition.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Confidence {
    Low,
    Normal,
    High,
    #[default]
    Unknown,
}

impl From<&str> for Confidence {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Confidence {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "low" => Confidence::Low,
            "normal" => Confidence::Normal,
            "high" => Confidence::High,
            _ => Confidence::Unknown,
        }
    }
}

/// Primary language
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaryLanguage {
    /// The language code
    pub language: Language,
    /// The confidence of the language detection
    pub confidence: Confidence,
}

impl PrimaryLanguage {
    #[allow(unused)]
    pub(crate) fn new(language: Language, confidence: Confidence) -> Self {
        Self {
            language,
            confidence,
        }
    }
}
