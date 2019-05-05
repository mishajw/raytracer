extern crate image;
extern crate raytracer;

mod util;

use std::path::Path;

use raytracer::Camera;
use raytracer::Color;
use raytracer::Shape;
use raytracer::Vec3;
use raytracer::World;

fn main() {
    let shapes = [
        Shape::sphere(Vec3::new(-0.5, 1, 0), 0.25, Color::red()),
        Shape::sphere(Vec3::new(0, 1, 0), 0.25, Color::green()),
        Shape::sphere(Vec3::new(0.5, 1, 0), 0.25, Color::blue()),
    ];
    let camera =
        Camera::new(Vec3::new(0, 0, 0), Vec3::new(0.001, 1, 0.001).unit());
    let world = World::new(camera, &shapes, Color::black());
    let image = raytracer::render(world, 600, 400, 1.0);
    util::save_image(image, Path::new("examples-output/simple.png"));
}
