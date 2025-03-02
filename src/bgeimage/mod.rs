extern crate image;

use inflate::inflate_bytes_zlib;

mod tokenize;
use tokenize::tokenize;

pub mod toimg;
use toimg::tokens2imgs;

pub fn bin2img(data: &[u8]) -> Result<Vec<toimg::Bin>, String>{
    let (lens, raw) = data.split_at(2);
    let image_len: u16 = ((lens[0] as u16) << 8) | lens[1] as u16;
    let (imgdata, _p) = raw.split_at(image_len as usize);
    let extres = inflate_bytes_zlib(imgdata);
    if let Ok(extracted) = extres {
        let token = tokenize(extracted.as_slice());
        return Ok(tokens2imgs(token)?);
    }else{
        return Err(String::from("Failed to extract graphic"));
    }
}