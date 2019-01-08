use std::f64::consts::PI;

use rand::prelude::*;

use crate::ray::Ray;
use crate::util::random_in_unit_disk;
use crate::vec3::{cross, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * PI / 180.0;
        println!("{:?}", focus_dist);
        let half_height = (theta / 2.0).tan() * focus_dist;
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit();
        let u = cross(&vup, &w).unit();
        let v = cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - u * half_width - v * half_height - w * focus_dist,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
