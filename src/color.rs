use std::io::{self, Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write(&self, stdout: &mut io::StdoutLock) -> io::Result<()> {
        let ir = (255.99 * self.x) as i32;
        let ig = (255.99 * self.y) as i32;
        let ib = (255.99 * self.z) as i32;

        let line = format!("{} {} {}\n", ir, ig, ib);

        stdout.write(&line.into_bytes())?;

        Ok(())
    }
}
