use crate::Camera;
use crate::Color;
use crate::Shape;

/// The scene to be rendered
pub struct World {
    /// The camera we look out of in the scene
    pub camera: Camera,
    /// All the shapes in the scene
    pub shapes: Vec<Shape>,
    /// The color of the background when rendering the scene
    pub background_color: Color,
}

impl World {
    #[allow(missing_docs)]
    pub fn new(
        camera: Camera,
        shapes: Vec<Shape>,
        background_color: Color,
    ) -> World
    {
        World {
            camera,
            shapes,
            background_color,
        }
    }
}
