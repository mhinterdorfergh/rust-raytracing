use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,  // where is it hit
    pub normal: Vec3, // where does it point
    pub t: f64,       // distance
}

pub trait Hittable {
    fn hit(ray: Ray, t_min: f64, t_max: f64) -> Result<(bool, Option<HitRecord>)>;
}
