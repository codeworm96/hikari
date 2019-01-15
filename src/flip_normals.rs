use crate::aabb::AABB;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct FlipNormals {
    ptr: Box<dyn Hitable + Sync>,
}

impl FlipNormals {
    pub fn new(ptr: Box<dyn Hitable + Sync>) -> FlipNormals {
        FlipNormals { ptr }
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(rec) = self.ptr.hit(r, t_min, t_max) {
            Some(HitRecord {
                normal: -rec.normal,
                ..rec
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.ptr.bounding_box(t0, t1)
    }
}
