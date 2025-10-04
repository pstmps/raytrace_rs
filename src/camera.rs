use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::cmp::max;

use crate::color::Color;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::rtweekend::{Shared, INFINITY_F64, PI, degrees_to_radians};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::hittable_list::HittableList;


pub struct Camera {

    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400usize;
        Self {
            aspect_ratio,
            image_width,
            image_height: 0, // will be computed in initialize()
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera{

    pub fn new_with(image_width: usize, aspect_ratio: f64) -> Self {
        let mut c = Self::default();
        c.image_width = image_width;
        c.aspect_ratio = aspect_ratio;
        c
    }

    fn initialize(&mut self) -> Result<(), String>{

        if self.aspect_ratio <= 0.0 {
            return Err("aspect_ratio must be > 0".into());
        }
        if self.image_width == 0 {
            return Err("image_width must be > 0".into());
        }

        // self.image_height = ((image_width as f64 / aspect_ratio).max(1.0)) as usize;
        self.image_height = ((self.image_width as f64) / self.aspect_ratio).max(1.0) as usize;

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * ((self.image_width as f64) / (self.image_height as f64));
        // let camera_center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / (self.image_width - 1) as f64;
        self.pixel_delta_v = viewport_v / (self.image_height - 1) as f64;

        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        Ok(())

    }

    pub fn render(&mut self, world: &HittableList) -> io::Result<()> {

        self.initialize().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        // Open output file
        let file = File::create("image.ppm")?;
        let mut out = BufWriter::new(file);

        // Render header
        writeln!(out, "P3")?;
        writeln!(out, "{} {}", self.image_width, self.image_height)?;
        writeln!(out, "255")?;

        // Render pixels
        // lock stderr once so we can overwrite the same line in-place
        let stderr = io::stderr();
        let mut err = stderr.lock();

        for j in 0..self.image_height {
            // write carriage return so the next output overwrites the same line
            // sleep(Duration::from_millis(1));
            write!(err, "\rScanlines remaining: {:>3}", self.image_height - j)?;
            err.flush()?;
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;

                let ray = Ray::new(self.center, ray_direction);

                // let r = i as f64 / (image_width - 1) as f64;
                // let g = j as f64 / (image_height - 1) as f64;
                // let b = 0.5;
                let pixel_color = Self::ray_color(ray, &world);
                pixel_color.write_ppm(&mut out)?;
            }
        }


        out.flush()?;
        // finish the progress line and move to the next line
        writeln!(err)?;
        eprintln!("Wrote image.ppm ({}x{})", self.image_width, self.image_height);
        Ok(())
    }

    fn ray_color(r: Ray, world: &HittableList) -> Color {

        if let Some(rec) = world.hit(&r, 0.001, INFINITY_F64) {
            // shade by normal (rec.normal is a Vec3)
            let shaded = 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
            return Color::from(shaded);
        }

        let unit_direction: Vec3 = r.direction.unit_vector();
        let a: f64 = 0.5 * (unit_direction.y + 1.0);
        let c: Vec3 = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0)
            + a * Vec3::new(0.5, 0.7, 1.0);
        Color::from(c)
    }
}