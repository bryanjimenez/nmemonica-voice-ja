pub mod trim;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use std::time::Duration;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use rodio::Source;
use trim::trim_wave;

pub struct VoiceWave {
    pub wave: Vec<i16>,
    // Count of samples
    len: usize,

    index: usize,
}

impl VoiceWave {
    #[allow(dead_code)]
    pub fn builder() -> VoiceWaveBuilder {
        VoiceWaveBuilder::default()
    }

    /// Count of the samples
    pub fn sample_length(&self) -> usize {
        self.len
    }

    /// How many `bytes` in a sample element
    pub fn bytes_per_sample(&self) -> usize {
        std::mem::size_of::<i16>()
    }

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    fn channels(&self) -> u16 {
        1
    }

    /// Frequency
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    fn sample_rate(&self) -> u32 {
        48000
    }

    /// Converts signal into a **wav** file buffer
    pub fn to_wav_buffer(&self) -> Vec<u8> {
        let header_size = 44;
        // each sample is i16, file writes are in u8. i16 = [u8,u8]
        let data_size = u32::try_from(self.sample_length() * self.bytes_per_sample())
            .expect("Data size exceeds max for .wav");
        let file_size = header_size + data_size - 8;

        let sample_el_size_byte = u16::try_from(self.bytes_per_sample())
            .expect("Expected sample element byte size to be small");
        let bit_per_sample = 8 * sample_el_size_byte;

        let byte_per_block = self.channels() * sample_el_size_byte;
        let byte_per_sec = self.sample_rate() * u32::from(byte_per_block);

        let file_type_bloc_id: &[u8; 4] = b"RIFF"; //                   Identifier « RIFF »  (0x52, 0x49, 0x46, 0x46)
        let file_size: &[u8; 4] = &file_size.to_le_bytes(); //          Overall file size minus 8 bytes
        let file_format_id: &[u8; 4] = b"WAVE"; //                      Format = « WAVE »  (0x57, 0x41, 0x56, 0x45)
        let format_bloc_id: &[u8; 4] = b"fmt "; //                      Identifier « fmt␣ »  (0x66, 0x6D, 0x74, 0x20)
        let bloc_size: &[u8; 4] = &16_u32.to_le_bytes(); //             Chunk size minus 8 bytes, which is 16 bytes here  (0x10) b"\x10\x00\x00\x00"
        let audio_format: &[u8; 2] = &1_u16.to_le_bytes(); //           Audio format (1: PCM integer, 3: IEEE 754 float)
        let nbr_channels: &[u8; 2] = &self.channels().to_le_bytes(); // Number of channels
        let frequency: &[u8; 4] = &self.sample_rate().to_le_bytes(); // Sample rate (in hertz) (=48000) (x0000bb80) (x80bb0000) b"\x80\xBB\x00\x00"
        let byte_per_sec: &[u8; 4] = &byte_per_sec.to_le_bytes(); //    Number of bytes to read per second (Frequency * BytePerBloc).(96000) (x00017700) (x00770100) b"\x00\x77\x01\x00"
        let byte_per_block: &[u8; 2] = &byte_per_block.to_le_bytes(); //Number of bytes per block (NbrChannels * BitsPerSample / 8). ((1*16)/8)=2 b"\x02\x00"
        let bits_per_sample: &[u8; 2] = &bit_per_sample.to_le_bytes(); //Number of bits per sample (=16) b"\x10\x00"
        let data_bloc_id: &[u8; 4] = b"data"; //                        Identifier « data »  (0x64, 0x61, 0x74, 0x61)
        let data_size: &[u8; 4] = &data_size.to_le_bytes(); //          SampledData size
        let sampled_data = &self.wave;

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
        let header_block: Vec<u8> = header_block.into_iter().flatten().copied().collect();

        let sampled_data: Vec<u8> = sampled_data.iter().flat_map(|x| x.to_le_bytes()).collect();

        // wav_buffer.append(&mut sampled_data);
        // wav_buffer
        [header_block, sampled_data].concat()
    }
}

impl Iterator for VoiceWave {
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

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl Source for VoiceWave {
    // https://docs-rs-web-prod.infra.rust-lang.org/rodio/0.6.0/rodio/source/trait.Source.html

    fn channels(&self) -> u16 {
        1
    }
    /// Frequency
    fn sample_rate(&self) -> u32 {
        48000
    }

    fn current_frame_len(&self) -> Option<usize> {
        Some(self.len - (self.index + 1))
    }

    fn total_duration(&self) -> Option<Duration> {
        todo!()
    }
}

#[derive(Default)]
pub struct VoiceWaveBuilder {
    wave: Vec<f64>,

    len: usize,
}

impl VoiceWaveBuilder {
    pub fn new(wave: Vec<f64>) -> VoiceWaveBuilder {
        let len = wave.len();
        VoiceWaveBuilder { wave, len }
    }

    #[allow(dead_code)]
    pub fn wave(mut self, wave: Vec<f64>) -> VoiceWaveBuilder {
        self.wave = wave;
        self
    }

    pub fn trim(mut self, cutoff: f64, end_padding: Option<usize>) -> VoiceWaveBuilder {
        let wave = trim_wave(self.wave, cutoff, end_padding);

        self.wave = wave;
        self
    }

    pub fn build(self) -> VoiceWave {
        let wave: Vec<i16> = self
            .wave
            .into_iter()
            .map(|p| {
                let clamped = p.min(i16::MAX as f64).max(i16::MIN as f64);

                clamped as i16
            })
            .collect();

        VoiceWave {
            wave,
            len: self.len,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_wav_buffer() {
        let wav = VoiceWave::builder().build().to_wav_buffer();
        assert_eq!(wav.len(), 44);

        let wav = VoiceWaveBuilder::default().build().to_wav_buffer();
        assert_eq!(wav.len(), 44);

        let wav = VoiceWaveBuilder::default()
            .wave([].to_vec())
            .build()
            .to_wav_buffer();
        assert_eq!(wav.len(), 44);
    }
}
