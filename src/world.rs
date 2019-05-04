use crate::Camera;
use crate::Shape;

/// The scene to be rendered
pub struct World<'shapes> {
    /// The camera we look out of in the scene
    pub camera: Camera,
    /// All the shapes in the scene
    pub shapes: &'shapes [Shape],
}

impl<'shapes> World<'shapes> {
    #[allow(missing_docs)]
    pub fn new(camera: Camera, shapes: &'shapes [Shape]) -> World {
        World { camera, shapes }
    }
}
