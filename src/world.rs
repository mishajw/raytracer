use crate::Camera;
use crate::Shape;

/// The scene to be rendered
pub struct World<'shapes> {
    camera: Camera,
    shapes: &'shapes [Shape],
}

impl<'shapes> World<'shapes> {
    #[allow(missing_docs)]
    pub fn new(camera: Camera, shapes: &'shapes [Shape]) -> World {
        World { camera, shapes }
    }
}
