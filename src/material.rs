use rand::prelude::*;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Vec3, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}
