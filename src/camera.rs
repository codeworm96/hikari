use std::f64::consts::PI;

use crate::ray::Ray;
use crate::vec3::{cross, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit();
        let u = cross(&vup, &w).unit();
        let v = cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - u * half_width - v * half_height - w,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
