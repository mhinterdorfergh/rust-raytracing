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

use material::Material;
use materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use objects::sphere::Sphere;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use util::{write_color, INFTY};
use vec3::Vec3;

use crate::{hittable::HittableList, ray::Ray};

fn ray_color(ray: &Ray, hittable_list: &hittable::HittableList, max_depth: u32) -> Vec3 {
    if max_depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match hittable_list.hit(&ray, 0.001, INFTY) {
        Some(hit_record) => match hit_record.material.scatter(ray, hit_record) {
            Some((color, scattered_ray)) => {
                color * ray_color(&scattered_ray, hittable_list, max_depth - 1)
            }
            None => Vec3::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = Vec3::unit_vector(&ray.direction);
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
                    material,
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
    // io
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("image.ppm")
        .expect("Unable to open file");
    let mut output_writer = BufWriter::new(output_file);

    // init logging
    dotenv::dotenv().ok();
    env_logger::init();

    // constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 320;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 10;
    const MAX_BOUNCE: u32 = 12;
    const FIELD_OF_VIEW: f64 = 20.0;
    const CALC_COUNT: u32 = IMAGE_HEIGHT * IMAGE_WIDTH;
    const GAMMA: f64 = 2.0;

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0); // position of the camera
    let lookat = Vec3::new(0.0, 0.0, 0.0); // the position the camera looks at
    let view_up = Vec3::new(0.0, 1.0, 0.0); // tilt of the camera
                                            // dist_to_focus & aperture are used for defocus blur or depth of field
                                            // the higher the dist_to_focus and the lower the aperture ... the smaller is the sharp area
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = camera::Camera::new(
        lookfrom,
        lookat,
        view_up,
        FIELD_OF_VIEW,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // world
    let world = random_scene();
    // print header
    output_writer
        .write_all(
            format!(
                r###"P3
{image_width} {image_height}
{max_color}
"###,
                image_width = IMAGE_WIDTH,
                image_height = IMAGE_HEIGHT,
                max_color = 255,
            )
            .as_bytes(),
        )
        .expect("Unable to write data");

    let pixels: Vec<Vec3> = (0..CALC_COUNT)
        .into_par_iter()
        .map(|x| {
            let i = (x) % IMAGE_WIDTH;
            let j = IMAGE_HEIGHT - ((x - i) / IMAGE_WIDTH);
            ((0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_s| {
                    let u = ((i as f64) + util::random()) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + util::random()) / ((IMAGE_HEIGHT - 1) as f64);

                    let ray = camera.shoot_ray(u, v);

                    ray_color(&ray, &world, MAX_BOUNCE)
                })
                .reduce(|| Vec3::default(), |a, b| a + b)
                / SAMPLES_PER_PIXEL as f64) // calc average pixel value
                .pow(1.0 / GAMMA) // gamma correction
        })
        .collect();

    pixels
        .into_iter()
        .for_each(|pixel_value| write_color(&mut output_writer, pixel_value))
}
