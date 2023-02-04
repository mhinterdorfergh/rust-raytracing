use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct Metal {
    pub color: Vec3,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        record: crate::hittable::HitRecord,
    ) -> Option<(Vec3, crate::ray::Ray)> {
        // let target = record.point + record.normal + Vec3::random_unit_vector();
        // let target = record.point + record.normal + Vec3::random_in_unit_sphere();

        let reflected = Vec3::reflect(Vec3::unit_vector(ray.direction), record.normal);
        let scattered = Ray::new(record.point, reflected);
        if Vec3::dot(scattered.direction, record.normal) > 0.0 {
            Some((self.color, scattered))
        } else {
            None
        }
    }
}
