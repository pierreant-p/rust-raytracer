// TODO(PAP): replace by the f64 clamp method when it's stable
// https://doc.rust-lang.org/std/primitive.f64.html#method.clamp
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    assert!(min <= max);
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(-3.0, -2.0, 1.0), -2.0);
        assert_eq!(clamp(-2.0, -2.0, 1.0), -2.0);
        assert_eq!(clamp(0.5, -2.0, 1.0), 0.5);
        assert_eq!(clamp(1.0, -2.0, 1.0), 1.0);
        assert_eq!(clamp(3.0, -2.0, 1.0), 1.0);
    }
}
