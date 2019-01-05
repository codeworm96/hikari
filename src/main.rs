use image::{ImageBuffer, Rgb};

mod ray;
mod vec3;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

const W: u32 = 200;
const H: u32 = 100;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = *r.origin() - *center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(&oc, r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        -1.0
    } else {
        (-b - d.sqrt()) / (2.0 * a)
    }
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5
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
    for x in 0..W {
        for y in 0..H {
            let u = x as f64 / W as f64;
            let v = 1.0 - y as f64 / H as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r);
            let ir = (col.r() * 255.99) as u8;
            let ig = (col.g() * 255.99) as u8;
            let ib = (col.b() * 255.99) as u8;
            img.put_pixel(x, y, Rgb([ir, ig, ib]));
        }
    }
    img.save("out.png").unwrap();
}
