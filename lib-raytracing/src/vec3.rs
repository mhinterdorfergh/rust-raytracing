use std::ops;

use crate::util::{self, random, random_range};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn pow(&self, exponent: f64) -> Vec3 {
        Vec3 {
            x: self.x.powf(exponent),
            y: self.y.powf(exponent),
            z: self.z.powf(exponent),
        }
    }

    pub fn negate(&self) -> Vec3 {
        *self * -1.0
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(vector_u: &Vec3, vector_v: &Vec3) -> f64 {
        vector_u.x * vector_v.x + vector_u.y * vector_v.y + vector_u.z * vector_v.z
    }

    pub fn cross(vector_u: &Vec3, vector_v: &Vec3) -> Vec3 {
        Vec3 {
            x: (vector_u.y * vector_v.z - vector_u.z * vector_v.y),
            y: (vector_u.z * vector_v.x - vector_u.x * vector_v.z),
            z: (vector_u.x * vector_v.y - vector_u.y * vector_v.x),
        }
    }

    pub fn unit_vector(vector: &Vec3) -> Vec3 {
        *vector / vector.length()
    }

    pub fn random() -> Vec3 {
        Self::new(random(), random(), random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Self::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let probant_vector = Self::random_range(-1.0, 1.0);
            if probant_vector.length_squared() >= 1.0 {
                continue;
            } else {
                return probant_vector;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::unit_vector(&Self::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere * -1.0
        }
    }

    pub fn near_zero(&self) -> bool {
        const NEAR_ZERO_APPROX: f64 = 1e-8;
        (self.x.abs() < NEAR_ZERO_APPROX)
            && (self.y.abs() < NEAR_ZERO_APPROX)
            && (self.z.abs() < NEAR_ZERO_APPROX)
    }

    pub fn reflect(vector: &Vec3, surface_normal: &Vec3) -> Vec3 {
        *vector - 2.0 * Self::dot(&vector, &surface_normal) * *surface_normal
    }

    pub fn refract(ray_direction: &Vec3, surface_normal: &Vec3, refraction_ratio: f64) -> Vec3 {
        // get unit_vectors for ray and normal
        let ray_direction_uv = Self::unit_vector(ray_direction);
        let surface_normal_uv = Self::unit_vector(surface_normal);
        let reverse_ray_uv = ray_direction_uv * -1.0;

        // get the angle theta between ray and surface_normal
        let cos_theta = Vec3::dot(&reverse_ray_uv, &surface_normal_uv).min(1.0);

        // get both parts of refracted ray according to SNELL
        let r_out_perp = refraction_ratio * (ray_direction_uv + cos_theta * surface_normal_uv);
        let r_out_parallel = surface_normal_uv * -1.0 * (1.0 - r_out_perp.length_squared()).abs();
        r_out_parallel + r_out_perp
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let probant_vector: Vec3 = Vec3::new(
                util::random_range(-1.0, 1.0),
                util::random_range(-1.0, 1.0),
                0.0,
            );
            if probant_vector.length_squared() >= 1.0 {
                continue;
            } else {
                return probant_vector;
            }
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, fact: f64) {
        self.x *= fact;
        self.y *= fact;
        self.z *= fact;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        (1.0 / other) * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negate_negates_vec() {
        let vec: Vec3 = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let neg_vec: Vec3 = Vec3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        };
        assert_eq!(vec.negate(), neg_vec)
    }
    #[test]
    fn negate_negates_vec_fail() {
        let vec: Vec3 = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_ne!(vec.negate(), vec)
    }
    #[test]
    fn new_same_as_create() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            },
            Vec3::new(1.0, 1.0, 1.0)
        )
    }
    #[test]
    fn dot_same_as_squared() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(a.length_squared(), Vec3::dot(&a, &a))
    }
    #[test]
    fn pow() {
        let subject = Vec3 {
            x: 4.2,
            y: 4.2,
            z: 4.2,
        };
        let target = Vec3 {
            x: (4.2 as f64).powf(2.0),
            y: (4.2 as f64).powf(2.0),
            z: (4.2 as f64).powf(2.0),
        };
        assert_eq!(subject.pow(2.0), target);
    }
}
