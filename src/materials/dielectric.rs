use crate::{material::Material, ray::Ray, util::random, vec3::Vec3};

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        record: crate::hittable::HitRecord,
    ) -> Option<(crate::vec3::Vec3, crate::ray::Ray)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = Vec3::unit_vector(&ray.direction);

        let cos_theta = Vec3::dot(&(unit_direction * -1.0), &record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if (refraction_ratio * sin_theta) > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > random()
        {
            Vec3::reflect(&unit_direction, &record.normal)
        } else {
            Vec3::refract(&unit_direction, &record.normal, refraction_ratio)
        };

        let scattered = Ray::new(record.point, direction);

        Some((Vec3::new(1.0, 1.0, 1.0), scattered))
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
