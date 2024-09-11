use std::{path::PathBuf, sync::Arc, time::Duration};

use jbonsai::{model::VoiceSet, Condition};
use jpreprocess::{
    kind::JPreprocessDictionaryKind, JPreprocess, JPreprocessConfig, SystemDictionaryConfig,
};
use rodio::Source;

use crate::voice_bundled::use_bundled_voice;

pub struct MySound {
    pub wave: Vec<i16>,
    pub len: usize,

    index: usize,
}

impl MySound {
    pub fn new(wave: Vec<f64>) -> MySound {
        let len = wave.len();

        let wave: Vec<i16> = wave
            .into_iter()
            .map(|p| {
                let clamped = p.min(i16::MAX as f64).max(i16::MIN as f64);

                clamped as i16
            })
            .collect();

        MySound {
            wave,
            len,

            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Iterator for MySound {
    type Item = i16;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.index < self.len {
            let item = self.wave[self.index];

            self.index += 1;
            return Some(item);
        }

        None
    }
}

impl Source for MySound {
    // https://docs-rs-web-prod.infra.rust-lang.org/rodio/0.6.0/rodio/source/trait.Source.html

    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        48000
    }

    fn current_frame_len(&self) -> Option<usize> {
        Some(self.len - self.index)
    }

    fn total_duration(&self) -> Option<Duration> {
        todo!()
    }
}

pub fn build_speech(
    text: &str,
    dictionary_path: Option<&str>,
    voice_path: Option<&str>,
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

    let engine = match voice_path {
        Some(voice_path) => match jbonsai::Engine::load(&[
            // The path to the `.htsvoice` model file.
            // Currently only Japanese models are supported (due to the limitation of jlabel).
            voice_path,
        ]) {
            Ok(e) => e,
            _ => return Err("Failed to load provided voice into engine".into()),
        },
        None => {
            let bundled_voice = match use_bundled_voice() {
                Ok(v) => vec![Arc::new(v)],
                Err(e) => {
                    let err = format!("Failed to load bundled voice into engine {:?}", e);
                    return Err(err.into());
                }
            };

            let voiceset = VoiceSet::new(bundled_voice)?;
            let mut condition = Condition::default();
            condition.load_model(&voiceset)?;

            jbonsai::Engine::new(voiceset, condition)
        }
    };

    let speech = match engine.synthesize(labels) {
        Ok(wave) => wave,
        _ => return Err("Failed to synthesize labels".into()),
    };

    Ok(speech)
}

pub fn buff_wav(wave: &[i16]) -> Vec<u8> {
    let header_size = 44;
    // each sample is i16, file writes are in u8. i16 = [u8,u8]
    let data_size = wave.len() * 2;
    let file_size = header_size + data_size - 8;

    let fs = file_size.to_le_bytes();
    let ds = data_size.to_le_bytes();

    let file_type_bloc_id: &[u8; 4] = b"RIFF"; //               Identifier « RIFF »  (0x52, 0x49, 0x46, 0x46)
    let file_size: &[u8; 4] = &[fs[0], fs[1], fs[2], fs[3]]; // Overall file size minus 8 bytes
    let file_format_id: &[u8; 4] = b"WAVE"; //                  Format = « WAVE »  (0x57, 0x41, 0x56, 0x45)
    let format_bloc_id: &[u8; 4] = b"fmt "; //                  Identifier « fmt␣ »  (0x66, 0x6D, 0x74, 0x20)
    let bloc_size: &[u8; 4] = b"\x10\x00\x00\x00"; //           Chunk size minus 8 bytes, which is 16 bytes here  (0x10)
    let audio_format: &[u8; 2] = b"\x01\x00"; //                Audio format (1: PCM integer, 3: IEEE 754 float)
    let nbr_channels: &[u8; 2] = b"\x01\x00"; //                Number of channels
    let frequency: &[u8; 4] = b"\x80\xBB\x00\x00"; //           Sample rate (in hertz) (=48000) (x0000bb80) (x80bb0000)
    let byte_per_sec: &[u8; 4] = b"\x00\x77\x01\x00"; //        Number of bytes to read per second (Frequency * BytePerBloc).(9600!) (6000) (x00001770) (x70170000)
    let byte_per_block: &[u8; 2] = b"\x02\x00"; //              Number of bytes per block (NbrChannels * BitsPerSample / 8). (1/8)
    let bits_per_sample: &[u8; 2] = b"\x10\x00"; //             Number of bits per sample (=16)
    let data_bloc_id: &[u8; 4] = b"data"; //                    Identifier « data »  (0x64, 0x61, 0x74, 0x61)
    let data_size: &[u8; 4] = &[ds[0], ds[1], ds[2], ds[3]]; // SampledData size
    let sampled_data = wave;

    let header_block = [
        file_type_bloc_id,
        file_size,
        file_format_id,
        format_bloc_id,
        bloc_size,
        &[
            audio_format[0],
            audio_format[1],
            nbr_channels[0],
            nbr_channels[1],
        ],
        frequency,
        byte_per_sec,
        &[
            byte_per_block[0],
            byte_per_block[1],
            bits_per_sample[0],
            bits_per_sample[1],
        ],
        data_bloc_id,
        data_size,
    ];

    // let mut header_block: Vec<u8> = header_block.into_iter().flatten().map(|h| *h).collect();
    // let mut wav_buffer: Vec<u8> = header_block.into_iter().flat_map(|f| **f).collect();
    let mut wav_buffer: Vec<u8> = header_block.into_iter().flatten().copied().collect();

    let mut sampled_data: Vec<u8> = sampled_data.iter().flat_map(|x| x.to_le_bytes()).collect();

    wav_buffer.append(&mut sampled_data);
    wav_buffer
}
