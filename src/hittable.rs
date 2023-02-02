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

pub trait Hittable: Clone {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone)]
pub struct HittableList<T: Hittable> {
    pub objects: Vec<T>,
}

impl<T> HittableList<T>
where
    T: Hittable,
{
    fn clear(&mut self) {
        self.objects.clear()
    }

    fn add(&mut self, object: T) {
        self.objects.push(object.clone())
    }
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for obj in self.objects.clone() {
            match obj.hit(ray, t_min, closest_so_far) {
                Some(record) => {
                    closest_so_far = record.t;
                    hit_record = Some(record)
                }
                None => (),
            }
        }
        hit_record
    }
}
