use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::rtweekend::Shared;
use std::sync::Arc;

use crate::material::MaterialPtr;

pub struct HittableList {
    pub objects: Vec<Shared<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self { Self { objects: Vec::new() } }
    pub fn with_capacity(cap: usize) -> Self { Self { objects: Vec::with_capacity(cap) } }

    /// push a boxed trait object
    pub fn add(&mut self, object: Shared<dyn Hittable>) {
        self.objects.push(object);
    }

    /// convenience generic push for concrete types
    pub fn push<T>(&mut self, v: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Arc::new(v));
    }

    pub fn clear(&mut self) { self.objects.clear() }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, t_min, closest) {
                closest = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}