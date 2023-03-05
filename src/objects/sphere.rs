use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // P(t) = ray.origin + ray.direction* t
        // find t for which (P(t)-sphere.center) dot (P(t)-sphere.center) = sphere.radius²
        // t² * direction dot direction + 2*t*direction dot (origin - center) + (origin - center)²
        // - radius² = 0
        // t = (-dot(d, (r-s)) ± sqrt(dot(d, (r-s))² - dot(d, d) * (dot((r-s), (r-s)) - h²))) / dot(d, d)
        // t = |negative b   | ± |sqrt | b²       |  - | a     | * |  c                     ||/ |a      |

        let dist_origin_center = ray.origin - self.center;
        let b = Vec3::dot(&ray.direction, &dist_origin_center);
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let c = Vec3::dot(&dist_origin_center, &dist_origin_center) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant <= 0.0 {
            return None;
        }
        let root = (b * b - a * c).sqrt();

        let t_plus = (-b + root)/a;
        let t_minus = (-b - root)/a;

        let t = if t_minus >= t_min && t_minus <= t_max {
            t_minus
        } else if t_plus >= t_min && t_plus <= t_max {
            t_plus
        } else {
            return None;
        };
        let outward_normal = (ray.at(t) - self.center) / self.radius;

        let mut record = HitRecord {
            distance: t,
            point: ray.at(t),
            normal: (ray.at(t) - self.center) / self.radius,
            front_face: false,
            material: &self.material,
        };
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }
}
