use crate::Vec3;

/// Point light in a scene
#[derive(Debug)]
pub struct Light {
    /// Position of the light
    pub position: Vec3,
    /// Intensity of the light between 0-1
    pub intensity: f64,
}

impl Light {
    #[allow(missing_docs)]
    pub fn new(position: Vec3, intensity: f64) -> Light {
        assert!(0.0 <= intensity && intensity <= 1.0);
        Light {
            position,
            intensity,
        }
    }
}
