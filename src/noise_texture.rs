use crate::perlin;
use crate::texture::Texture;
use crate::vec3::Vec3;

pub struct NoiseTexture {
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture { scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * (1.0 + (self.scale * p.z() + 10.0 * perlin::turb(p, 7)).sin())
            * 0.5
    }
}
