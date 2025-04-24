use std::{cmp::min, io::Cursor};

use tokio_stream::{Stream, StreamExt as _};
use tracing::{debug, error, warn};
use url::Url;

use super::{
    messages,
    utils::{
        create_audio_header_message, create_speech_config_message, create_speech_context_message,
    },
};
use crate::{
    connector::Client as BaseClient,
    recognizer::message::common::RecognitionStatus,
    stream_ext::StreamExt,
    translator::{
        session::Session, utils::create_audio_message, AudioDevice, AudioFormat, Config, Event,
        OutputFormat,
    },
    utils::get_azure_hostname_from_region,
    Auth, Data, Message,
};

const BUFFER_SIZE: usize = 4096;

#[derive(Clone)]
pub struct Client {
    pub client: BaseClient,
    pub config: Config,
}

impl Client {
    pub fn new(client: BaseClient, config: Config) -> Self {
        Self { client, config }
    }

    pub async fn connect(auth: Auth, config: Config) -> crate::Result<Self> {
        let mut url = match &auth {
            Auth::Subscription { region, .. } => {
                let base_url = format!(
                    "wss://{region}.s2s.speech{}/speech/translation/cognitiveservices/v1",
                    get_azure_hostname_from_region(region)
                );
                Url::parse(&base_url)?
            }
            Auth::Host { host, .. } => host.clone(),
        };

        let from_language = &config.recognition_language;
        url.query_pairs_mut()
            .append_pair("language", from_language)
            .append_pair("from", from_language)
            .append_pair("to", &config.target_languages.join(","))
            .append_pair("format", config.output_format.as_str())
            .append_pair("profanity", config.profanity.as_str())
            .append_pair("storeAudio", &config.store_audio.to_string())
            .append_pair("scenario", config.mode.as_str());

        if let Some(timeout) = config.initial_silence_timeout {
            url.query_pairs_mut()
                .append_pair("initialSilenceTimeoutMs", &timeout.as_millis().to_string());
        }

        if config.synthesize {
            url.query_pairs_mut()
                .append_pair("features", "texttospeech");
        }

        if let Some(voice) = &config.synthesize_voice {
            url.query_pairs_mut().append_pair("voice", voice);
        }

        if config.output_format == OutputFormat::Detailed {
            url.query_pairs_mut()
                .append_pair("wordLevelTimestamps", "true");
        }

        // if config.languages.len() > 1 {
        //     url.query_pairs_mut().append_pair("lidEnabled", "true");
        // }

        if let Some(ref connection_id) = config.connection_id {
            url.query_pairs_mut()
                .append_pair("X-ConnectionId", connection_id);
        }

        debug!("url: {url}");

        let ws_client = tokio_websockets::ClientBuilder::new()
            .uri(url.as_str())
            .unwrap()
            .add_header(
                "Ocp-Apim-Subscription-Key".try_into().unwrap(),
                auth.key().to_string().as_str().try_into().unwrap(),
            )?
            .add_header(
                "X-ConnectionId".try_into().unwrap(),
                uuid::Uuid::new_v4().to_string().try_into().unwrap(),
            )?;

        let client = BaseClient::connect(ws_client).await?;
        Ok(Self::new(client, config))
    }

    pub async fn disconnect(&self) -> crate::Result<()> {
        self.client.disconnect().await
    }

