//! Toy ray tracer.
//!
//! ## Usage
//!
//! The ray tracer takes a [`World`] object, which contains the [`Object`]s and
//! [`Light`]s in a in the scene, as well as a single [`Camera`] that defines
//! the perspective of the rendered image.
//!
//! For example:
//!
//! ```rust
//! # use std::f64;
//! # use raytracer::render;
//! # use raytracer::shape::{Shape, Sphere};
//! # use raytracer::texture::{Diffuse, Solid, Texture};
//! # use raytracer::{Camera, Color, Light, Vec3, World};
//!
//! let shapes = vec![Sphere::new(Vec3::new(0, 1, 0), 0.25)
//!     .with_textures(vec![Solid::new(Color::red()), Diffuse::new()])];
//! let camera =
//!     Camera::new(Vec3::new(0, 0, 0), Vec3::new(0.001, 1, 0.001).unit());
//! let lights = vec![Light::new(Vec3::new(-1, 1, 1), 0.7)];
//! let world = World::new(camera, shapes, vec![], Color::black());
//!
//! // Set the image width, image height, and field of view
//! let image = render(&world, 800, 600, f64::consts::PI / 3.0);
//! ```
//!
//! ## Algorithm
//!
//! For every pixel in the output image:
//! 1. Calculate [`Ray`] from the camera to the pixel location on
//!    the image plane (done in [`render`]).
//! 2. Find the first object that intersects with the ray using the [`tracer`],
//!    which checks intersections using [`Shape::get_collision`].
//! 3. Get the color of the object at that point using [`Texture::get_color`].
//!    a. The texture can use the [`tracer`] to cast more rays if need (for
//!       example, for reflection).
//! 4. Add the texture's color to the image.
//!
//! [`World`]: struct.World.html
//! [`Object`]: struct.Object.html
//! [`Light`]: struct.Light.html
//! [`Camera`]: struct.Camera.html
//! [`Ray`]: struct.Ray.html
//! [`render`]: fn.render.html
//! [`tracer`]: struct.Tracer.html
//! [`Shape::get_collision`]: shape/trait.Shape.html#tymethod.get_collision
//! [`Texture::get_color`]: texture/trait.Texture.html#tymethod.get_color

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
pub use tracer::get_color_for_ray;
pub use tracer::Tracer;
pub use vec3::Vec3;
pub use world::World;
