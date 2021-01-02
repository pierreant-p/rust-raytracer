use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::io::{self, Write};

mod camera;
mod hittable;
mod ray;
mod utils;
mod vec3;

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::{Color, Point};

fn ray_color(r: Ray, world: &HittableList) -> Color {
    match world.hit(r, 0.0, f64::INFINITY) {
        Some(hit_record) => {
            let normal = hit_record.normal;

            0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
        }
        None => {
            let unit_direction = r.dir.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 10;

    // World
    let mut world = HittableList::new();
    let sphere1 = Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    world.objects.push(Box::new(sphere1));
    let sphere2 = Sphere {
        center: Point::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    world.objects.push(Box::new(sphere2));

    // Camera
    let camera = Camera::new();

    // Random
    let mut rng = rand::thread_rng();

    // Progress bar
    let nb_steps = image_width * image_height * samples_per_pixel;
    let progress = ProgressBar::new(nb_steps as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} samples (ETA: {eta})",
            )
            .progress_chars("#>-"),
    );

    // Render
    let stdout = io::stdout();
    let mut stdout_hdl = stdout.lock();
    let stderr = io::stderr();
    let mut stderr_hdl = stderr.lock();

    let line = format!("P3\n{} {} \n255\n", image_width, image_height);
    stdout_hdl.write(&line.into_bytes())?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let randu: f64 = rng.gen();
                let randv: f64 = rng.gen();
                let u = (i as f64 + randu) / (image_width as f64 - 1.0);
                let v = (j as f64 + randv) / (image_height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                let sample_color = ray_color(ray, &world);
                color += sample_color;
                progress.inc(1);
            }
            color.write_color(&mut stdout_hdl, samples_per_pixel)?;
        }
    }

    stdout_hdl.flush()?;

    progress.finish();

    stderr_hdl.write(b"Done\n")?;
    stderr_hdl.flush()?;

    Ok(())
}
