use wavegen::{wf, sine};
use crate::bgeimage::toimg::Bin;
use super::tokenize::Music;

pub fn token2wav(token: Music) -> Result<Bin, String>{
    sine
}

pub fn tokens2wavs(tokens: Vec<Music>) -> Result<Vec<Bin>, String>{
    let mut waves: Vec<Bin> = Vec::new();
    for token in tokens {
        waves.push(token2wav(token)?);
    }
    return Ok(waves);
}