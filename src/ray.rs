use crate::Vec3;

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
        self.position + self.direction * scalar
    }
}
