use std::marker::PhantomData;

use crate::shape::Shape;
use crate::shape::SurfaceNormal;
use crate::texture::Texture;
use crate::Color;
use crate::Ray;
use crate::Tracer;
use crate::Vec3;
use crate::World;

/// Reflective texture, that reflects perfectly off its surface
pub struct Reflective<ShapeT: Shape + SurfaceNormal> {
    phantom: PhantomData<ShapeT>,
}

impl<ShapeT: Shape + SurfaceNormal> Reflective<ShapeT> {
    #[allow(missing_docs)]
    pub fn new() -> Box<Self> {
        Box::new(Reflective {
            phantom: PhantomData,
        })
    }
}

impl<ShapeT: Shape + SurfaceNormal> Texture<ShapeT> for Reflective<ShapeT> {
    fn get_color(
        &self,
        shape: &ShapeT,
        _ray: &Ray,
        position: Vec3,
        tracer: &Tracer,
        world: &World,
    ) -> Color
    {
        // TODO: Use reflective angle instead of normal
        let normal = shape.get_normal(position);
        // TODO: Share this common code in `render.rs`
        let ray = Ray::new(position, normal);
        let trace_result = tracer.trace(&ray);
        match trace_result {
            // If it does collide, ask the collided shape what color to draw
            Some(result) => result.renderable.get_color(
                &ray,
                result.collision_position,
                &tracer,
                &world,
            ),
            // If it doesn't collide, use the background color
            None => world.background_color,
        }
    }
}
