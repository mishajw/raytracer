//! Toy raytracer

#![warn(missing_docs)]

extern crate image;

mod camera;
mod color;
pub mod math;
mod ray;
mod render;
mod shape;
mod tracer;
mod vec3;
mod world;

pub use camera::Camera;
pub use color::Color;
pub use ray::Ray;
pub use render::render;
pub use shape::Shape;
pub use tracer::Tracer;
pub use vec3::Vec3;
pub use world::World;
