use serde::{Deserialize, Serialize};

use super::{Language, Voice};
use crate::{config::Device, synthesizer::AudioFormat};

/// The configuration for the recognizer.
///
/// The configuration is used to set the parameters of the speech recognition.
#[derive(Clone, Debug)]
pub struct Config {
    pub recognition_language: Language,
    pub target_languages: Vec<Language>,
    pub output_format: OutputFormat,
    /// Synthesize the output?
    pub synthesize: bool,

    // Not supported (yet), it's always 16khz WAV.
    pub synthesize_format: AudioFormat,
    pub synthesize_voice: Option<Voice>,

    // todo: probably this will be removed and moved directly in the connection.
    pub mode: RecognitionMode, // todo: what is this?

    // pub(crate) language_detect_mode: Option<LanguageDetectMode>,
    pub phrases: Option<Vec<String>>,
    pub custom_models: Option<Vec<(String, String)>>,
    pub connection_id: Option<String>, // todo: what is this for?
    pub store_audio: bool,             // todo: is this needed?
    pub device: Device,
    pub profanity: Profanity,
    // todo: check diarization https://learn.microsoft.com/en-us/azure/ai-services/speech-service/get-started-stt-diarization?tabs=macos&pivots=programming-language-javascript
    // probably will be moved from here and added to a separate module.
    //pub recognize_speaker: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            recognition_language: "en-GB".into(),
            target_languages: ["en-GB".to_string()].into(),
            output_format: OutputFormat::default(),
            synthesize: false,
            synthesize_format: AudioFormat::default(),
            synthesize_voice: None,
            mode: RecognitionMode::default(),
            // language_detect_mode: None,
            phrases: None,
            custom_models: None,
            connection_id: None,
            store_audio: false,
            device: Device::default(),
            profanity: Profanity::default(),
        }
    }
}

impl Config {
    pub fn new(recognition_language: Language, target_language: Language) -> Self {
        Self {
            recognition_language,
            target_languages: vec![target_language],
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
/// The profanity level.
pub enum Profanity {
    #[allow(missing_docs)]
    #[default]
    Masked,
    #[allow(missing_docs)]
    Removed,
    #[allow(missing_docs)]
    Raw,
}

impl Profanity {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Profanity::Masked => "masked",
            Profanity::Removed => "removed",
            Profanity::Raw => "raw",
        }
    }
}

#[derive(Debug, Clone)]
/// The configuration for the silence detection.
///
/// Untested.
pub struct Silence {
    #[allow(missing_docs)]
    pub initial_timeout_ms: Option<i32>,
    #[allow(missing_docs)]
    pub end_timeout_ms: Option<i32>,
    #[allow(missing_docs)]
    pub segmentation_timeout_ms: Option<i32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
/// The recognition mode.
pub enum RecognitionMode {
    /// Use this mode for normal conversation.
    #[serde(rename = "conversation")]
    #[default]
    Conversation,
    /// Untested.
    #[serde(rename = "interactive")]
    Interactive,
    /// Untested.
    #[serde(rename = "dictation")]
    Dictation,
}

impl RecognitionMode {
    #[allow(unused)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            RecognitionMode::Conversation => "conversation",
            RecognitionMode::Interactive => "interactive",
            RecognitionMode::Dictation => "dictation",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
/// The output format of the messages.
pub enum OutputFormat {
    #[allow(missing_docs)]
    #[default]
    Simple,
    #[allow(missing_docs)]
    Detailed,
}

impl OutputFormat {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Simple => "simple",
            OutputFormat::Detailed => "detailed",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
/// The primary language of the recognized text.
pub enum LanguageDetectMode {
    /// Detect the language at the start of the audio.
    #[serde(rename = "DetectContinuous")]
    #[default]
    Continuous,
    /// Detect the language at the start of the audio.
    #[serde(rename = "DetectAtAudioStart")]
    AtStart,
}

#[derive(Debug, Clone, Default, Serialize)]
/// Details of the source.
///
/// This is used to provide information about the source.
pub struct AudioDevice {
    /// Name of the Audio Device
    pub(crate) name: String,
    /// Model of the Audio Device
    pub(crate) model: String,
    /// Manufacturer of the Audio Device
    pub(crate) manufacturer: String,
    /// Type of the Audio Device
    #[serde(rename = "type")]
    pub(crate) source: SourceType,
    /// Connectivity of the Audio Device
    pub(crate) connectivity: ConnectionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub enum ConnectionType {
    Bluetooth,
    Wired,
    WiFi,
    Cellular,
    InBuilt,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub enum SourceType {
    Phone,
    Speaker,
    Car,
    Headset,
    Thermostat,
    Microphones,
    Deskphone,
    RemoteControl,
    #[default]
    Unknown,
    File,
    Stream,
}

impl AudioDevice {
    /// Create a new Details instance
    pub fn new(source: SourceType) -> Self {
        AudioDevice {
            source,
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_manufacturer(mut self, manufacturer: impl Into<String>) -> Self {
        self.manufacturer = manufacturer.into();
        self
    }

    pub fn with_connectivity(mut self, connectivity: ConnectionType) -> Self {
        self.connectivity = connectivity;
        self
    }

    pub fn with_source(mut self, source: SourceType) -> Self {
        self.source = source;
        self
    }

    #[allow(missing_docs)]
    pub fn unknown() -> Self {
        AudioDevice::new(SourceType::Unknown)
    }

    #[allow(missing_docs)]
    pub fn stream() -> Self {
        AudioDevice::new(SourceType::Stream)
    }
    #[allow(missing_docs)]
    pub fn microphone(
        name: impl Into<String>,
        manufacture: impl Into<String>,
        connectivity: ConnectionType,
    ) -> Self {
        AudioDevice::new(SourceType::Microphones)
            .with_connectivity(connectivity)
            .with_manufacturer(manufacture)
            .with_name(name)
    }
    #[allow(missing_docs)]
    pub fn file() -> Self {
        AudioDevice::new(SourceType::File)
    }
}
