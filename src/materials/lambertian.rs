use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct Lambertian {
    pub color: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        record: crate::hittable::HitRecord,
    ) -> Option<(Vec3, crate::ray::Ray)> {
        // let target = record.point + record.normal + Vec3::random_unit_vector();
        // let target = record.point + record.normal + Vec3::random_in_unit_sphere();

        let mut scatter_direction = record.normal + Vec3::random_unit_vector();
        scatter_direction = if scatter_direction.near_zero() {
            record.normal
        } else {
            scatter_direction
        };
        let scattered_ray = Ray::new(record.point, scatter_direction);
        Some((self.color, scattered_ray))
    }
}
