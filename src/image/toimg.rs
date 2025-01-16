extern crate image;
use std::io::Cursor;

use image::{ImageBuffer, Rgba};
use base64::encode;

use crate::image::tokenize::Image;

pub fn tokens2imgs(tokens: Vec<Image>) -> Vec<String>{
    let mut imgs: Vec<String> = Vec::new();
    for i in 0..tokens.len() {
        imgs.push(token2img(tokens[i].clone()));
    }
    return imgs;
}

const TRANSPARENT: Rgba<u8> = Rgba([0,0,0,0]);
fn token2img(token: Image) -> String{
    let mut width: usize = 0;
    let height: usize = token.data.len();
    for i in 0..height {
        if width < token.data[i].len() {
            width = token.data[i].len();
        }
    }
    let mut img = ImageBuffer::from_pixel(width as u32, height as u32, TRANSPARENT);

    for y in 0..height {
        for x in 0..token.data[y].len() {
            if !token.data[y][x].is_transparent {
                img.put_pixel(x as u32, y as u32, Rgba([
                    token.data[y][x].r * 85,
                    token.data[y][x].g * 85,
                    token.data[y][x].b * 85,
                    1
                ]));
            }
        }
    }

    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut cursor = Cursor::new(&mut buffer);
        img.write_to(&mut cursor, image::ImageFormat::Png).expect("Failed to generate PNG");
    }
    
    return encode(buffer);
}
