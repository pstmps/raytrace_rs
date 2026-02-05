use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg, Deref, DerefMut};

use crate::rtweekend::{random_double, random_double_range};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3; // 3D point
// pub type Color = Vec3;  // RGB color

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length_squared(&self) -> f64 { self.dot(self) }

    pub fn length(&self) -> f64 { self.length_squared().sqrt() }

    pub fn unit_vector(&self) -> Self { *self / self.length() }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let mut p = Vec3::random_range(-1.0, 1.0);
            p.z = 0.0;
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let length_squared = p.length_squared();
            if length_squared <= 1.0 && length_squared > 1e-160 {
                return p / length_squared.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3{
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(&normal) > 0.0 {
            return on_unit_sphere;
        }
        else {
            return -on_unit_sphere;
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*uv).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;
        r_out_perp + r_out_parallel
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }

    pub fn random() -> Self {
        Self::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }
}

// Operators
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output { rhs * self }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output { self * (1.0 / rhs) }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output { Self::new(-self.x, -self.y, -self.z) }
}



// impl From<Vec3> for Point3 { fn from(v: Vec3) -> Self { Self(v) } }
// impl From<Point3> for Vec3 { fn from(p: Point3) -> Vec3 { p.0 } }








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v + w, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v.dot(&w), 32.0);
        assert_eq!(v.cross(&w), Vec3::new(-3.0, 6.0, -3.0));
    }
}
