// mod vec3;
// use vec3::Color;

mod color;
mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod rtweekend;
use color::Color;
use vec3::Vec3;
use vec3::Point3;
use ray::Ray;
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use sphere::Sphere;
use rtweekend::{Shared, INFINITY_F64, PI, degrees_to_radians};



use std::cmp::max;

use std::fs::File;
use std::io::{self, Write, BufWriter};

use std::time::Duration;

// fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
//     let oc: Vec3 = center - r.origin;
//     // let a: f64 = r.direction.dot(&r.direction);
//     // let b: f64 = -2.0 * r.direction.dot(&oc);
//     // let c: f64 = oc.dot(&oc) - radius * radius;
//     // let discriminant = b * b - 4.0 * a * c;
//     let a: f64 = r.direction.length_squared();
//     let h: f64 = oc.dot(&r.direction);
//     let c: f64 = oc.length_squared() - radius * radius;
//     let discriminant: f64 = h * h - a * c;

//     if discriminant < 0.0 {
//         return -1.0
//     } else {
//         // return (-b - discriminant.sqrt()) / ( 2.0 * a)
//         return h - discriminant.sqrt() /  a
//     }

//     // return (discriminant >= 0.0)
// }

fn ray_color(r: Ray, world: &HittableList) -> Color {

    if let Some(rec) = world.hit(&r, 0.001, INFINITY_F64) {
        // shade by normal (rec.normal is a Vec3)
        let shaded = 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
        return Color::from(shaded);
    }


    // if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
    //     return Color::new(0.0, 0.5, 0.7)
    // }

    // let t: f64 = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    // if t > 0.0 {
    //     let N: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
    //     return color::Color(Color::new(N.x + 1.0, N.y + 1.0, N.z + 1.0).0 * 0.5)
    // }

    let unit_direction: Vec3 = r.direction.unit_vector();
    let a: f64 = 0.5 * (unit_direction.y + 1.0);
    let c: Vec3 = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0)
          + a * Vec3::new(0.5, 0.7, 1.0);
    Color::from(c)
}


fn main() -> io::Result<()> {

    // Image
    // let image_width: usize = 256;
    // let image_height: usize = 256;

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = ((image_width as f64 / aspect_ratio).max(1.0)) as usize;

    // Worls

    let mut world: HittableList = HittableList::new();

    world.add(Box::new(crate::sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(crate::sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width - 1) as f64;
    let pixel_delta_v = viewport_v / (image_height - 1) as f64;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    // Open output file
    let file = File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    // Render header
    writeln!(out, "P3")?;
    writeln!(out, "{} {}", image_width, image_height)?;
    writeln!(out, "255")?;

    // Render pixels
    // lock stderr once so we can overwrite the same line in-place
    let stderr = io::stderr();
    let mut err = stderr.lock();

    for j in 0..image_height {
        // write carriage return so the next output overwrites the same line
        // sleep(Duration::from_millis(1));
        write!(err, "\rScanlines remaining: {:>3}", image_height - j)?;
        err.flush()?;
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);

            // let r = i as f64 / (image_width - 1) as f64;
            // let g = j as f64 / (image_height - 1) as f64;
            // let b = 0.5;
            let pixel_color = ray_color(ray, &world);
            pixel_color.write_ppm(&mut out)?;
        }
    }


    out.flush()?;
    // finish the progress line and move to the next line
    writeln!(err)?;
    eprintln!("Wrote image.ppm ({}x{})", image_width, image_height);
    Ok(())
}
