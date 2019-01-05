use image::{ImageBuffer, Rgb};

mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;
use crate::hitable::{HitRecord, Hitable};
use crate::hitable_list::HitableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{dot, Vec3};

const W: u32 = 200;
const H: u32 = 100;

fn color(r: &Ray, world: &dyn Hitable) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, std::f64::MAX) {
        Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        ) * 0.5
    } else {
        let unit_direction = r.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let mut img = ImageBuffer::from_pixel(W, H, Rgb([0u8, 0u8, 0u8]));
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let world = HitableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);
    for x in 0..W {
        for y in 0..H {
            let u = x as f64 / W as f64;
            let v = 1.0 - y as f64 / H as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r, &world);
            let ir = (col.r() * 255.99) as u8;
            let ig = (col.g() * 255.99) as u8;
            let ib = (col.b() * 255.99) as u8;
            img.put_pixel(x, y, Rgb([ir, ig, ib]));
        }
    }
    img.save("out.png").unwrap();
}
