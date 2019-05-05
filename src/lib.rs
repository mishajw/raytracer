//! Toy raytracer

#![warn(missing_docs)]

mod camera;
mod color;
mod image;
mod light;
pub mod math;
mod object;
mod ray;
mod render;
pub mod shape;
pub mod texture;
mod tracer;
mod vec3;
mod world;

pub use crate::image::Image;
pub use camera::Camera;
pub use color::Color;
pub use light::Light;
pub use object::Object;
pub use object::Renderable;
pub use ray::Ray;
pub use render::render;
pub use tracer::Tracer;
pub use vec3::Vec3;
pub use world::World;
