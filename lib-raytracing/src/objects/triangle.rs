use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub material: Arc<dyn Material>,
}
impl Triangle {
    fn get_surface_normal(&self) -> Vec3 {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        Vec3::cross(&ab, &ac)
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        const EPSILON: f64 = 0.0000001;
        let ab = self.b - self.a;
        let ac = self.c - self.a;

        let plane_vector = Vec3::cross(&ray.direction, &ac);
        let determinant = Vec3::dot(&ab, &plane_vector);

        if determinant.abs() < EPSILON {
            return None;
        }

        let inverse_determinant = 1.0 / determinant;
        let t_vector = ray.origin - self.a;
        let u = inverse_determinant * Vec3::dot(&t_vector, &plane_vector);

        if u < -EPSILON || u > 1.0 + EPSILON {
            return None;
        }
        let q_vector = Vec3::cross(&t_vector, &ab);
        let v = inverse_determinant * Vec3::dot(&ray.direction, &q_vector);

        if v < -EPSILON || u + v > 1.0 {
            return None;
        }

        let t = inverse_determinant * Vec3::dot(&ac, &q_vector);
        if t > EPSILON && t < t_max && t > t_min {
            Some(HitRecord {
                point: ray.at(t),
                normal: self.get_surface_normal(),
                distance: t,
                front_face: false,
                material: &*self.material,
            })
        } else {
            None
        } // ray intersects
    }
}
