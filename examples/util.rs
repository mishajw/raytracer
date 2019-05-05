use std::fs;
use std::path::Path;

use image::ImageBuffer;
use image::Rgb;
use raytracer::Image;

pub fn save_image(image: Image, output_path: &Path) {
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
