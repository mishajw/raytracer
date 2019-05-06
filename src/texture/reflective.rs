use std::marker::PhantomData;

use crate::get_color_for_ray;
use crate::math;
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
        ray: &Ray,
        position: Vec3,
        tracer: &Tracer,
        world: &World,
    ) -> Color
    {
        let normal = shape.get_normal(position);
        let reflection = math::reflect(ray.direction, normal);
        let ray = Ray::new(position, reflection);
        get_color_for_ray(&ray, &tracer, &world)
    }
}
