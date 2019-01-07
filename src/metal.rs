use rand::prelude::*;

use crate::hitable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::util::random_in_unit_sphere;
use crate::vec3::{dot, Vec3};

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Vec3, f: f64) -> Metal {
        Metal { albedo: a, fuzz: f }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * 2.0 * dot(v, n)
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r.direction().unit(), &rec.normal);
        let direction = reflected + random_in_unit_sphere(rng) * self.fuzz;
        if dot(&direction, &rec.normal) > 0.0 {
            Some((self.albedo, Ray::new(rec.p, direction)))
        } else {
            None
        }
    }
}
