mod lib_utils;
mod speech;
mod voice_sound;

use jbonsai::Condition;
use lib_utils::set_panic_hook;
use speech::build_speech;
use voice_sound::{trim::trim_wave, VoiceWave};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(non_snake_case)]
#[wasm_bindgen(js_name = "buildSpeech")]
pub fn build_speech_fn(query: &str, voice_model: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut condition = Condition::default();
    condition.set_speed(0.85);

    let wave = match build_speech(query, None, voice_model, Some(condition)) {
        Ok(x) => x,
        Err(e) => {
            let err = format!("@nmemonica/voice-ja {:?}", e);
            return Err(err.into());
        }
    };

    let wave = trim_wave(wave, 500.0, Some(4000));

    let speech = VoiceWave::new(wave);

    Ok(speech.to_wav_buffer())
}

#[wasm_bindgen(start)]
fn run() {
    // log(&String::from("Hello from 'run()'!"));
    set_panic_hook();
}
