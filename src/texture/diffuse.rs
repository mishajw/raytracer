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
        _tracer: &Tracer,
        world: &World,
    ) -> Color
    {
        let normal = shape.get_normal(position);
        let total_intensity: f64 = world
            .lights
            .iter()
            .map(|l| {
                let relative_light = l.position - position;
                let angle = math::angle(normal, relative_light.unit());
                (angle / f64::consts::PI) * l.intensity
            })
            .sum();
        let total_intensity = total_intensity.min(1.0);
        let gray_color = (total_intensity * 255.0) as u8;
        Color::new(gray_color, gray_color, gray_color)
    }
}
