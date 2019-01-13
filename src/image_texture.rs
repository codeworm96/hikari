use image::RgbImage;

use crate::texture::Texture;
use crate::vec3::Vec3;

pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> ImageTexture {
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let mut i = (u * (self.image.width() as f64)) as i32;
        let mut j = ((1.0 - v) * (self.image.height() as f64) - 0.001) as i32;
        if i < 0 {
            i = 0;
        }
        if j < 0 {
            j = 0;
        }
        if i > self.image.width() as i32 - 1 {
            i = self.image.width() as i32 - 1;
        }
        if j > self.image.height() as i32 - 1 {
            j = self.image.height() as i32 - 1;
        }
        let r = self.image.get_pixel(i as u32, j as u32)[0] as f64 / 255.0;
        let g = self.image.get_pixel(i as u32, j as u32)[1] as f64 / 255.0;
        let b = self.image.get_pixel(i as u32, j as u32)[2] as f64 / 255.0;
        Vec3::new(r * r, g * g, b * b)
    }
}
