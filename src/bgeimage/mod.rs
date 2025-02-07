extern crate image;

use inflate::inflate_bytes_zlib;

mod tokenize;
use tokenize::tokenize;

pub mod toimg;
use toimg::tokens2imgs;

pub fn bin2img(data: &[u8]) -> Vec<toimg::Bin>{
    let (lens, raw) = data.split_at(2);
    let image_len: u16 = ((lens[0] as u16) << 8) | lens[1] as u16;
    let (imgdata, _p) = raw.split_at(image_len as usize);
    let extracted = inflate_bytes_zlib(imgdata).expect("Image extract error");
    let token = tokenize(extracted.as_slice());
    return tokens2imgs(token);
}