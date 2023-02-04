use std::ops;

use crate::util::{random, random_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
struct IndexError;

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 {
            x: e1,
            y: e2,
            z: e3,
        }
    }

    pub fn negate(&self) -> Vec3 {
        Vec3 {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: (u.y * v.z - u.z * v.y),
            y: (u.z * v.x - u.x * v.z),
            z: (u.x * v.y - u.y * v.x),
        }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
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
            let p = Self::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::unit_vector(Self::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere * -1.0
        }
    }

    pub fn near_zero(&self) -> bool {
        const s: f64 = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Self::dot(v, n) * n
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
