use crate::bgeimage::toimg;

mod tokenize;
use tokenize::tokenize;

mod towav;

pub fn bin2wav(data: &[u8]) -> Result<Vec<toimg::Bin>, String>{
    let token = tokenize(data);
    return Err(String::from(""))
}
