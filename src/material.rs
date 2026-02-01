
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vec3::{Vec3, Point3};
use crate::rtweekend::{Shared, random_double};

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
    pub fuzz: f64,
}
impl Metal {
    pub fn new(a: Color, fuzz: f64) -> Self { Self { albedo: a, fuzz: fuzz.min(1.0)} }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(&r_in.direction, &rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        // let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.normal;
        // }
        // let scattered = Ray::new(rec.p + rec.normal * 1e-4, scatter_direction);
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction.dot(&rec.normal) > 0.0{
            Some((self.albedo, scattered))
        } else {
            None
        }
        // Some((self.albedo, scattered))
    }
}

pub struct Dielectric {
    pub refraction_index: f64

}
impl Dielectric {
    pub fn new(refraction_index: f64) -> Self { Self { refraction_index: refraction_index } }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (( 1.0 - refraction_index) / ( 1.0 + refraction_index)).sqrt();

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0,1.0,1.0);
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let unit_direction = r_in.direction.unit_vector();

        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta.sqrt())).sqrt();

        let direction = match (ri * sin_theta) {
            x if x > 1.0 || self.reflectance(cos_theta, ri) > random_double() => Vec3::reflect(&unit_direction, &rec.normal),
            _ => Vec3::refract(&unit_direction, &rec.normal, ri),
        };

        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }


}