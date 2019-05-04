use crate::Vec3;

/// The camera in the scene
pub struct Camera {
    position: Vec3,
    direction: Vec3,
}

impl Camera {
    #[allow(missing_docs)]
    pub fn new(position: Vec3, direction: Vec3) -> Camera {
        Self {
            position,
            direction,
        }
    }
}
