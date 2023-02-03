pub mod camera;
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
use util::{random_double, write_color, INFTY};
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
    let image_width: i32 = 3840;
    let image_height: i32 = ((image_width as f64) / aspect_ratio).round() as i32;
    let samples_per_pixel: u32 = 100;

    // Camera
    let camera = camera::Camera::new();
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
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + util::rand()) / ((image_width - 1) as f64);
                let v = ((j as f64) + util::rand()) / ((image_height - 1) as f64);

                let ray = camera.shoot_ray(u, v);

                pixel_color += ray_color(&ray, &world);
            }
            write_color(&mut writer, pixel_color, samples_per_pixel);
        }
    }
}
