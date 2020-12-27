use std::io::{self, Write};

mod vec3;
use vec3::Vec3;

fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

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
            let color = Vec3::new(
                i as f64 / (image_width as f64 - 1.0),
                j as f64 / (image_height as f64 - 1.0),
                0.25,
            );
            color.write_color(&mut stdout_hdl)?;
        }
    }

    stdout_hdl.flush()?;
    stderr_hdl.write(b"Done\n")?;
    stderr_hdl.flush()?;

    Ok(())
}
