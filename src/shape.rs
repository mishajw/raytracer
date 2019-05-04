use crate::Color;
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
}

enum ShapeType {
    Circle { centre: Vec3, radius: f64 },
}

enum TextureType {
    Diffuse,
}
