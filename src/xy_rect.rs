use crate::aabb::AABB;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mat: Box<dyn Material + Sync>,
}

impl XyRect {
    pub fn new(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        mat: Box<dyn Material + Sync>,
    ) -> XyRect {
        XyRect {
            x0,
            x1,
            y0,
            y1,
            k,
            mat,
        }
    }
}

impl Hitable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            None
        } else {
            let x = r.origin().x() + t * r.direction().x();
            let y = r.origin().y() + t * r.direction().y();
            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                Some(HitRecord {
                    t,
                    u: (x - self.x0) / (self.x1 - self.x0),
                    v: (y - self.y0) / (self.y1 - self.y0),
                    p: r.point_at_parameter(t),
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    mat: &*self.mat,
                })
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}
