use crate::JapaneseVoice;
use jbonsai::{
    model::{load_htsvoice_from_bytes, VoiceSet},
    Condition, Engine,
};
use jpreprocess::{
    kind::JPreprocessDictionaryKind, JPreprocess, JPreprocessConfig, SystemDictionaryConfig,
};
use std::path::PathBuf;

pub fn build_speech(
    text: &str,
    voice_model: Option<JapaneseVoice>,
    dictionary_path: Option<&str>,
    req_condition: Option<Condition>,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let config = JPreprocessConfig {
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

    let angry = include_bytes!("../htsvoice/tohoku-f01/tohoku-f01-angry.htsvoice");
    let sad = include_bytes!("../htsvoice/tohoku-f01/tohoku-f01-sad.htsvoice");
    let happy = include_bytes!("../htsvoice/tohoku-f01/tohoku-f01-happy.htsvoice");
    let neutral = include_bytes!("../htsvoice/tohoku-f01/tohoku-f01-neutral.htsvoice");
    // let deep = include_bytes!("../htsvoice/hts_voice_nitech_jp_atr503_m001-1.05/nitech_jp_atr503_m001.htsvoice");

    let voice_byte: &[u8] = match voice_model {
        Some(JapaneseVoice::angry) => angry,
        Some(JapaneseVoice::happy) => happy,
        Some(JapaneseVoice::sad) => sad,
        // Some(JapaneseVoice::deep) => deep,

        // Some(JapaneseVoice::default)
        _ => neutral,
    };

    let voices = match load_htsvoice_from_bytes(voice_byte) {
        Ok(wave) => wave,
        Err(e) => return Err(format!("Failed to load voice {voice_model:?} {e:?}").into()),
    };

    let voiceset = match VoiceSet::new(vec![std::sync::Arc::new(voices)]) {
        Ok(wave) => wave,
        Err(e) => return Err(format!("VoiceSet assumptions not met {e:?}").into()),
    };

    let mut condition = Condition::default();
    if let Some(con) = req_condition {
        condition = con;
    }
    condition.load_model(&voiceset)?;

    let engine = Engine::new(voiceset, condition);

    let speech = match engine.synthesize(labels) {
        Ok(wave) => wave,
        Err(e) => return Err(format!("Failed to synthesize labels {e:?}").into()),
    };

    Ok(speech)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn build_speech_fn_default() {
        let query = "テスト";
        let wave = build_speech(query, None, None, None).unwrap();

        assert_eq!(wave.len(), 74880);
    }

    #[test]
    fn build_speech_fn_neutral() {
        let query = "テスト";
        let wave = build_speech(query, Some(JapaneseVoice::neutral), None, None).unwrap();

        assert_eq!(wave.len(), 74880);
    }

    // #[test]
    // fn no_unknown_model_build_speech() {
    //     let query = "テスト";
    //     let wave = build_speech(query, None, None, None).err();

    //     let err = wave.unwrap().to_string();
    //     assert!(err.starts_with("Failed to load engine"));
    // }
}
