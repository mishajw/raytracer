use std::f64;
use std::marker::PhantomData;

use crate::math;
use crate::shape::Shape;
use crate::shape::SurfaceNormal;
use crate::texture::Texture;
use crate::Color;
use crate::Ray;
use crate::Tracer;
use crate::Vec3;
use crate::World;

/// Diffuse texture, colored by a light
pub struct Diffuse<ShapeT: Shape + SurfaceNormal> {
    phantom: PhantomData<ShapeT>,
}

impl<ShapeT: Shape + SurfaceNormal> Diffuse<ShapeT> {
    #[allow(missing_docs)]
    pub fn new() -> Box<Self> {
        Box::new(Diffuse {
            phantom: PhantomData,
        })
    }
}

impl<ShapeT: Shape + SurfaceNormal> Texture<ShapeT> for Diffuse<ShapeT> {
    fn get_color(
        &self,
        shape: &ShapeT,
        _ray: &Ray,
        position: Vec3,
        tracer: &Tracer,
        world: &World,
    ) -> Color
    {
        let normal = shape.get_normal(position);
        let total_intensity: f64 = world
            .lights
            .iter()
            .filter_map(|l| {
                let relative_light = l.position - position;
                let light_ray = Ray::new(position, relative_light.unit());
                if tracer.trace(&light_ray).is_some() {
                    return None;
                }
                let angle = math::angle(normal, relative_light.unit());
                Some((1.0 - angle / f64::consts::PI) * l.intensity)
            })
            .sum();
        let total_intensity = total_intensity.min(1.0);
        Color::new(total_intensity, total_intensity, total_intensity)
    }
}
