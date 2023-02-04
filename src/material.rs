use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Vec3, Ray)>;
}
