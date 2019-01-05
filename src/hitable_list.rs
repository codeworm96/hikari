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
}
