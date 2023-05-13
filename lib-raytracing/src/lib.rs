use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use util::INFTY;
use vec3::Vec3;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod util;
pub mod vec3;

pub fn render_scene(
    scene: &HittableList,
    camera: &Camera,
    image_height: u32,
    image_width: u32,
    max_bounce: u32,
) -> Vec<Vec3> {
    (0..image_height * image_width)
        .into_par_iter()
        .map(|x| {
            let i = (x) % image_width;
            let j = image_height - ((x - i) / image_width);
            let u = ((i as f64) + util::random()) / ((image_width - 1) as f64);
            let v = ((j as f64) + util::random()) / ((image_height - 1) as f64);

            let ray = camera.shoot_ray(u, v);
            raytrace(&ray, &scene, max_bounce)
        })
        .collect()
}

fn raytrace(ray: &Ray, scene: &HittableList, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match scene.hit(&ray, 0.001, INFTY) {
        Some(hit_record) => match hit_record.material.scatter(&ray, &hit_record) {
            Some((color, scattered_ray)) => color * raytrace(&scattered_ray, &scene, depth - 1),
            None => Vec3::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = Vec3::unit_vector(&ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0) //skybox
        }
    }
}
