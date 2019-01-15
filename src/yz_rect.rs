use crate::aabb::AABB;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat: Box<dyn Material + Sync>,
}

impl YzRect {
    pub fn new(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        mat: Box<dyn Material + Sync>,
    ) -> YzRect {
        YzRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hitable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            None
        } else {
            let y = r.origin().y() + t * r.direction().y();
            let z = r.origin().z() + t * r.direction().z();
            if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
                None
            } else {
                Some(HitRecord {
                    t,
                    u: (y - self.y0) / (self.y1 - self.y0),
                    v: (z - self.z0) / (self.z1 - self.z0),
                    p: r.point_at_parameter(t),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    mat: &*self.mat,
                })
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.k - 0.0001, self.y0, self.z0),
            max: Vec3::new(self.k + 0.0001, self.y1, self.z1),
        })
    }
}
