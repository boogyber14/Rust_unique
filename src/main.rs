extern crate image;
extern crate num_complex;

use image::RgbImage;
use num_complex::Complex;

fn mandelbrot_pixel(c: Complex<f64>, max_iter: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max_iter {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

fn generate_mandelbrot(width: u32, height: u32, max_iter: u32) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    let scalex = 3.0 / width as f64;
    let scaley = 2.0 / height as f64;

    for y in 0..height {
        for x in 0..width {
            let cx = x as f64 * scalex - 2.0;
            let cy = y as f64 * scaley - 1.0;
            let c = Complex::new(cx, cy);
            let iter = mandelbrot_pixel(c, max_iter);
            let color = if iter < max_iter {
                let r = (iter * 10) as u8;
                let g = (iter * 5) as u8;
                let b = (iter * 2) as u8;
                image::Rgb([r, g, b])
            } else {
                image::Rgb([0, 0, 0])
            };
            img.put_pixel(x, y, color);
        }
    }
    img
}

fn main() {
    let width = 800;
    let height = 600;
    let max_iter = 2000;
    let img = generate_mandelbrot(width, height, max_iter);
    img.save("mandelbrot.png").expect("Failed to save image");
    println!("Mandelbrot image generated!");
}
