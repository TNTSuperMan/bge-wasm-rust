pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub is_transparent: bool
}
impl Clone for Pixel {
    fn clone(&self) -> Pixel {
        Pixel {
            r: self.r,
            g: self.g,
            b: self.b,
            is_transparent: self.is_transparent
        }
    }
}
impl Pixel {
    pub fn new(c: u8) -> Pixel {
        Pixel {
            r: (c & 0b110000) >> 4,
            g: (c & 0b001100) >> 2,
            b: (c & 0b000011) >> 0,
            is_transparent: (c & 0b11000000) != 0
        }
    }
}
pub struct Image {
    pub data: Vec<Vec<Pixel>>
}
impl Image {
    pub fn clone(&self) -> Image{
        Image { data: self.data.as_slice().to_vec() }
    }
}
pub fn tokenize(data: &[u8]) -> Vec<Image>{
    let mut images: Vec<Image> = Vec::new();
    let mut stacked_data: Vec<Vec<Pixel>> = Vec::new();
    stacked_data.push(Vec::new());
    for i in 0..data.len() {
        match (data[i] & 0b11000000) >> 6 {
            0 => {
                let last_i = stacked_data.len() - 1;
                stacked_data[last_i].push(Pixel::new(data[i]));
            },
            1 => {
                let last_i = stacked_data.len() - 1;
                stacked_data[last_i].push(Pixel::new(data[i]));
            },
            2 => {
                stacked_data.push(Vec::new());
            },
            3 => {
                images.push(Image{
                    data: stacked_data.as_slice().to_vec()
                })
            }
            _ => {}
        }
    }
    return images;
}