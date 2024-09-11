mod utils;
mod voice;
mod voice_bundled;

use cfg_if::cfg_if;
use utils::set_panic_hook;
use voice::{buff_wav, build_speech, MySound};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(js_name = "buildSpeech")]
pub fn build_speech_fn(query: &str) -> Vec<u8> {
    let wave = match build_speech(query, None, None) {
        Ok(x) => x,
        Err(e) => {
            // TODO: is there a try catch alternative
            let err = format!("Failed to build speech {:?}", e);
            log(err.as_str());
            return vec![];
        }
    };

    let speech = MySound::new(wave);

    buff_wav(&speech.wave)
}

#[wasm_bindgen(start)]
fn run() {
    // log(&String::from("Hello from 'run()'!"));
    set_panic_hook();
}
