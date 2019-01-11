use rand::prelude::*;

use crate::aabb::{surrounding_box, AABB};
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct BvhNode {
    left: Box<dyn Hitable + Sync>,
    right: Box<dyn Hitable + Sync>,
    aabb: AABB,
}

pub fn build(
    mut list: Vec<Box<dyn Hitable + Sync>>,
    time0: f64,
    time1: f64,
    rng: &mut ThreadRng,
) -> Box<dyn Hitable + Sync> {
    let len = list.len();
    if len == 0 {
        panic!("no hitables");
    } else if len == 1 {
        list.remove(0)
    } else {
        let axis = rng.gen_range(0, 3);
        /* TODO perf */
        list.sort_by(|a, b| {
            a.bounding_box(time0, time1).unwrap().min[axis]
                .partial_cmp(&b.bounding_box(time0, time1).unwrap().min[axis])
                .unwrap()
        });
        let list2 = list.split_off(len / 2);
        let left = build(list, time0, time1, rng);
        let right = build(list2, time0, time1, rng);
        let aabb = surrounding_box(
            &left.bounding_box(time0, time1).unwrap(),
            &right.bounding_box(time0, time1).unwrap(),
        );
        Box::new(BvhNode { left, right, aabb })
    }
}

impl Hitable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.aabb.hit(r, t_min, t_max) {
            match (
                self.left.hit(r, t_min, t_max),
                self.right.hit(r, t_min, t_max),
            ) {
                (Some(l), Some(r)) => {
                    if l.t < r.t {
                        Some(l)
                    } else {
                        Some(r)
                    }
                }
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.aabb)
    }
}
