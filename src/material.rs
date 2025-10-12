
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vec3::{Vec3, Point3};
use crate::rtweekend::Shared;

/// object-safe trait representing a material (like a C++ abstract base)
pub trait Material: Send + Sync {
    /// return Some((attenuation, scattered_ray)) if the ray scatters, else None
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

/// runtime handle type: use Box for single ownership, or Arc (Shared) to share between threads
pub type MaterialPtr = Shared<dyn Material + Send + Sync>;

pub struct Lambertian {
    pub albedo: Color,
}
impl Lambertian {
    pub fn new(a: Color) -> Self { Self { albedo: a } }
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        // let scattered = Ray::new(rec.p + rec.normal * 1e-4, scatter_direction);
        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}


pub struct Metal {
    pub albedo: Color,
}
impl Metal {
    pub fn new(a: Color) -> Self { Self { albedo: a } }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction, &rec.normal);
        // let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.normal;
        // }
        // let scattered = Ray::new(rec.p + rec.normal * 1e-4, scatter_direction);
        let scattered = Ray::new(rec.p, reflected);

        Some((self.albedo, scattered))
    }
}