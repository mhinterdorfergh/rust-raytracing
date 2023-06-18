use crate::{hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};
#[derive(Clone, Copy)]
pub struct Metal {
    pub color: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        // let target = record.point + record.normal + Vec3::random_unit_vector();
        // let target = record.point + record.normal + Vec3::random_in_unit_sphere();

        let reflected = Vec3::reflect(&Vec3::unit_vector(&ray.direction), &record.normal);
        let scattered = Ray::new(
            record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if Vec3::dot(&scattered.direction, &record.normal) > 0.0 {
            Some((self.color, scattered))
        } else {
            None
        }
    }
}
