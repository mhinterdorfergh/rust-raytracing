use std::io::{BufWriter, Write};

use rand::Rng;

pub fn write_color<W: Write>(
    out: &mut BufWriter<W>,
    pixel_color: crate::vec3::Vec3,
    samples_per_pixel: u32,
) {
    let scale = 1.0 / (samples_per_pixel as f64);

    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    out.write_all(
        format!(
            "{} {} {}\n",
            (256.0 * clamp(r, 0.0, 0.999)).round() as i32,
            (256.0 * clamp(g, 0.0, 0.999)).round() as i32,
            (256.0 * clamp(b, 0.0, 0.999)).round() as i32
        )
        .as_bytes(),
    )
    .expect("Could not write data");
}

pub const INFTY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
