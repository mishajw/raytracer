use crate::shape::Shape;
use crate::texture::Texture;
use crate::Color;
use crate::Ray;
use crate::Tracer;
use crate::Vec3;

/// Solid texture, showing a single color
pub struct Solid {
    color: Color,
}

impl Solid {
    #[allow(missing_docs)]
    pub fn new(color: Color) -> Box<Solid> { Box::new(Solid { color }) }
}

impl<ShapeT: Shape> Texture<ShapeT> for Solid {
    fn get_color(
        &self,
        _shape: &ShapeT,
        _ray: &Ray,
        _position: Vec3,
        _tracer: &Tracer,
    ) -> Color
    {
        self.color
    }
}
