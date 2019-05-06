use crate::Color;
use crate::Ray;
use crate::Renderable;
use crate::Vec3;
use crate::World;

/// Traces a ray through the world and finds what collides with it
// TODO: Extract to interface so we can have different implementations of ray
// tracing
pub struct Tracer<'world> {
    world: &'world World,
}

pub struct TraceResult<'shape> {
    /// The shape that the ray collided with
    pub renderable: &'shape Renderable,
    /// Where on the surface of the shape the collision happened
    pub collision_position: Vec3,
}

impl<'world> Tracer<'world> {
    #[allow(missing_docs)]
    pub fn new(world: &World) -> Tracer { Tracer { world } }

    /// Trace the ray and get what object (if any) it collides with in the world
    pub fn trace(&self, ray: &Ray) -> Option<TraceResult<'world>> {
        self.world
            .renderables
            .iter()
            // Get pair of `(shape, ray scalar)`
            .flat_map(|shape| {
                shape
                    .get_collision(&ray)
                    .into_iter()
                    .map(|scalar| (shape, scalar))
                    .collect::<Vec<_>>()
            })
            // Negative values are behind the start of the ray, so ignore them
            .filter(|(_, s)| *s >= 0.0)
            // Get the lowest by `ray scalar`
            .min_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap())
            // Map to a `TraceResult`
            .map(|(renderable, scalar)| TraceResult {
                renderable: &**renderable,
                collision_position: ray.point(scalar),
            })
    }
}

/// Get the color that a ray would hit using a tracer
pub fn get_color_for_ray(ray: &Ray, tracer: &Tracer, world: &World) -> Color {
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
