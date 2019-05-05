//! Textures that can draw shapes in different ways

mod diffuse;
mod solid;

pub use diffuse::Diffuse;
pub use solid::Solid;

use crate::shape::Shape;
use crate::Color;
use crate::Ray;
use crate::Tracer;
use crate::Vec3;
use crate::World;

/// Defines how a shape should appear when rendering
pub trait Texture<ShapeT: Shape> {
    /// Get the color that a `ray` would see if it hit the shape at `position`
    ///
    /// `tracer` is passed so that new rays can be cast.
    fn get_color(
        &self,
        shape: &ShapeT,
        ray: &Ray,
        position: Vec3,
        tracer: &Tracer,
        world: &World,
    ) -> Color;
}
