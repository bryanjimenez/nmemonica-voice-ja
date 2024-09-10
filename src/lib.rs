mod utils;
mod voice;

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

#[wasm_bindgen(js_name = "exampleBuffer")]
pub fn example_buffer() -> Vec<u8> {
    // let wave: &[u8; 4] = b"\x80\xBB\x00\x00"; //      Sample rate (in hertz) (=48000) (x0000bb80) (x80bb0000)
    let mut wave: Vec<i16> = vec![];

    for _i in 0..1000 {
        wave.push(2300)
    }

    // let wave: Vec<i16> = wave
    // .into_iter()
    // .map(|p| {
    //     let clamped = p.min(i16::MAX as f64).max(i16::MIN as f64);

    //     clamped as i16
    // })
    // .collect();

    buff_wav(&wave)
}

#[wasm_bindgen(js_name = "testVoice")]
pub fn test_voice(query: &str) -> Vec<u8> {
    // let mut wave:Vec<i16> =vec![];
    let voice = "";
    let wave = match build_speech(query, None, voice) {
        Ok(x) => x,
        Err(_e) => {
            log("Failed to build speech {:?}");
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
