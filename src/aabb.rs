use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let t0;
            let t1;
            if inv_d < 0.0 {
                t1 = (self.min[a] - r.origin()[a]) * inv_d;
                t0 = (self.max[a] - r.origin()[a]) * inv_d;
            } else {
                t0 = (self.min[a] - r.origin()[a]) * inv_d;
                t1 = (self.max[a] - r.origin()[a]) * inv_d;
            }
            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    AABB {
        min: Vec3::new(
            box0.min.x().min(box1.min.x()),
            box0.min.y().min(box1.min.y()),
            box0.min.z().min(box1.min.z()),
        ),
        max: Vec3::new(
            box0.max.x().max(box1.max.x()),
            box0.max.y().max(box1.max.y()),
            box0.max.z().max(box1.max.z()),
        ),
    }
}
