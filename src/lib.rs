//! Toy raytracer

#![warn(missing_docs)]

mod camera;
mod color;
mod render;
mod shape;
mod vec3;
mod world;

pub use camera::Camera;
pub use color::Color;
pub use render::render;
pub use shape::Shape;
pub use vec3::Vec3;
pub use world::World;
