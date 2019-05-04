extern crate raytracer;

use raytracer::Camera;
use raytracer::Color;
use raytracer::Shape;
use raytracer::Vec3;
use raytracer::World;

fn main() {
    let shapes = [
        Shape::circle(Vec3::new(0, 1, -0.5), 0.5, Color::red()),
        Shape::circle(Vec3::new(0, 1, 0), 0.5, Color::green()),
        Shape::circle(Vec3::new(0, 1, 0.5), 0.5, Color::blue()),
    ];

    let camera = Camera::new(Vec3::new(0, 0, 0), Vec3::new(0, 1, 0));

    let world = World::new(camera, &shapes);

    raytracer::render(world, 600, 400, 1.0);
}
