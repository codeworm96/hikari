use image::{ImageBuffer, Rgb};

const W: u32 = 200;
const H: u32 = 100;

fn main() {
    let mut img = ImageBuffer::from_pixel(W, H, Rgb([0u8, 0u8, 0u8]));
    for x in 0..W {
        for y in 0..H {
            let r = x as f64 / W as f64;
            let g = 1.0 - y as f64 / H as f64;
            let b = 0.2;
            let ir = (r * 255.99) as u8;
            let ig = (g * 255.99) as u8;
            let ib = (b * 255.99) as u8;
            img.put_pixel(x, y, Rgb([ir, ig, ib]));
        }
    }
    img.save("out.png").unwrap();
}
