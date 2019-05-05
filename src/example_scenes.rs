#[macro_use]
extern crate criterion;
extern crate image;
extern crate raytracer;

use std::fs;
use std::path::Path;

use criterion::Criterion;
use image::ImageBuffer;
use image::Rgb;

use raytracer::Camera;
use raytracer::Color;
use raytracer::Image;
use raytracer::Shape;
use raytracer::Vec3;
use raytracer::World;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

fn run(bench: &mut Criterion) {
    // Write the images to the example-output directory
    simple(|world| {
        let image = raytracer::render(&world, WIDTH, HEIGHT, 1.0);
        save_image(image, Path::new("examples-output/simple.png"));
    });

    // Set up benchmarks
    bench.bench_function("simple", |b| {
        simple(|world| b.iter(|| raytracer::render(&world, WIDTH, HEIGHT, 1.0)))
    });
}

criterion_group!(benches, run);
criterion_main!(benches);

fn simple(mut callback: impl FnMut(World)) {
    let shapes = [
        Shape::sphere(Vec3::new(-0.5, 1, 0), 0.25, Color::red()),
        Shape::sphere(Vec3::new(0, 1, 0), 0.25, Color::green()),
        Shape::sphere(Vec3::new(0.5, 1, 0), 0.25, Color::blue()),
    ];
    let camera =
        Camera::new(Vec3::new(0, 0, 0), Vec3::new(0.001, 1, 0.001).unit());
    let world = World::new(camera, &shapes, Color::black());
    callback(world)
}

fn save_image(image: Image, output_path: &Path) {
    // Copy to image buffer
    let mut image_buffer =
        ImageBuffer::new(image.width as u32, image.height as u32);
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get(x, y);
            image_buffer.put_pixel(
                x as u32,
                y as u32,
                Rgb([color.red, color.green, color.blue]),
            );
        }
    }
    // Create output directory
    let output_directory = output_path.parent();
    if output_directory.is_some() && !output_directory.unwrap().exists() {
        fs::create_dir_all(output_directory.unwrap())
            .expect("Failed to create image directory");
    }
    // Write image
    image_buffer
        .save(output_path)
        .expect("Failed to save image");
}
