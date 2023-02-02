pub mod hittable;
pub mod objects;
pub mod ray;
pub mod util;
pub mod vec3;

use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use hittable::Hittable;
use objects::sphere::Sphere;
use util::INFTY;
use vec3::Vec3;

use crate::{hittable::HittableList, ray::Ray};

fn ray_color(ray: &Ray, world: &hittable::HittableList<Sphere>) -> Vec3 {
    match world.hit(&ray, 0.0, INFTY) {
        Some(record) => 0.5 * (record.normal + Vec3::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = Vec3::unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    /*
     * PPM File:
     * P3
     * <image_width> <image_height>
     * <max_color=255>
     * r g b  r g b  ...  r g b
     * ...
     * r g b  r g b  ...  r g b
     */

    // io
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("image.ppm")
        .expect("Unable to open file");
    let mut writer = BufWriter::new(file);

    // init logging
    dotenv::dotenv().ok();
    env_logger::init();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio).round() as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // print header
    writer
        .write_all(
            format!(
                r###"P3
{image_width} {image_height}
{max_color}
"###,
                image_width = image_width,
                image_height = image_height,
                max_color = 255,
            )
            .as_bytes(),
        )
        .expect("Unable to write data");

    log::debug!("wrote header");

    let mut world = HittableList { objects: vec![] };

    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    world.add(sphere2);
    world.add(sphere);

    // draw line by line from top to bottom
    for j in (0..image_height).rev() {
        // from left to right
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel = ray_color(&r, &world);

            util::write_color(&mut writer, pixel);

            log::debug!(
                "wrote data {} / {}",
                (image_height - j - 1) * image_width + i + 1,
                image_height * image_width
            );
        }
    }
}
