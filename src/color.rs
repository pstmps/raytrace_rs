use std::ops::{Deref, DerefMut, Mul, Add, AddAssign, Div};
use crate::vec3::Vec3;
use crate::interval::Interval;

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
        let intensity = Interval::new(0.000, 0.999);
        let r = (256.0 * intensity.clamp(self.r())) as i32;
        let g = (256.0 * intensity.clamp(self.g())) as i32;
        let b = (256.0 * intensity.clamp(self.b())) as i32;
        (r, g, b)
    }

    pub fn write_ppm(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        let (r, g, b) = self.to_rgb_i32();
        writeln!(out, "{} {} {}", r, g, b)
    }
}

// allow `Color + Color`
impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

// allow `color += other_color`
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        // delegate to Vec3 ops
        self.0 = self.0 + rhs.0;
    }
}

// allow `Color * f64`
impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs)
    }
}

// allow `Color / f64`
impl Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Self::Output {
        Color(self.0 / rhs)
    }
}

impl From<Vec3> for Color { fn from(v: Vec3) -> Self { Self(v) } }
impl From<Color> for Vec3 { fn from(c: Color) -> Vec3 { c.0 } }