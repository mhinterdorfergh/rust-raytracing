use crate::{ray::Ray, util, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov_deg: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
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
                - (focus_dist * viewport_width * u) / 2.0
                - (focus_dist * viewport_height * v) / 2.0
                - focus_dist * w,
            horizontal: focus_dist * viewport_width * u,
            vertical: focus_dist * viewport_height * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn shoot_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
