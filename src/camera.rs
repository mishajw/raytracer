use crate::Vec3;

/// The camera in the scene
pub struct Camera {
    /// Position of the camera
    pub position: Vec3,
    /// Unit vector of the direction of the camera
    pub direction: Vec3,
}

impl Camera {
    #[allow(missing_docs)]
    pub fn new(position: Vec3, direction: Vec3) -> Camera {
        assert!(direction.is_unit());
        Self {
            position,
            direction,
        }
    }
}
