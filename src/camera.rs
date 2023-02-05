use crate::{ray::Ray, util, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        vfov_deg: f64,
        aspect_ratio: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
    ) -> Camera {
        let theta = util::degrees_to_radians(vfov_deg);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat); // get view direction
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - (viewport_width * u) / 2.0
                - (viewport_height * v) / 2.0
                - w,
            horizontal: viewport_width * u,
            vertical: viewport_height * v,
        }
    }

    pub fn shoot_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
