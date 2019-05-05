use crate::Camera;
use crate::Color;
use crate::Light;
use crate::Renderable;

/// The scene to be rendered
pub struct World {
    /// The camera we look out of in the scene
    pub camera: Camera,
    /// All the shapes in the scene
    pub renderables: Vec<Box<Renderable>>,
    /// All the lights in the scene
    pub lights: Vec<Light>,
    /// The color of the background when rendering the scene
    pub background_color: Color,
}

impl World {
    #[allow(missing_docs)]
    pub fn new(
        camera: Camera,
        renderables: Vec<Box<Renderable>>,
        lights: Vec<Light>,
        background_color: Color,
    ) -> World
    {
        World {
            camera,
            renderables,
            lights,
            background_color,
        }
    }
}
