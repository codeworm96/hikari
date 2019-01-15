use rand::prelude::*;

use crate::hitable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

pub struct DiffuseLight {
    emit: Box<dyn Texture + Sync>,
}

impl DiffuseLight {
    pub fn new(a: Box<dyn Texture + Sync>) -> DiffuseLight {
        DiffuseLight { emit: a }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r: &Ray, _rec: &HitRecord, _rng: &mut ThreadRng) -> Option<(Vec3, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
