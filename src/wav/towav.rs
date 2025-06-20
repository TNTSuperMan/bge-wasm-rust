use std::io::Cursor;
use hound::{SampleFormat, WavSpec, WavWriter};
use crate::bgeimage::toimg::Bin;
use super::tokenize::Music;

pub fn token2wav(token: Music) -> Result<Bin, String>{
    let mut samples: Vec<i16> = Vec::new();

    // Samples generate code here

    let mut buf = Cursor::new(Vec::new());
    {
        let mut writer = WavWriter::new(&mut buf, WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        }).unwrap();
        for sample in samples {
            writer.write_sample(sample).unwrap();
        }
        writer.finalize().unwrap();
    }

    Ok(Bin::new(buf.into_inner()))
}

pub fn tokens2wavs(tokens: Vec<Music>) -> Result<Vec<Bin>, String>{
    let mut waves: Vec<Bin> = Vec::new();
    for token in tokens {
        waves.push(token2wav(token)?);
    }
    Ok(waves)
}