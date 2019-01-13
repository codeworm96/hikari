use lazy_static::lazy_static;
use rand::prelude::*;

use crate::vec3::{dot, Vec3};

fn generate() -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    let mut res = Vec::new();
    for _ in 0..256 {
        res.push(
            Vec3::new(
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
            )
            .unit(),
        )
    }
    res
}

fn permute(p: &mut Vec<usize>) {
    let mut rng = rand::thread_rng();
    for i in (1..p.len()).rev() {
        let target = rng.gen_range(0, i + 1);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn generate_perm() -> Vec<usize> {
    let mut res = Vec::new();
    for i in 0..256 {
        res.push(i)
    }
    permute(&mut res);
    res
}

lazy_static! {
    static ref RAN_FLOAT: Vec<Vec3> = generate();
    static ref PERM_X: Vec<usize> = generate_perm();
    static ref PERM_Y: Vec<usize> = generate_perm();
    static ref PERM_Z: Vec<usize> = generate_perm();
}

pub fn noise(p: &Vec3) -> f64 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let i = p.x().floor() as usize;
    let j = p.y().floor() as usize;
    let k = p.z().floor() as usize;
    let acc = (1.0 - uu)
        * (1.0 - vv)
        * (1.0 - ww)
        * dot(
            &RAN_FLOAT[PERM_X[i & 255] ^ PERM_Y[j & 255] ^ PERM_Z[k & 255]],
            &Vec3::new(u, v, w),
        )
        + (1.0 - uu)
            * (1.0 - vv)
            * ww
            * dot(
                &RAN_FLOAT[PERM_X[i & 255] ^ PERM_Y[j & 255] ^ PERM_Z[(k + 1) & 255]],
                &Vec3::new(u, v, w - 1.0),
            )
        + (1.0 - uu)
            * vv
            * (1.0 - ww)
            * dot(
                &RAN_FLOAT[PERM_X[i & 255] ^ PERM_Y[(j + 1) & 255] ^ PERM_Z[k & 255]],
                &Vec3::new(u, v - 1.0, w),
            )
        + (1.0 - uu)
            * vv
            * ww
            * dot(
                &RAN_FLOAT[PERM_X[i & 255] ^ PERM_Y[(j + 1) & 255] ^ PERM_Z[(k + 1) & 255]],
                &Vec3::new(u, v - 1.0, w - 1.0),
            )
        + uu * (1.0 - vv)
            * (1.0 - ww)
            * dot(
                &RAN_FLOAT[PERM_X[(i + 1) & 255] ^ PERM_Y[j & 255] ^ PERM_Z[k & 255]],
                &Vec3::new(u - 1.0, v, w),
            )
        + uu * (1.0 - vv)
            * ww
            * dot(
                &RAN_FLOAT[PERM_X[(i + 1) & 255] ^ PERM_Y[j & 255] ^ PERM_Z[(k + 1) & 255]],
                &Vec3::new(u - 1.0, v, w - 1.0),
            )
        + uu * vv
            * (1.0 - ww)
            * dot(
                &RAN_FLOAT[PERM_X[(i + 1) & 255] ^ PERM_Y[(j + 1) & 255] ^ PERM_Z[k & 255]],
                &Vec3::new(u - 1.0, v - 1.0, w),
            )
        + uu * vv
            * ww
            * dot(
                &RAN_FLOAT[PERM_X[(i + 1) & 255] ^ PERM_Y[(j + 1) & 255] ^ PERM_Z[(k + 1) & 255]],
                &Vec3::new(u - 1.0, v - 1.0, w - 1.0),
            );
    acc
}

pub fn turb(p: &Vec3, depth: u32) -> f64 {
    let mut acc = 0.0;
    let mut p = *p;
    let mut weight = 1.0;
    for _ in 0..depth {
        acc += weight * noise(&p);
        weight *= 0.5;
        p = p * 2.0;
    }
    acc.abs()
}
