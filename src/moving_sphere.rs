use crate::aabb::{surrounding_box, AABB};
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat: Box<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat,
        }
    }

    fn center(&self, time: f64) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center(r.time());
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
                    normal: (p - self.center(r.time())) * (1.0 / self.radius),
                    mat: &*self.mat,
                })
            } else {
                let t = (-b + d.sqrt()) / a;
                if t < t_max && t > t_min {
                    let p = r.point_at_parameter(t);
                    Some(HitRecord {
                        t,
                        p,
                        normal: (p - self.center(r.time())) * (1.0 / self.radius),
                        mat: &*self.mat,
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let box0 = AABB {
            min: self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = AABB {
            min: self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        };
        Some(surrounding_box(&box0, &box1))
    }
}
