use std::marker::PhantomData;

use crate::shape::Shape;
use crate::shape::SurfaceNormal;
use crate::texture::Texture;
use crate::Color;
use crate::Ray;
use crate::Tracer;
use crate::Vec3;

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
        _shape: &ShapeT,
        _ray: &Ray,
        _position: Vec3,
        _tracer: &Tracer,
    ) -> Color
    {
        unimplemented!();
    }
}
