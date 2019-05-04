use crate::Color;
use crate::Ray;
use crate::Tracer;
use crate::Vec3;

/// A shape in the scene
pub struct Shape {
    shape_type: ShapeType,
    texture_type: TextureType,
    color: Color,
}

impl Shape {
    #[allow(missing_docs)]
    pub fn circle(centre: Vec3, radius: f64, color: Color) -> Shape {
        Shape {
            shape_type: ShapeType::Circle { centre, radius },
            texture_type: TextureType::Diffuse,
            color,
        }
    }

    /// Get the color that a `ray` would see if it hit the shape at `position`
    ///
    /// `tracer` is passed so that new rays can be cast.
    pub fn get_color(
        &self,
        _ray: &Ray,
        _position: Vec3,
        _tracer: &Tracer,
    ) -> Color
    {
        self.color
    }

    /// Check if a ray collides with this shape
    ///
    /// If it does collide, return the scalar multiple of `ray.direction` where
    /// the ray meets the shape.
    pub fn get_collision(&self, ray: &Ray) -> Option<f64> {
        // TODO: Implement
        None
    }
}

enum ShapeType {
    Circle { centre: Vec3, radius: f64 },
}

enum TextureType {
    Diffuse,
}
