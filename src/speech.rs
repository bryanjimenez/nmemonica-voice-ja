use std::{path::PathBuf, sync::Arc};

use jbonsai::{
    model::{load_htsvoice_bytes, VoiceSet},
    Condition,
};
use jpreprocess::{
    kind::JPreprocessDictionaryKind, JPreprocess, JPreprocessConfig, SystemDictionaryConfig,
};

pub fn build_speech(
    text: &str,
    dictionary_path: Option<&str>,
    voice_model: &[u8],
    condition: Option<Condition>,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let config = JPreprocessConfig {
        // dictionary: jpreprocess::SystemDictionaryConfig::File(PathBuf::from(dictionary)),
        dictionary: match dictionary_path {
            Some(dictionary_path) => {
                jpreprocess::SystemDictionaryConfig::File(PathBuf::from(dictionary_path))
            }
            None => SystemDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic),
        },
        user_dictionary: None,
    };

    let jpreprocess = JPreprocess::from_config(config)?;

    let fc = jpreprocess.extract_fullcontext(text)?;

    let labels: Vec<String> = fc.into_iter().map(|x| x.to_string()).collect();

    let engine = {
        let loaded_voice = match load_htsvoice_bytes(voice_model) {
            Ok(v) => vec![Arc::new(v)],
            Err(e) => {
                let err = format!("Failed to load voice into engine {:?}", e);
                return Err(err.into());
            }
        };

        let voiceset = VoiceSet::new(loaded_voice)?;
        let mut condition = condition.unwrap_or_default();
        condition.load_model(&voiceset)?;

        jbonsai::Engine::new(voiceset, condition)
    };

    let speech = match engine.synthesize(labels) {
        Ok(wave) => wave,
        _ => return Err("Failed to synthesize labels".into()),
    };

    Ok(speech)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const DEFAULT_VOICE: &str =
        "./models/hts_voice_nitech_jp_atr503_m001-1.05/nitech_jp_atr503_m001.htsvoice";

    #[test]
    fn build_speech_fn() {
        let voice_model = fs::read(DEFAULT_VOICE).unwrap();
        let voice_model = voice_model.as_slice();
        let query = "テスト";
        let wave = build_speech(query, None, voice_model, None).unwrap();

        assert_eq!(wave.len(), 62640);
    }

    #[test]
    fn no_unknown_model_build_speech() {
        let voice_model = &[];
        let query = "テスト";
        let wave = build_speech(query, None, voice_model, None).err();

        let err = wave.unwrap().to_string();
        assert!(err.starts_with("Failed to load voice into engine"));
    }
}
