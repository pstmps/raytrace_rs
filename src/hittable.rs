use std::ops::{Add, Sub, Mul, Div, Neg, Deref, DerefMut};

use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::material::MaterialPtr;
// use crate::rtweekend::Shared;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: MaterialPtr,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, r: &Ray, outward_normal: Vec3, mat: MaterialPtr) -> Self {
        let front_face: bool = r.direction.dot(&outward_normal) < 0.0;
        let normal: Vec3 = if front_face { outward_normal } else { -outward_normal };
        Self { p, normal, t, front_face, mat }
    }
}

pub trait Hittable: Send + Sync {
    /// Return Some(HitRecord) if the ray hits the object in (t_min, t_max), else None.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}