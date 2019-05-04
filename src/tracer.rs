use crate::Ray;
use crate::Shape;
use crate::Vec3;
use crate::World;

/// Traces a ray through the world and finds what collides with it
// TODO: Extract to interface so we can have different implementations of ray
// tracing
pub struct Tracer<'world> {
    world: &'world World<'world>,
}

pub struct TraceResult<'shape> {
    /// The shape that the ray collided with
    pub shape: &'shape Shape,
    /// Where on the surface of the shape the collision happened
    pub collision_position: Vec3,
}

impl<'world> Tracer<'world> {
    #[allow(missing_docs)]
    pub fn new(world: &'world World<'world>) -> Tracer { Tracer { world } }

    /// Trace the ray and get what object (if any) it collides with in the world
    pub fn trace(&self, _ray: Ray) -> Option<TraceResult<'world>> { None }
}
