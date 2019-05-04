use crate::math;
use crate::Color;
use crate::Ray;
use crate::Tracer;
use crate::Vec3;

/// A shape in the scene
pub struct Shape {
    shape_type: ShapeType,
    _texture_type: TextureType,
    color: Color,
}

impl Shape {
    #[allow(missing_docs)]
    pub fn circle(centre: Vec3, radius: f64, color: Color) -> Shape {
        Shape {
            shape_type: ShapeType::Circle { centre, radius },
            _texture_type: TextureType::Diffuse,
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
        // TODO: Implement more complex rendering based on `texture_type`
        self.color
    }

    /// Check if a ray collides with this shape
    ///
    /// If it does collide, return the scalar multiple of `ray.direction` where
    /// the ray meets the shape.
    pub fn get_collision(&self, ray: &Ray) -> Option<f64> {
        match self.shape_type {
            ShapeType::Circle { centre, radius } => {
                // Make the ray's centre the origin to simplify
                let c = centre - ray.position;
                let d = ray.direction;
                let a = d.x.powf(2.0) + d.y.powf(2.0) + d.z.powf(2.0);
                let b =
                    (2.0 * d.x * c.x) + (2.0 * d.y * c.y) + (2.0 * d.z * c.z);
                let c = c.x.powf(2.0) + c.y.powf(2.0) + c.z.powf(2.0);
                match math::factorize(a, b, c - radius.powf(2.0)) {
                    Some((x, y)) => Some(x.min(y)),
                    None => None,
                }
            }
        }
    }
}

enum ShapeType {
    Circle { centre: Vec3, radius: f64 },
}

enum TextureType {
    Diffuse,
}
