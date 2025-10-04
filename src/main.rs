// mod vec3;
// use vec3::Color;

mod color;
mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod rtweekend;
mod camera;
mod interval;
use color::Color;
use vec3::Vec3;
use vec3::Point3;
use ray::Ray;
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use sphere::Sphere;
use rtweekend::{Shared, INFINITY_F64, PI, degrees_to_radians};
use camera::Camera;

use std::sync::Arc;



use std::cmp::max;

use std::fs::File;
use std::io::{self, Write, BufWriter};

use std::time::Duration;

fn main() -> io::Result<()> {


    // Worls

    let mut world: HittableList = HittableList::new();

    world.add(Shared::new(crate::sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Shared::new(crate::sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // let mut cam = Camera::default();
    // cam.image_width = 800;
    // cam.aspect_ratio = 4.0 / 3.0;

    let mut cam = Camera::new_with(1280, 16.0 / 9.0, 100);
    cam.render(&world)?;

    Ok(())
}
