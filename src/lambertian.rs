use rand::prelude::*;

use crate::hitable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::util::random_in_unit_sphere;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Box<dyn Texture + Sync>,
}

impl Lambertian {
    pub fn new(a: Box<dyn Texture + Sync>) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere(rng);
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p, target - rec.p, r.time()),
        ))
    }
}
