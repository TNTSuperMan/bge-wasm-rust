extern crate image;

mod tokenize;
use tokenize::tokenize;

pub mod toimg;
use toimg::tokens2imgs;

pub fn bin2img(data: &[u8]) -> Vec<toimg::Bin>{
    let token = tokenize(data);
    return tokens2imgs(token);
}