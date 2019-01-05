use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(&oc, r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let d = b * b - a * c;
        if d > 0.0 {
            let t = (-b - d.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) * (1.0 / self.radius),
                })
            } else {
                let t = (-b + d.sqrt()) / a;
                if t < t_max && t > t_min {
                    let p = r.point_at_parameter(t);
                    Some(HitRecord {
                        t,
                        p,
                        normal: (p - self.center) * (1.0 / self.radius),
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
