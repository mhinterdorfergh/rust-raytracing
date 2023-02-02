use std::ops;

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone)]
struct IndexError;

impl Vec3 {
    fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 {
            x: e1,
            y: e2,
            z: e3,
        }
    }

    fn negate(&self) -> Vec3 {
        Vec3 {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
        }
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
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
