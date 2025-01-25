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

#[wasm_bindgen(js_name = "buildSpeech")]
pub fn build_speech_fn(
    key: &str,
    index: Option<u32>,
    query: &str,
    voice_model: &[u8],
) -> Result<QueryResult, JsValue> {
    let mut condition = Condition::default();
    condition.set_speed(0.85);

    let wave = match build_speech(query, None, voice_model, Some(condition)) {
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
    // log(&String::from("Hello from 'run()'!"));
    set_panic_hook();
}
