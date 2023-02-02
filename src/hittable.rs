use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,      // where is it hit
    pub normal: Vec3,     // where does it point
    pub t: f64,           // distance
    pub front_face: bool, // does the hit come from a ray facing in or out the object
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
