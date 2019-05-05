#[macro_use]
extern crate criterion;
extern crate image;
extern crate raytracer;

use std::fs;
use std::path::Path;

use criterion::Criterion;
use image::ImageBuffer;
use image::Rgb;

use raytracer::shape::Shape;
use raytracer::shape::Sphere;
use raytracer::texture::Diffuse;
use raytracer::texture::Solid;
use raytracer::Camera;
use raytracer::Color;
use raytracer::Image;
use raytracer::Light;
use raytracer::Vec3;
use raytracer::World;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;
const SMALL_WIDTH: usize = 60;
const SMALL_HEIGHT: usize = 40;

fn run(bench: &mut Criterion) {
    // Write the images to the example-output directory
    println!("Creating images in ./examples-output");
    let simple_image = raytracer::render(&simple(), WIDTH, HEIGHT, 1.0);
    save_image(simple_image, Path::new("examples-output/simple.png"));
    let diffuse_image = raytracer::render(&diffuse(), WIDTH, HEIGHT, 1.0);
    save_image(diffuse_image, Path::new("examples-output/diffuse.png"));
    println!("Finished creating images in ./examples-output");

    // Set up benchmarks
    bench.bench_function("simple", |b| {
        b.iter(|| raytracer::render(&simple(), SMALL_WIDTH, SMALL_HEIGHT, 1.0))
    });
}

criterion_group!(benches, run);
criterion_main!(benches);

fn simple() -> World {
    let shapes = vec![
        Sphere::new(Vec3::new(-0.5, 1, 0), 0.25)
            .with_textures(vec![Solid::new(Color::red())]),
        Sphere::new(Vec3::new(0, 1, 0), 0.25)
            .with_textures(vec![Solid::new(Color::green())]),
        Sphere::new(Vec3::new(0.5, 1, 0), 0.25)
            .with_textures(vec![Solid::new(Color::blue())]),
    ];
    let camera =
        Camera::new(Vec3::new(0, 0, 0), Vec3::new(0.001, 1, 0.001).unit());
    World::new(camera, shapes, vec![], Color::black())
}

fn diffuse() -> World {
    let shapes = vec![
        Sphere::new(Vec3::new(-0.5, 1, 0), 0.25)
            .with_textures(vec![Solid::new(Color::red()), Diffuse::new()]),
        Sphere::new(Vec3::new(0, 1, 0), 0.25)
            .with_textures(vec![Solid::new(Color::green()), Diffuse::new()]),
        Sphere::new(Vec3::new(0.5, 1, 0), 0.25)
            .with_textures(vec![Solid::new(Color::blue()), Diffuse::new()]),
    ];
    let camera =
        Camera::new(Vec3::new(0, 0, 0), Vec3::new(0.001, 1, 0.001).unit());
    let lights = vec![Light::new(Vec3::new(-1, 1, 1), 1.0)];
    World::new(camera, shapes, lights, Color::black())
}

fn save_image(image: Image, output_path: &Path) {
    // Copy to image buffer
    let mut image_buffer =
        ImageBuffer::new(image.width as u32, image.height as u32);
    for x in 0..image.width {
        for y in 0..image.height {
            let color = image.get(x, y);
            let red = (color.red * 255.0) as u8;
            let green = (color.green * 255.0) as u8;
            let blue = (color.blue * 255.0) as u8;
            image_buffer.put_pixel(x as u32, y as u32, Rgb([red, green, blue]));
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
