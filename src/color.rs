use std::ops::{Deref, DerefMut, Mul};
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(pub Vec3);  // RGB color


impl Deref for Color {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self { Self(Vec3::new(r, g, b)) }

    pub fn r(&self) -> f64 { self.0.x }
    pub fn g(&self) -> f64 { self.0.y }
    pub fn b(&self) -> f64 { self.0.z }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Self(self.0.clamp(min, max))
    }

    pub fn to_rgb_i32(&self) -> (i32, i32, i32) {
        let r = (255.999 * self.r()) as i32;
        let g = (255.999 * self.g()) as i32;
        let b = (255.999 * self.b()) as i32;
        (r, g, b)
    }

    pub fn write_ppm(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        let (r, g, b) = self.to_rgb_i32();
        writeln!(out, "{} {} {}", r, g, b)
    }
}

impl From<Vec3> for Color { fn from(v: Vec3) -> Self { Self(v) } }
impl From<Color> for Vec3 { fn from(c: Color) -> Vec3 { c.0 } }