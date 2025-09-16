pub const INFINITY_F64: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

/// Shared pointer alias similar to std::shared_ptr in C++
pub type Shared<T> = std::sync::Arc<T>;

/// Common re-exports (optional, convenience)
pub use crate::color::Color;
pub use crate::ray::Ray;
pub use crate::vec3::{Vec3, Point3};