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
mod material;
use color::Color;

use vec3::Vec3;
use vec3::Point3;
use ray::Ray;
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use sphere::Sphere;
use rtweekend::{Shared, INFINITY_F64, PI, degrees_to_radians};
use camera::Camera;
use material::{Lambertian,Metal, Dielectric};

use std::sync::Arc;



use std::cmp::max;

use std::fs::File;
use std::io::{self, Write, BufWriter};

use std::time::Duration;

fn main() -> io::Result<()> {

    let multithreaded = std::env::args().any(|arg| arg == "--mt" || arg == "-mt");
    // Worls

    let mut world: HittableList = HittableList::new();

    let mat_ground = Shared::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Shared::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let mat_left   = Shared::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_left = Shared::new(Dielectric::new(1.00 / 1.33));
    let mat_right  = Shared::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Shared::new(crate::sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center)));
    world.add(Shared::new(crate::sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Shared::new(crate::sphere::Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left)));
    world.add(Shared::new(crate::sphere::Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right)));

    // let mut cam = Camera::default();
    // cam.image_width = 800;
    // cam.aspect_ratio = 4.0 / 3.0;

    // let mut cam = Camera::new_with(1280, 16.0 / 9.0, 100, 500);
    let mut cam = Camera::new_with(640, 16.0 / 9.0, 10, 50);

    if multithreaded {
        eprintln!("Rendering multithreaded...");
        cam.render_multithreaded(&world)?;
    } else {
        eprintln!("Rendering single-threaded...");
        cam.render(&world)?;
    }
    // cam.render(&world)?;


    Ok(())
}
