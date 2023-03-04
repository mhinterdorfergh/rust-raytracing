use std::io::{BufWriter, Write};

use rand::Rng;

pub fn write_color<W: Write>(out: &mut BufWriter<W>, pixel_color: crate::vec3::Vec3) {
    // use clamp to limit the color between min and max to prevent over/underflow
    out.write_all(
        format!(
            "{} {} {}\n",
            (256.0 * clamp(pixel_color.x, 0.0, 0.999)).round() as i32,
            (256.0 * clamp(pixel_color.y, 0.0, 0.999)).round() as i32,
            (256.0 * clamp(pixel_color.z, 0.0, 0.999)).round() as i32
        )
        .as_bytes(),
    )
    .expect("Could not write data");
}

pub const INFTY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

#[macro_export]
macro_rules! degrees_to_radians {
    ($degrees: expr) => {
        $degrees * std::f64::consts::PI / 180.0
    };
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