    pub async fn translate<A>(
        &self,
        mut audio: A,
        audio_format: AudioFormat,
        audio_device: AudioDevice,
    ) -> crate::Result<impl Stream<Item = crate::Result<Event>>>
    where
        A: Stream<Item = Vec<u8>> + Sync + Send + Unpin + 'static,
    {
        let messages = self.client.stream().await?;
        let session = Session::new();
        let config = self.config.clone();
        let client = self.client.clone();
        let (restart_tx, mut restart_rx) = tokio::sync::mpsc::channel(1);

        // Send the initial speech configuration.
        client
            .send(create_speech_config_message(
                session.request_id().to_string(),
                &config,
                &audio_device,
            ))
            .await?;

        // let synthesizer_config = synthesizer::Config {
        //     audio_format: config.synthesize_format.clone(),
        //     ..Default::default()
        // };

        // client
        //     .send(create_synthesis_context_message(
        //         session.request_id().to_string(),
        //         &synthesizer_config,
        //     ))
        //     .await?;

        // Send the initial context and audio header messages.
        client
            .send(create_speech_context_message(
                session.request_id().to_string(),
                &config,
            ))
            .await?;

        // For WAV audio, extract the header and extra data.
        let (audio_header, extra) = match audio_format {
            AudioFormat::Wav => {
                let (header, extra) = extract_header_from_wav(&mut audio).await?;
                (Some(header), extra)
            }
            _ => (None, vec![]),
        };

        // Create the audio data buffer and seed it with any extra bytes.
        let mut buffer = Vec::with_capacity(BUFFER_SIZE);
        buffer.extend(extra);

        client
            .send(create_audio_header_message(
                session.request_id().to_string(),
                audio_format.clone(),
                audio_header.as_deref(),
            ))
            .await?;

        let _session = session.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Handle any restart signal.
                    _ = restart_rx.recv() => {
                        tracing::info!("Refreshing audio header");
                        _session.refresh();

                        if client.send(create_audio_header_message(
                            _session.request_id().to_string(),
                            audio_format.clone(),
                            audio_header.as_deref(),
                        )).await.is_err() {
                            warn!("Failed to refresh audio header");
                            break;
                        }
                    },
                    // Process the next chunk from the audio stream.
                    maybe_chunk = audio.next() => {
                        match maybe_chunk {
                            Some(chunk) => {
                                // Append the new data to the buffer.
                                buffer.extend(chunk);
                                // While there is enough data, send it in fixed-size chunks.
                                while buffer.len() >= BUFFER_SIZE {
                                    let data: Vec<u8> = buffer.drain(..BUFFER_SIZE).collect();
                                    if client.send(create_audio_message(_session.request_id().to_string(), Some(&data))).await.is_err() {
                                        warn!("Failed to send audio message");
                                        break;
                                    }
                                }
                            }
                            None => {
                                // No more audio: flush remaining bytes in the buffer.
                                while !buffer.is_empty() {
                                    let data: Vec<u8> = buffer.drain(..min(buffer.len(), BUFFER_SIZE)).collect();
                                    if client.send(create_audio_message(_session.request_id().to_string(), Some(&data))).await.is_err() {
                                        warn!("Failed to send final audio chunk");
                                        break;
                                    }
                                }
                                // Signal the end of audio.
                                let _ = client.send(create_audio_message(_session.request_id().to_string(), None)).await;
                                _session.set_audio_completed(true);
                                break;
                            }
                        }
                    }
                }
            }
        });

        // Build the output stream that filters and converts messages into events.
        let session_clone = session.clone();
        let output_stream = messages
            .filter(move |msg| match msg {
                Ok(m) => m.id == session.request_id().to_string(),
                Err(_) => true,
            })
            .filter_map(move |msg| {
                let session_ref = session_clone.clone();
                match msg {
                    Ok(m) => convert_message_to_event(m, &session_ref),
                    Err(e) => Some(Err(e)),
                }
            })
            .map(move |event| {
                if let Ok(Event::SessionEnded(_)) = event {
                    let _ = restart_tx.try_send(());
                }
                event
            })
            .stop_after(|event| event.is_err());

        Ok(output_stream)
    }
}

