extern crate image;
use std::io::Cursor;
use wasm_bindgen::prelude::*;


use image::{ImageBuffer, Rgba};

use crate::bgeimage::tokenize::Image;

#[wasm_bindgen]
pub struct Bin{
    data: Vec<u8>
}
impl Clone for Bin{
    fn clone(&self) -> Bin{
        Bin{
            data: self.data.as_slice().to_vec()
        }
    }
}
#[wasm_bindgen]
impl Bin{
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> Bin{
        Bin {
            data: data
        }
    }
    pub fn get(&self) -> Vec<u8>{
        return self.data.as_slice().to_vec();
    }
}

pub fn tokens2imgs(tokens: Vec<Image>) -> Vec<Bin>{
    let mut imgs: Vec<Bin> = Vec::new();
    for i in 0..tokens.len() {
        imgs.push(token2img(tokens[i].clone()));
    }
    return imgs;
}

const TRANSPARENT: Rgba<u8> = Rgba([0,0,0,0]);
fn token2img(token: Image) -> Bin{
    let mut width: usize = 0;
    let height: usize = token.data.len();
    for i in 0..height {
        if width < token.data[i].len() {
            width = token.data[i].len();
        }
    }
    let mut img = ImageBuffer::from_pixel(width.max(1) as u32, height as u32, TRANSPARENT);

    for y in 0..height {
        for x in 0..token.data[y].len() {
            if !token.data[y][x].is_transparent {
                img.put_pixel(x as u32, y as u32, Rgba([
                    token.data[y][x].r * 85,
                    token.data[y][x].g * 85,
                    token.data[y][x].b * 85,
                    255
                ]));
            }
        }
    }

    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut cursor = Cursor::new(&mut buffer);
        img.write_to(&mut cursor, image::ImageFormat::Png).expect("Failed to generate PNG");
    }
    
    return Bin::new(buffer);
}
