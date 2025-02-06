extern crate image;

use inflate::inflate_bytes;

mod tokenize;
use tokenize::tokenize;

pub mod toimg;
use toimg::tokens2imgs;

pub fn bin2img(data: &[u8]) -> Vec<toimg::Bin>{
    let extracted = inflate_bytes(data).expect("Image extract error");
    let token = tokenize(extracted.as_slice());
    return tokens2imgs(token);
}