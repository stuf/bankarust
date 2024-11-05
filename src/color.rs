use cgmath::Vector3;

use Vector3 as Color;

pub fn write_color(pixel: Vector3<f64>) {
    let r = pixel.x;
    let g = pixel.y;
    let b = pixel.z;

    let ir = (r * 255.999) as usize;
    let ig = (g * 255.999) as usize;
    let ib = (b * 255.999) as usize;

    println!("{} {} {}", ir, ig, ib);
}

pub fn color<T>(x: T, y: T, z: T) -> Color<T> {
    Color::new(x, y, z)
}
