use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_at() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let dir = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray { origin, dir };
        let t = 2.0;
        let point = ray.point_at(t);
        assert_eq!(point.x, 9.0);
        assert_eq!(point.y, 12.0);
        assert_eq!(point.z, 15.0);
    }
}