fn convert_message_to_event(message: Message, session: &Session) -> Option<crate::Result<Event>> {
    match (message.path.as_str(), message.data, message.headers) {
        ("turn.start", _, _) => Some(Ok(Event::SessionStarted(session.request_id()))),
        ("turn.end", _, _) => Some(Ok(Event::SessionEnded(session.request_id()))),
        ("speech.startdetected", Data::Text(Some(data)), _) => {
            serde_json::from_str::<messages::SpeechStartDetected>(&data)
                .map(|v| Event::StartDetected(session.request_id(), v.offset))
                .map(Ok)
                .ok()
        }
        ("speech.enddetected", Data::Text(Some(data)), _) => {
            let value =
                serde_json::from_str::<messages::SpeechEndDetected>(&data).unwrap_or_default();
            Some(Ok(Event::EndDetected(session.request_id(), value.offset)))
        }
        ("translation.hypothesis", Data::Text(Some(data)), _) => {
            match serde_json::from_str::<messages::TranslationHypothesis>(&data) {
                Ok(value) => {
                    let offset = value.offset + session.audio_offset();
                    session.on_hypothesis_received(offset);
                    Some(Ok(Event::Translating(
                        session.request_id(),
                        value.text,
                        offset,
                        value.duration,
                        data,
                    )))
                }
                Err(e) => Some(Err(crate::Error::ParseError(e.to_string()))),
            }
        }
        ("translation.phrase", Data::Text(Some(data)), _) => {
            match serde_json::from_str::<messages::TranslationPhrase>(&data) {
                Ok(phrase) => match phrase.recognition_status {
                    RecognitionStatus::Success => Some(Ok(Event::Translated(
                        session.request_id(),
                        phrase.text.unwrap_or_default(),
                        phrase.offset,
                        phrase.duration,
                        data,
                    ))),
                    RecognitionStatus::NoMatch => Some(Ok(Event::NoMatch(
                        session.request_id(),
                        phrase.offset,
                        phrase.duration,
                        data,
                    ))),
                    RecognitionStatus::EndOfDictation => None,
                    status => {
                        let status_to_error: Option<crate::Error> = (&status).into();
                        if let Some(err) = status_to_error {
                            error!("Translation status: {status:?}");
                            return Some(Err(err));
                        };
                        warn!("Unprocessed translation.phrase status: {status:?}");
                        None
                    }
                },
                Err(e) => Some(Err(crate::Error::ParseError(e.to_string()))),
            }
        }
        ("audio.start", Data::Text(Some(data)), _) => {
            match serde_json::from_str::<messages::AudioStart>(&data) {
                Ok(value) => {
                    // TODO:
                    debug!("Unprocessed: {:?}", value);
                    None
                }
                Err(e) => Some(Err(crate::Error::ParseError(e.to_string()))),
            }
        }
        ("audio.end", Data::Text(Some(data)), _) => {
            match serde_json::from_str::<messages::AudioEnd>(&data) {
                Ok(value) => {
                    // TODO:
                    debug!("Unprocessed: {:?}", value);
                    None
                }
                Err(e) => Some(Err(crate::Error::ParseError(e.to_string()))),
            }
        }
        ("translation.synthesis", Data::Binary(audio), _) => {
            let Some(audio) = audio else {
                error!("No audio returned");
                return None;
            };

            let Ok(reader) = hound::WavReader::new(Cursor::new(audio)) else {
                error!("Invalid WAV header");
                return None;
            };
            let spec = reader.spec();
            debug!("Wav Spec: {spec:?}");
            let samples: Result<Vec<i16>, _> = reader.into_samples().collect();
            let Ok(samples) = samples else {
                error!("Failed to convert audio to i16 samples");
                return None;
            };

            Some(Ok(Event::TranslationSynthesis(
                session.request_id(),
                samples,
            )))
        }
        (msg, data, _) => {
            warn!("Unexpected: `{msg}`:`{data:?}`");
            None
        }
    }
}

async fn extract_header_from_wav(
    reader: &mut (impl Stream<Item = Vec<u8>> + Unpin + Send + Sync + 'static),
) -> Result<(Vec<u8>, Vec<u8>), crate::Error> {
    let mut header = Vec::new();

    // Loop until the stream is exhausted.
    while let Some(chunk) = reader.next().await {
        header.extend(chunk);

        // We need at least 12 bytes to check the RIFF and WAVE identifiers.
        if header.len() < 12 {
            continue;
        }

        // Check for a valid WAV header: bytes 0..4 must be "RIFF" and 8..12 must be "WAVE".
        if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
            return Err(crate::Error::ParseError("Invalid wav header".to_string()));
        }

        // Look for the "data" descriptor.
        if let Some(pos) = header.windows(4).position(|w| w == b"data") {
            // Ensure we have read the 4 bytes following "data" (i.e. the length field).
            if header.len() < pos + 8 {
                // Not enough bytes yet; continue reading.
                continue;
            }
            // Split the header at the end of the "data" chunk descriptor and its length field.
            let header_end = pos + 8;
            let remainder = header.split_off(header_end);
            return Ok((header, remainder));
        }
    }

    Err(crate::Error::ParseError(
        "Reached end of stream without finding 'data' chunk".to_string(),
    ))
}
