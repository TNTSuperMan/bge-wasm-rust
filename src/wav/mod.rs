use crate::{bgeimage::toimg, wav::towav::tokens2wavs};

mod tokenize;
use tokenize::tokenize;

mod towav;

pub fn bin2wav(data: &[u8]) -> Result<Vec<toimg::Bin>, String>{
    let tokens = tokenize(data);
    let bins = tokens2wavs(tokens)?;
    Ok(bins)
}
