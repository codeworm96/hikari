use crate::aabb::{surrounding_box, AABB};
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res = None;
        let mut closest_so_far = t_max;
        for h in &self.list {
            if let Some(rec) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                res = Some(rec);
            }
        }
        res
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let mut res = None;
        for h in &self.list {
            if let Some(r) = h.bounding_box(t0, t1) {
                res = match res {
                    Some(res) => Some(surrounding_box(&res, &r)),
                    None => Some(r),
                }
            } else {
                return None;
            }
        }
        res
    }
}
