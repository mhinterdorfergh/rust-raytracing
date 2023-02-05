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

use material::Material;
use materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use objects::sphere::Sphere;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
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

fn random_scene<'a>() -> HittableList {
    let mut world: HittableList = Default::default();

    let ground_material = Lambertian {
        color: Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    };
    world.add(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        material: Box::new(ground_material),
        radius: 1000.0,
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random();
            let center = Vec3::new(
                (a as f64) + 0.9 * util::random(),
                0.2,
                (b as f64) + 0.9 * util::random(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Box<dyn Material> = if choose_mat < 0.8 {
                    Box::new(Lambertian {
                        color: Vec3::random() * Vec3::random(),
                    })
                } else if choose_mat < 0.95 {
                    Box::new(Metal {
                        color: Vec3::random_range(0.5, 1.0),
                        fuzz: util::random_range(0.0, 0.5),
                    })
                } else {
                    Box::new(Dielectric {
                        index_of_refraction: 1.5,
                    })
                };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material: material,
                });
            }
        }
    }
    world.add(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        material: Box::new(Dielectric {
            index_of_refraction: 1.5,
        }),
        radius: 1.0,
    });
    world.add(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        material: Box::new(Lambertian {
            color: Vec3 {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            },
        }),
        radius: 1.0,
    });
    world.add(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        material: Box::new(Metal {
            color: Vec3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        }),
        radius: 1.0,
    });
    world
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
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = ((image_width as f64) / aspect_ratio).round() as u32;
    let samples_per_pixel: u32 = 100;
    let max_bounce: u32 = 50;

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // world
    let world = random_scene();
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

    let calc_count = image_height * image_width;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .expect("Could not create threadpool");

    let pixels: Vec<Vec3> = (0..calc_count)
        .into_par_iter()
        .into_par_iter()
        .map(|x| {
            let i = (x) % image_width;
            let j = image_height - ((x - i) / image_width);
            let mut samples: Vec<Vec3> = vec![];

            (0..samples_per_pixel).into_iter().for_each(|s| {
                let u = ((i as f64) + util::random()) / ((image_width - 1) as f64);
                let v = ((j as f64) + util::random()) / ((image_height - 1) as f64);

                let ray = camera.shoot_ray(u, v);

                samples.push(ray_color(&ray, &world, max_bounce));
            });

            samples
                .into_par_iter()
                .reduce(|| Vec3::default(), |a, b| a + b)
        })
        .collect();

    pixels
        .into_iter()
        .for_each(|p| write_color(&mut writer, p, samples_per_pixel))
}
