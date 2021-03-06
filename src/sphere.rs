use crate::aabb::AABB;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::util::get_sphere_uv;
use crate::vec3::{dot, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Box<dyn Material + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Box<dyn Material + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
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
                let normal = (p - self.center) * (1.0 / self.radius);
                let (u, v) = get_sphere_uv(&normal);
                Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    mat: &*self.mat,
                })
            } else {
                let t = (-b + d.sqrt()) / a;
                if t < t_max && t > t_min {
                    let p = r.point_at_parameter(t);
                    let normal = (p - self.center) * (1.0 / self.radius);
                    let (u, v) = get_sphere_uv(&normal);
                    Some(HitRecord {
                        t,
                        u,
                        v,
                        p,
                        normal,
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

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}
