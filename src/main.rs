use std::f64::consts::PI;

use image::{ImageBuffer, Rgb};
use rand::prelude::*;

mod camera;
mod dielectric;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod util;
mod vec3;
use crate::camera::Camera;
use crate::dielectric::Dielectric;
use crate::hitable::{HitRecord, Hitable};
use crate::hitable_list::HitableList;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{dot, Vec3};

const W: u32 = 800;
const H: u32 = 600;
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

fn random_scene(rng: &mut ThreadRng) -> HitableList {
    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Box::new(Lambertian::new(Vec3::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    ))),
                )));
            } else if choose_mat < 0.95 {
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Box::new(Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                        ),
                        0.5 * rng.gen::<f64>(),
                    )),
                )));
            } else {
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Box::new(Dielectric::new(1.5)),
                )));
            }
        }
    }
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.6, 0.7, 0.5), 0.0)),
    )));
    HitableList::new(list.into())
}

fn main() {
    let mut img = ImageBuffer::from_pixel(W, H, Rgb([0u8, 0u8, 0u8]));
    let mut rng = rand::thread_rng();
    let world = random_scene(&mut rng);
    let lookfrom = Vec3::new(5.5, 2.5, 2.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        W as f64 / H as f64,
        0.1,
        (lookfrom - lookat).len(),
    );
    for x in 0..W {
        for y in 0..H {
            let mut col = Vec3::zero();
            for _ in 0..N {
                let u = (x as f64 + rng.gen::<f64>()) / W as f64;
                let v = 1.0 - (y as f64 + rng.gen::<f64>()) / H as f64;
                let r = cam.get_ray(u, v, &mut rng);
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
