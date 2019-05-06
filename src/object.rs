use crate::shape::Shape;
use crate::texture::Texture;
use crate::Color;
use crate::Ray;
use crate::Tracer;
use crate::Vec3;
use crate::World;

/// Can be rendered to the scene
///
/// The only implementation of this is `Object`, but we have to separate the
/// trait from the implementation. This is because we can't store a list of
/// `Object<T>` in `World` as `T` changes from item to item. Therefore, we
/// introduce `Renderable` so that `World` can hold a list of `Object<T>`/
/// `Renderable`.
///
/// TODO: Is there a better work around?
pub trait Renderable {
    /// Get the color that a `ray` would see if it hit the shape at `position`
    ///
    /// `tracer` is passed so that new rays can be cast.
    fn get_color(
        &self,
        ray: &Ray,
        position: Vec3,
        tracer: &Tracer,
        world: &World,
    ) -> Color;

    /// Check if a ray collides with this shape
    ///
    /// If it does collide, return the scalar multiple of `ray.direction` where
    /// the ray meets the shape.
    fn get_collision(&self, ray: &Ray) -> Option<f64>;
}

/// An object in the scene
pub struct Object<ShapeT: Shape> {
    shape: ShapeT,
    textures: Vec<Box<Texture<ShapeT>>>,
}

impl<ShapeT: Shape> Object<ShapeT> {
    #[allow(missing_docs)]
    pub fn new(shape: ShapeT, textures: Vec<Box<Texture<ShapeT>>>) -> Self {
        Object { shape, textures }
    }
}

impl<ShapeT: Shape> Renderable for Object<ShapeT> {
    fn get_color(
        &self,
        ray: &Ray,
        position: Vec3,
        tracer: &Tracer,
        world: &World,
    ) -> Color
    {
        Color::merge(
            &self
                .textures
                .iter()
                .map(|t| t.get_color(&self.shape, ray, position, tracer, world))
                .collect::<Vec<_>>(),
        )
    }

    fn get_collision(&self, ray: &Ray) -> Option<f64> {
        self.shape.get_collision(ray)
    }
}
