use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        /*
         * find the nearest root that lies in the acceptable range
         * a root is a point where the ray hits an object
         */
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;

        let mut record = HitRecord {
            t: root,
            point: ray.at(root),
            normal: (ray.at(root) - self.center) / self.radius,
            front_face: false,
            material: &self.material,
        };
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }
}
