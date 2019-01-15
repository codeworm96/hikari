use crate::aabb::AABB;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat: Box<dyn Material + Sync>,
}

impl XzRect {
    pub fn new(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        mat: Box<dyn Material + Sync>,
    ) -> XzRect {
        XzRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hitable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            None
        } else {
            let x = r.origin().x() + t * r.direction().x();
            let z = r.origin().z() + t * r.direction().z();
            if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
                None
            } else {
                Some(HitRecord {
                    t,
                    u: (x - self.x0) / (self.x1 - self.x0),
                    v: (z - self.z0) / (self.z1 - self.z0),
                    p: r.point_at_parameter(t),
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    mat: &*self.mat,
                })
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.k - 0.0001, self.z0),
            max: Vec3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }
}
