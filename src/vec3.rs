use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

/// Vector with 3 dimensions
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[allow(missing_docs)]
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(
        x: X,
        y: Y,
        z: Z,
    ) -> Self
    {
        let x = x.into();
        let y = y.into();
        let z = z.into();
        assert!(x.is_finite());
        assert!(y.is_finite());
        assert!(z.is_finite());
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// Check if the vector has unit length
    pub fn is_unit(&self) -> bool { (self.magnitude() - 1.0).abs() < 0.0001 }

    /// Get the unit vector in the same direction as this vector
    pub fn unit(&self) -> Vec3 {
        let magnitude = self.magnitude();
        Vec3::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
