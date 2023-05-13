use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    view_plane_vector_one: Vec3,
    view_plane_vector_2: Vec3,
    view_direction: Vec3,
    lens_radius: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 10.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            20.0,
            16.0 / 9.0,
            0.1,
            10.0,
        )
    }
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        view_up: Vec3,
        vertical_field_of_view_deg: f64,
        aspect_ratio: f64,
        aperture: f64,   // lens diameter
        focus_dist: f64, // distance between lens and focus plane
    ) -> Camera {
        let vertical_field_of_view_rad = crate::degrees_to_radians!(vertical_field_of_view_deg); // theta
        let vfov_distance_ratio = (vertical_field_of_view_rad / 2.0).tan(); // h
        let viewport_height = 2.0 * vfov_distance_ratio;
        let viewport_width = aspect_ratio * viewport_height;

        // use view_up and view_direction to calculate a plane that represents the cameras
        // orientation in 2D
        let view_direction = Vec3::unit_vector(&(lookfrom - lookat)); // w
        let view_plane_vector_1 = Vec3::unit_vector(&Vec3::cross(&view_up, &view_direction)); // u
        let view_plane_vector_2 = Vec3::cross(&view_direction, &view_plane_vector_1); // v

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - (focus_dist * viewport_width * view_plane_vector_1) / 2.0
                - (focus_dist * viewport_height * view_plane_vector_2) / 2.0
                - focus_dist * view_direction,
            horizontal: focus_dist * viewport_width * view_plane_vector_1,
            vertical: focus_dist * viewport_height * view_plane_vector_2,
            view_plane_vector_one: view_plane_vector_1,
            view_plane_vector_2,
            view_direction,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn shoot_ray(&self, viewport_x: f64, viewport_y: f64) -> Ray {
        let random_xy_plane_offset = self.lens_radius * Vec3::random_in_unit_disk();
        let offset_vector = self.view_plane_vector_one * random_xy_plane_offset.x
            + self.view_plane_vector_2 * random_xy_plane_offset.y;
        let viewport_target =
            self.lower_left_corner + viewport_x * self.horizontal + viewport_y * self.vertical;
        Ray::new(
            self.origin + offset_vector,                   // ray origin
            viewport_target - self.origin - offset_vector, // ray direction
        )
    }
}
