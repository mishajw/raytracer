use crate::Vec3;

/// Subtract `POINT_BIAS` from a the ray when calculating where an intersection
/// happened
///
/// This helps to remove "shadow acne".
const POINT_BIAS: f64 = 1e-10;

/// A line in space with a set start point, but no end point
pub struct Ray {
    /// Starting position of the ray
    pub position: Vec3,
    /// Unit vector of the direction of the ray
    pub direction: Vec3,
}

impl Ray {
    #[allow(missing_docs)]
    pub fn new(position: Vec3, direction: Vec3) -> Ray {
        assert!(direction.is_unit());
        Ray {
            position,
            direction,
        }
    }

    /// Get the point on the ray given by a scalar multiple of the direction
    pub fn point(&self, scalar: f64) -> Vec3 {
        // We shouldn't have to back-trace rays anywhere
        assert!(scalar >= 0.0);
        self.position + self.direction * (scalar + POINT_BIAS)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        let point = Ray::new(Vec3::new(1, 0, 2), Vec3::new(0, 1, 0)).point(5.0);
        assert!((point.x - 1.0).abs() < 1e-6);
        assert!((point.y - 5.0).abs() < 1e-6);
        assert!((point.z - 2.0).abs() < 1e-6);
    }
}
