use crate::texture::Texture;
use crate::vec3::Vec3;

pub struct CheckerTexture {
    odd: Box<dyn Texture + Sync>,
    even: Box<dyn Texture + Sync>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture + Sync>, even: Box<dyn Texture + Sync>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
