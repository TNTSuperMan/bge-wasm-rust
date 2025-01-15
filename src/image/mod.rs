extern crate image;

mod tokenize;
use tokenize::tokenize;

mod toimg;
use toimg::tokens2imgs;

pub fn bin2img(data: &[u8]) -> Vec<String>{
    let token = tokenize(data);
    return tokens2imgs(token);
}