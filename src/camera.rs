use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::cmp::max;

use crate::color::Color;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::rtweekend::{Shared, INFINITY_F64, PI, degrees_to_radians, random_double};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::hittable_list::HittableList;

use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;

use std::time::Instant;

pub struct Camera {

    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pixel_sample_scale: f64,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400usize;
        let samples_per_pixel = 10usize;
        let max_depth = 10usize;
        // let vfov = 0.0;
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov : 90.0, //will be computed in initialize()
            lookfrom: Point3::new(0.0,0.0,0.0),
            lookat: Point3::new(0.0,0.0,0.0),
            vup: Vec3::new(0.0,0.0,0.0),
            defocus_angle: 0.0,
            focus_dist: 0.0,
            pixel_sample_scale: 1.0,
            image_height: 0, // will be computed in initialize()
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera{

    pub fn new_with(image_width: usize, aspect_ratio: f64, samples_per_pixel: usize, max_depth: usize, vfov: f64) -> Self {
        let mut c = Self::default();
        c.image_width = image_width;
        c.aspect_ratio = aspect_ratio;
        c.samples_per_pixel = samples_per_pixel;
        c.max_depth = max_depth;
        c.vfov = vfov;
        // c.lookfrom = lookfrom;
        // c.lookat = lookat;
        // c.vup = vup;
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

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // let focal_length = (self.lookfrom - self.lookat).length();
        let theta: f64 = degrees_to_radians(self.vfov);
        let h: f64 = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 = viewport_height * ((self.image_width as f64) / (self.image_height as f64));
        // let camera_center = Point3::new(0.0, 0.0, 0.0);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = (self.vup.cross(&self.w)).unit_vector();
        self.v = self.w.cross(&self.u);


        // let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        // let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / (self.image_width - 1) as f64;
        self.pixel_delta_v = viewport_v / (self.image_height - 1) as f64;

        // let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        Ok(())

    }

        pub fn render_multithreaded(&mut self, world: &HittableList) -> io::Result<()> {
            self.initialize().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

            let file = File::create("image.ppm")?;
            let mut out = BufWriter::new(file);
            writeln!(out, "P3")?;
            writeln!(out, "{} {}", self.image_width, self.image_height)?;
            writeln!(out, "255")?;

            // copy small hot values into locals so parallel closure captures cheap copies
            let image_w = self.image_width;
            let image_h = self.image_height;
            let samples = self.samples_per_pixel;
            let sample_scale = self.pixel_sample_scale;
            let center = self.center;
            let pixel00 = self.pixel00_loc;
            let du = self.pixel_delta_u;
            let dv = self.pixel_delta_v;
            let max_depth = self.max_depth;


            let start = Instant::now();
            // compute each scanline in parallel (coarse-grain)
            let rows: Vec<String> = (0..image_h)
                .into_par_iter()
                .map(|j| {
                    let mut row_buf = String::with_capacity(image_w * 12);
                    for i in 0..image_w {
                        // per-pixel: do samples sequentially (avoids tiny rayon tasks)
                        let sum = (0..samples).fold(Color::new(0.0, 0.0, 0.0), |acc, _| {
                            let r = self.get_ray(center, pixel00, du, dv, i, j);
                            acc + Self::ray_color(r, max_depth, world)
                        });

                        let pixel = sum * sample_scale;
                        // convert to integer RGB components
                        let ppm_string = pixel.to_ppm_string();
                        row_buf.push_str(&ppm_string);

                    }
                    row_buf
                })
                .collect();

            // write rows sequentially to avoid IO contention
            for row in rows {
                out.write_all(row.as_bytes())?;
            }

            out.flush()?;
            eprintln!("Wrote image.ppm ({}x{}) {:?}", image_w, image_h, start.elapsed());
            Ok(())
        }


    pub fn render(&mut self, world: &HittableList) -> io::Result<()> {
        let mut scanline_times: Vec<std::time::Duration> = Vec::with_capacity(self.image_height);

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
            let start = Instant::now();
            for i in 0..self.image_width {

                let mut pixel_color = Color::new(0.0,0.0,0.0);

                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(self.center, self.pixel00_loc, self.pixel_delta_u, self.pixel_delta_v, i, j);
                    pixel_color += Self::ray_color(r, self.max_depth, &world);
                }

                pixel_color = pixel_color * self.pixel_sample_scale;

                // let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                // let ray_direction = pixel_center - self.center;

                // let ray = Ray::new(self.center, ray_direction);

                // // let r = i as f64 / (image_width - 1) as f64;
                // // let g = j as f64 / (image_height - 1) as f64;
                // // let b = 0.5;
                // let pixel_color = Self::ray_color(ray, &world);
                // pixel_color.write_ppm(&mut out)?;
                let ppm_string = pixel_color.to_ppm_string();
                out.write_all(ppm_string.as_bytes())?;
            }
            scanline_times.push(start.elapsed());
        }


        out.flush()?;



        writeln!(err)?;
        eprintln!("Wrote image.ppm ({}x{})", self.image_width, self.image_height);
        let total: std::time::Duration = scanline_times.iter().copied().sum();
       let avg = if !scanline_times.is_empty() {
           total / (scanline_times.len() as u32)
       } else {
           std::time::Duration::ZERO
       };
       eprintln!("mrender: wrote image.ppm ({}x{}). total={:?} avg_per_scanline={:?}", self.image_width, self.image_height, total, avg);
        Ok(())
    }

    fn get_ray(&self, center: Point3, pixel00: Point3, pixel_delta_u: Vec3, pixel_delta_v: Vec3,i: usize, j: usize) -> Ray {
        let offset = Self::sample_square();

        let pixel_sample = pixel00 + ((i as f64 + offset.x) * pixel_delta_u) + ((j as f64 + offset.y) * pixel_delta_v);

        // let ray_origin = center;
        let ray_origin= match self.defocus_angle {
            x if x <= 0.0 => center,
            _ => self.defocus_disk_sample(),
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)

    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        return self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v);
    }

    fn ray_color(r: Ray, depth: usize, world: &HittableList) -> Color {

        if depth == 0 { return Color::new(0.0,0.0,0.0)}

        // if let Some(rec) = world.hit(&r, 0.001, INFINITY_F64) {
        //     // shade by normal (rec.normal is a Vec3)
        //     // let shaded = 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
        //     // return Color::from(shaded);

        //     let direction = rec.normal + Vec3::random_on_hemisphere(&rec.normal);
        //     return Camera::ray_color(Ray::new(rec.p, direction), depth - 1, &world) * 0.1
        // }

        if let Some(rec) = world.hit(&r, 0.001, INFINITY_F64) {
            // Try to scatter via the material on the hit record.
            // `scatter` should return Some((attenuation_color, scattered_ray)) or None.
            if let Some((attenuation, scattered)) = rec.mat.scatter(&r, &rec) {
                let scattered_color = Self::ray_color(scattered, depth - 1, world);
                // component-wise multiply attenuation * scattered_color
                // return Color::new(
                //     attenuation.0.x * scattered_color.0.x,
                //     attenuation.0.y * scattered_color.0.y,
                //     attenuation.0.z * scattered_color.0.z,
                // );
                return attenuation * scattered_color;
            }
            // material absorbed the ray
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction: Vec3 = r.direction.unit_vector();
        let a: f64 = 0.5 * (unit_direction.y + 1.0);
        let c: Vec3 = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0)
            + a * Vec3::new(0.5, 0.7, 1.0);
        Color::from(c)
    }
}