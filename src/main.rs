use std::io::{self, Write};

mod hittable;
mod ray;
mod vec3;
use hittable::{Hittable, Sphere};
use ray::Ray;
use vec3::{Color, Point};

fn ray_color(r: Ray) -> Color {
    let sphere = Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    match sphere.hit(r, 0.0, f64::INFINITY) {
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

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Point::new(viewport_width, 0.0, 0.0);
    let vertical = Point::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Point::new(0.0, 0.0, focal_length);

    // Render
    let stdout = io::stdout();
    let mut stdout_hdl = stdout.lock();
    let stderr = io::stderr();
    let mut stderr_hdl = stderr.lock();

    let line = format!("P3\n{} {} \n255\n", image_width, image_height);
    stdout_hdl.write(&line.into_bytes())?;

    for j in (0..image_height).rev() {
        let msg = format!("Scanlines remaining: {}\n", j);
        stderr_hdl.write(&msg.into_bytes())?;
        stderr_hdl.flush()?;

        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let ray = Ray {
                origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let color = ray_color(ray);
            color.write_color(&mut stdout_hdl)?;
        }
    }

    stdout_hdl.flush()?;
    stderr_hdl.write(b"Done\n")?;
    stderr_hdl.flush()?;

    Ok(())
}
