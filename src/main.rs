pub mod camera;
pub mod hittable;
pub mod material;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod util;
pub mod vec3;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use hittable::Hittable;
use log::debug;

use materials::{lambertian::Lambertian, metal::Metal};
use objects::sphere::Sphere;
use util::{write_color, INFTY};
use vec3::Vec3;

use crate::{hittable::HittableList, ray::Ray};

fn ray_color(ray: &Ray, world: &hittable::HittableList, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match world.hit(&ray, 0.001, INFTY) {
        Some(record) => match record.material.scatter(ray, record) {
            Some((color, scattered_ray)) => color * ray_color(&scattered_ray, world, depth - 1),
            None => Vec3::new(0.0, 0.0, 0.0),
        },
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
    let samples_per_pixel: u32 = 100;
    let max_bounce: u32 = 50;

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

    let sphere_ground = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Lambertian {
            color: Vec3::new(0.8, 0.8, 0.0),
        },
    };

    let sphere_center = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Lambertian {
            color: Vec3::new(0.7, 0.3, 0.3),
        },
    };

    let sphere_left = Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Metal {
            color: Vec3::new(0.8, 0.8, 0.8),
        },
    };

    let sphere_right = Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Metal {
            color: Vec3::new(0.8, 0.6, 0.2),
        },
    };

    world.add(sphere_ground);
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);

    // draw line by line from top to bottom
    for j in (0..image_height).rev() {
        // from left to right
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + util::random()) / ((image_width - 1) as f64);
                let v = ((j as f64) + util::random()) / ((image_height - 1) as f64);

                let ray = camera.shoot_ray(u, v);

                pixel_color += ray_color(&ray, &world, max_bounce);
            }
            write_color(&mut writer, pixel_color, samples_per_pixel);
        }
        debug!(
            "calculated pixels {}/{} {}%",
            (image_height - j + 1) * image_width,
            image_height * image_width,
            (((image_height - j + 1) * image_width) as f64) / ((image_height * image_width) as f64)
                * 100.0
        );
    }
}
