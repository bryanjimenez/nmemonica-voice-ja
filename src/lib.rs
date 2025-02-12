mod lib_utils;
mod speech;
mod voice_sound;

use jbonsai::Condition;
use lib_utils::set_panic_hook;
use speech::build_speech;
use voice_sound::VoiceWaveBuilder;
use wasm_bindgen::prelude::*;

const PACKAGE_NAME: &str = "@nmemonica/voice-ja";

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
pub struct QueryResult {
    pub uid: String,
    pub index: Option<u32>,
    pub buffer: Vec<u8>,
}

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub enum JapaneseVoice {
    neutral = "neutral",
    angry = "angry",
    happy = "happy",
    sad = "sad",
    // deep = "deep",
}

#[wasm_bindgen(js_name = "buildSpeech")]
pub fn build_speech_fn(
    key: &str,
    index: Option<u32>,
    query: &str,
    voice_model: Option<JapaneseVoice>,
) -> Result<QueryResult, JsValue> {
    let mut condition = Condition::default();
    condition.set_speed(0.85);

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

    let wave = match build_speech(query, None, voice_byte, Some(condition)) {
        Ok(x) => x,
        Err(e) => {
            let err = format!("{PACKAGE_NAME} {:?}", e);
            return Err(err.into());
        }
    };

    let speech = VoiceWaveBuilder::new(wave).trim(500.0, Some(4000)).build();

    let result = speech.to_wav_buffer();

    Ok(QueryResult {
        uid: key.to_string(),
        index,
        buffer: result,
    })
}

#[wasm_bindgen(start)]
fn run() {
    set_panic_hook();
}
