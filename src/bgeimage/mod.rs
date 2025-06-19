extern crate image;

mod tokenize;
use tokenize::tokenize;

pub mod toimg;
use toimg::tokens2imgs;

pub fn bin2img(data: &[u8]) -> Result<Vec<toimg::Bin>, String>{
    let token = tokenize(data);
    return Ok(tokens2imgs(token)?);
}