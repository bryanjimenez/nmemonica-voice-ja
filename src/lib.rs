mod utils;
mod voice;

use jbonsai::Condition;
use utils::set_panic_hook;
use voice::{buff_wav, build_speech, MySound};
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

    let speech = MySound::new(wave);

    Ok(buff_wav(&speech.wave))
}

#[wasm_bindgen(start)]
fn run() {
    // log(&String::from("Hello from 'run()'!"));
    set_panic_hook();
}
