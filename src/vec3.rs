use crate::utils::clamp;
use rand::Rng;
use std::io::{self, Write};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let v = loop {
            let v = Vec3::random_range(-1.0, 1.0);
            if v.length_squared() < 1.0 {
                break v;
            }
        };

        v
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn write_color(
        &self,
        stdout: &mut io::StdoutLock,
        samples_per_pixel: i32,
    ) -> io::Result<()> {
        // Divide the color by the number of samples
        let scale = 1.0 / (samples_per_pixel as f64);

        // sqrt is for gamma=2.0 gamma-correction
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
        let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
        let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

        let line = format!("{} {} {}\n", ir, ig, ib);

        stdout.write(&line.into_bytes())?;

        Ok(())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Vec3 {
        Vec3 {
            x: k * self.x,
            y: k * self.y,
            z: k * self.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, k: f64) -> Vec3 {
        Vec3 {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}

pub type Color = Vec3;
pub type Point = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 5.0);
        assert_eq!(v3.y, 7.0);
        assert_eq!(v3.z, 9.0);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1.x, 5.0);
        assert_eq!(v1.y, 7.0);
        assert_eq!(v1.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 1.0, 4.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, -3.0);
        assert_eq!(v3.y, 1.0);
        assert_eq!(v3.z, -1.0);
    }

    #[test]
    fn test_neg() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v3 = -v1;
        assert_eq!(v3.x, -1.0);
        assert_eq!(v3.y, -2.0);
        assert_eq!(v3.z, -3.0);
    }

    #[test]
    fn test_mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let k = 3.0;
        let v3 = v1 * k;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 9.0);
    }

    #[test]
    fn test_mul_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let k = 3.0;
        let v3 = k * v1;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 9.0);
    }

    #[test]
    fn test_div_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let k = 2.0;
        let v3 = v1 / k;
        assert_eq!(v3.x, 0.5);
        assert_eq!(v3.y, 1.0);
        assert_eq!(v3.z, 1.5);
    }

    #[test]
    fn test_length_squared() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length_squared(), 14.0);
    }

    #[test]
    fn test_length() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let len_squared = v1.length_squared();
        assert_eq!(v1.length(), len_squared.sqrt());
    }

    #[test]
    fn test_unit_vector() {
        let v1 = Vec3::new(2.0, 0.0, 0.0);
        let v2 = v1.unit_vector();
        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 0.0);
        assert_eq!(v2.z, 0.0);
    }

    #[test]
    fn test_random() {
        let v1 = Vec3::random();
        assert!(0.0 <= v1.x && v1.x < 1.0);
        assert!(0.0 <= v1.y && v1.y < 1.0);
        assert!(0.0 <= v1.z && v1.z < 1.0);
    }

    #[test]
    fn test_random_range() {
        let v1 = Vec3::random_range(-2.0, 3.0);
        assert!(-2.0 <= v1.x && v1.x < 3.0);
        assert!(-2.0 <= v1.y && v1.y < 3.0);
        assert!(-2.0 <= v1.z && v1.z < 3.0);
    }

    #[test]
    fn test_random_in_unit_sphere() {
        let v1 = Vec3::random_in_unit_sphere();
        assert!(v1.length_squared() < 1.0);
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 2.0, -3.0);
        let v2 = Vec3::new(4.0, -5.0, 6.0);
        assert_eq!(v1.dot(&v2), -24.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 2.0, -3.0);
        let v2 = Vec3::new(4.0, -5.0, 6.0);
        let v3 = v1.cross(&v2);
        assert_eq!(v3.x, -3.0);
        assert_eq!(v3.y, -18.0);
        assert_eq!(v3.z, -13.0);
    }
}
