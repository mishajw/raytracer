//! Shapes that are contained in the scene

mod sphere;

pub use sphere::Sphere;

use crate::texture::Texture;
use crate::Object;
use crate::Ray;
use crate::Renderable;

/// Shape of an object in the scene
pub trait Shape: Sized + 'static {
    /// Check if a ray collides with this shape
    ///
    /// If it does collide, return the scalar multiple of `ray.direction` where
    /// the ray meets the shape.
    fn get_collision(&self, ray: &Ray) -> Option<f64>;

    /// Make the shape renderable by giving it textures
    fn with_textures(
        self,
        textures: Vec<Box<Texture<Self>>>,
    ) -> Box<Renderable>
    {
        Box::new(Object::new(self, textures))
    }
}
