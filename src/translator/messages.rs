use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TranslationPhrase {
    #[serde(rename = "Text")]
    pub(crate) text: String,
    #[serde(rename = "Translation")]
    pub(crate) translation: Translation,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Translation {
    #[serde(rename = "TranslationStatus")]
    pub(crate) translation_status: TranslationStatus,
    #[serde(rename = "Translations")]
    pub(crate) translations: Vec<TranslationItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TranslationItem {
    #[serde(rename = "Text")]
    pub(crate) text: String,
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
