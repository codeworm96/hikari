use image::{ImageBuffer, Rgb};
use rand::prelude::*;
use rayon::prelude::*;

mod aabb;
mod bvh_node;
mod camera;
mod checker_texture;
mod constant_texture;
mod dielectric;
mod diffuse_light;
mod flip_normals;
mod hitable;
mod hitable_list;
mod image_texture;
mod lambertian;
mod material;
mod metal;
mod moving_sphere;
mod noise_texture;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod util;
mod vec3;
mod xy_rect;
mod xz_rect;
mod yz_rect;
use crate::camera::Camera;
use crate::checker_texture::CheckerTexture;
use crate::constant_texture::ConstantTexture;
use crate::dielectric::Dielectric;
use crate::diffuse_light::DiffuseLight;
use crate::flip_normals::FlipNormals;
use crate::hitable::Hitable;
use crate::hitable_list::HitableList;
use crate::image_texture::ImageTexture;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::moving_sphere::MovingSphere;
use crate::noise_texture::NoiseTexture;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::xy_rect::XyRect;
use crate::xz_rect::XzRect;
use crate::yz_rect::YzRect;

const W: u32 = 300;
const H: u32 = 300;
const N: u32 = 10000;

fn color(r: &Ray, world: &dyn Hitable, rng: &mut ThreadRng, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, std::f64::MAX) {
        let emitted = rec.mat.emitted(rec.u, rec.v, &rec.p);
        if depth < 50 {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec, rng) {
                emitted + attenuation * color(&scattered, world, rng, depth + 1)
            } else {
                emitted
            }
        } else {
            emitted
        }
    } else {
        Vec3::zero()
    }
}

fn two_spheres() -> Box<dyn Hitable + Sync> {
    Box::new(HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -10.0, 0.0),
            10.0,
            Box::new(Lambertian::new(Box::new(CheckerTexture::new(
                Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
                Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            )))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            Box::new(Lambertian::new(Box::new(CheckerTexture::new(
                Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
                Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            )))),
        )),
    ]))
}

fn two_perlin_spheres() -> Box<dyn Hitable + Sync> {
    Box::new(HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Box::new(NoiseTexture::new(4.0)))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Box::new(Lambertian::new(Box::new(NoiseTexture::new(4.0)))),
        )),
    ]))
}

fn earth() -> Box<dyn Hitable + Sync> {
    Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        10.0,
        Box::new(Lambertian::new(Box::new(ImageTexture::new(
            image::open("earth.jpg").unwrap().to_rgb(),
        )))),
    ))
}

fn random_scene(rng: &mut ThreadRng) -> Box<dyn Hitable + Sync> {
    let mut list: Vec<Box<dyn Hitable + Sync>> = Vec::new();
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Box::new(CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
        )))),
    )));
    for a in -10..10 {
        for b in -10..10 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                list.push(Box::new(MovingSphere::new(
                    center,
                    center + Vec3::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
                    0.0,
                    1.0,
                    0.2,
                    Box::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    ))))),
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
        Box::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.6, 0.7, 0.5), 0.0)),
    )));
    bvh_node::build(list, 0.0, 1.0, rng)
}

fn simple_light() -> Box<dyn Hitable + Sync> {
    Box::new(HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Box::new(NoiseTexture::new(4.0)))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Box::new(Lambertian::new(Box::new(NoiseTexture::new(4.0)))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 7.0, 0.0),
            2.0,
            Box::new(DiffuseLight::new(Box::new(ConstantTexture::new(
                Vec3::new(4.0, 4.0, 4.0),
            )))),
        )),
        Box::new(XyRect::new(
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            Box::new(DiffuseLight::new(Box::new(ConstantTexture::new(
                Vec3::new(4.0, 4.0, 4.0),
            )))),
        )),
    ]))
}

fn cornell_box() -> Box<dyn Hitable + Sync> {
    Box::new(HitableList::new(vec![
        Box::new(FlipNormals::new(Box::new(YzRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Box::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                0.12, 0.45, 0.15,
            ))))),
        )))),
        Box::new(YzRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            Box::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                0.65, 0.05, 0.05,
            ))))),
        )),
        Box::new(XzRect::new(
            213.0,
            343.0,
            227.0,
            332.0,
            554.0,
            Box::new(DiffuseLight::new(Box::new(ConstantTexture::new(
                Vec3::new(15.0, 15.0, 15.0),
            )))),
        )),
        Box::new(FlipNormals::new(Box::new(XzRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Box::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                0.73, 0.73, 0.73,
            ))))),
        )))),
        Box::new(XzRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            Box::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                0.73, 0.73, 0.73,
            ))))),
        )),
        Box::new(FlipNormals::new(Box::new(XyRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Box::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                0.73, 0.73, 0.73,
            ))))),
        )))),
    ]))
}

fn main() {
    let mut img = ImageBuffer::from_pixel(W, H, Rgb([0u8, 0u8, 0u8]));
    let mut rng = rand::thread_rng();
    let world = cornell_box();
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        W as f64 / H as f64,
        0.0,
        10.0,
        0.0,
        1.0,
    );
    for x in 0..W {
        for y in 0..H {
            let mut col: Vec3 = (0..N)
                .map(|_| {
                    (
                        (x as f64 + rng.gen::<f64>()) / W as f64,
                        1.0 - (y as f64 + rng.gen::<f64>()) / H as f64,
                    )
                })
                .collect::<Vec<_>>()
                .into_par_iter()
                .map(|(u, v)| {
                    let mut rng = rand::thread_rng();
                    let r = cam.get_ray(u, v, &mut rng);
                    color(&r, &*world, &mut rng, 0)
                })
                .sum();
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
