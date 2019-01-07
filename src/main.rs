use image::{ImageBuffer, Rgb};
use rand::prelude::*;

mod camera;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;
use crate::camera::Camera;
use crate::hitable::{HitRecord, Hitable};
use crate::hitable_list::HitableList;
use crate::lambertian::Lambertian;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{dot, Vec3};

const W: u32 = 200;
const H: u32 = 100;
const N: u32 = 100;

fn color(r: &Ray, world: &dyn Hitable, rng: &mut ThreadRng, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, std::f64::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec, rng) {
                attenuation * color(&scattered, world, rng, depth + 1)
            } else {
                Vec3::zero()
            }
        } else {
            Vec3::zero()
        }
    } else {
        let unit_direction = r.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let mut img = ImageBuffer::from_pixel(W, H, Rgb([0u8, 0u8, 0u8]));
    let world = HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
    ]);
    let cam = Camera::new();
    let mut rng = rand::thread_rng();
    for x in 0..W {
        for y in 0..H {
            let mut col = Vec3::zero();
            for _ in 0..N {
                let u = (x as f64 + rng.gen::<f64>()) / W as f64;
                let v = 1.0 - (y as f64 + rng.gen::<f64>()) / H as f64;
                let r = cam.get_ray(u, v);
                col = col + color(&r, &world, &mut rng, 0)
            }
            col = col * (1.0 / N as f64);
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = (col.r() * 255.99) as u8;
            let ig = (col.g() * 255.99) as u8;
            let ib = (col.b() * 255.99) as u8;
            img.put_pixel(x, y, Rgb([ir, ig, ib]));
        }
    }
    img.save("out.png").unwrap();
}
