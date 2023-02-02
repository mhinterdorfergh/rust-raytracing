pub mod ray;
pub mod util;
pub mod vec3;

use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use vec3::Vec3;

use crate::ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    /*
     * solution from wikipedia
     * equation to calculate intersection:
     *  d²*(u.u)+2*d*[u.(o-c)]+(o-c).(o-c)-r²=0
     *  where d is the distance between a point on a line and its origin
     *        u is the direction of the line (ray)
     *        o is the origin of the line (ray)
     *        c is the center of the sphere
     *        r is the radius of the sphere
     * solving for d results in
     * -[u.(o-c)]+-sqrt((u.(o-c))²-||u||²*(||o-c||²-r²))
     * --------------------------------------------------
     * ||u||²
     *
     * if d results in a value greater than zero, the ray intersects with the sphere
     */

    let u = Vec3::unit_vector(ray.direction);
    let oc = ray.origin - center;
    let oc_len = oc.length();

    let discriminant = Vec3::dot(u, oc) * Vec3::dot(u, oc) - (oc_len * oc_len - radius * radius);
    /*
    how to calculate distance from hitting point to ray origin
    let d_plus = -Vec3::dot(u, oc) + delta.sqrt();
    let d_minus = -Vec3::dot(u, oc) - delta.sqrt();
    d_plus > 0.0 || d_minus > 0.0 */

    if discriminant <= 0.0 {
        -1.0
    } else {
        let a = Vec3::dot(ray.direction, ray.direction); // u * u -> ray direction
        let b = 2.0 * Vec3::dot(oc, ray.direction);
        (-b - discriminant.sqrt()) / (2.0 * a)
    }

    /*
    solution from book
    let vec_origin_center = ray.origin - center; // calculate vector between point and circle center
    let direction_len = Vec3::dot(ray.direction, ray.direction); // u * u -> ray direction
    let b = 2.0 * Vec3::dot(vec_origin_center, ray.direction);
    let c = Vec3::dot(vec_origin_center, vec_origin_center) - radius * radius;
    let discriminant = b * b - 4.0 * direction_len * c;
    discriminant > 0.0 */
}

fn ray_color(ray: ray::Ray) -> Vec3 {
    let mut t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let n = Vec3::unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = Vec3::unit_vector(ray.direction);
    t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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
            let pixel = ray_color(r);

            util::write_color(&mut writer, pixel);

            log::debug!(
                "wrote data {} / {}",
                (image_height - j - 1) * image_width + i + 1,
                image_height * image_width
            );
        }
    }
}
