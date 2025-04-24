use serde::Deserialize;

use super::{Duration, Language, Offset};

pub type SpeechStartDetected = crate::recognizer::message::SpeechStartDetected;
pub type SpeechEndDetected = crate::recognizer::message::SpeechEndDetected;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AudioStart {
    #[allow(unused)]
    pub translation_language: Language,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AudioEnd {
    #[allow(unused)]
    pub status: SynthesisStatus,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TranslationHypothesis {
    pub text: String,
    pub offset: Offset,
    pub duration: Duration,
    #[allow(unused)]
    pub translation: Translation,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TranslationPhrase {
    pub text: Option<String>,
    pub offset: Offset,
    pub duration: Duration,
    pub recognition_status: RecognitionStatus,
    #[allow(unused)]
    pub translation: Translation,
}

pub type RecognitionStatus = crate::recognizer::message::common::RecognitionStatus;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Translation {
    #[allow(unused)]
    pub translation_status: TranslationStatus,
    #[allow(unused)]
    pub translations: Vec<TranslationItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TranslationItem {
    #[allow(unused)]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub(crate) enum TranslationStatus {
    Success,
    NoMatch,
    InitialSilenceTimeout,
    BabbleTimeout,
    Error,
    EndOfDictation,
    TooManyRequests,
    BadRequest,
    Forbidden,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub(crate) enum SynthesisStatus {
    Success,
    Error,
}
