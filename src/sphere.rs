use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

use crate::material::MaterialPtr;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    mat: MaterialPtr,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: MaterialPtr) -> Self { Self { center, radius: radius.max(0.0) , mat} }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a: f64 = r.direction.length_squared();
        let h: f64 = oc.dot(&r.direction);
        let c: f64 = oc.length_squared() - self.radius * self.radius;


        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None
        }

        let sqrtd: f64 = discriminant.sqrt();

        let mut root = (-h - sqrtd) / a;
        if root <= ray_t_min || ray_t_max <= root{
            root = (-h + sqrtd) / a;
            if root <= ray_t_min || ray_t_max <= root {
                return None;
            }
        }

        let p: Vec3 = r.at(root);
        let normal: Vec3 = ( p - self.center) / self.radius;

        Some(HitRecord::new(p, root, r, normal, self.mat.clone()))
    }
}