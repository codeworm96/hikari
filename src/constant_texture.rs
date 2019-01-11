use crate::texture::Texture;
use crate::vec3::Vec3;

pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(c: Vec3) -> ConstantTexture {
        ConstantTexture { color: c }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color
    }
}
