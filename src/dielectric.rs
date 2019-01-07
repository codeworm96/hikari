use rand::prelude::*;

use crate::hitable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::util::random_in_unit_sphere;
use crate::vec3::{dot, Vec3};

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit();
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * 2.0 * dot(v, n)
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Vec3, Ray)> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let dt = dot(r.direction(), &rec.normal);
        if dt > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dt / r.direction().len();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dt / r.direction().len();
        }
        if let Some(refracted) = refract(r.direction(), &outward_normal, ni_over_nt) {
            if rng.gen::<f64>() < schlick(cosine, self.ref_idx) {
                let reflected = reflect(r.direction(), &rec.normal);
                Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(rec.p, reflected)))
            } else {
                Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(rec.p, refracted)))
            }
        } else {
            let reflected = reflect(r.direction(), &rec.normal);
            Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(rec.p, reflected)))
        }
    }
}
