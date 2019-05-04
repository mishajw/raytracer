use image::ImageBuffer;
use image::Rgb;

use crate::math;
use crate::Ray;
use crate::Tracer;
use crate::World;

/// Render an image of the world
// TODO: Is field_of_view the correct term?
pub fn render(
    world: World,
    width: usize,
    height: usize,
    field_of_view: f64,
    output_path: &str,
)
{
    let tracer = Tracer::new(&world);
    let mut image = ImageBuffer::new(width as u32, height as u32);
    // The centre of the image plane that we project through
    let image_plane_centre = world.camera.position + world.camera.direction;
    // The size (in world units) of a pixel
    let pixel_size = field_of_view / width as f64;
    // The basis vectors for the image plane
    let (image_plane_x_unit, image_plane_y_unit) =
        math::get_camera_unit_vectors(&world.camera.direction);
    // For every pixel...
    for image_x in 0..width {
        for image_y in 0..height {
            // Calculate the vector from the camera centre to the correct point
            // on the image plane
            // Need to offset to make the centre of the image zero
            let offset_x_pixels = image_x as f64 - (width / 2) as f64;
            let offset_y_pixels = image_y as f64 - (height / 2) as f64;
            let image_point = image_plane_centre
                + (image_plane_x_unit * offset_x_pixels as f64 * pixel_size)
                + (image_plane_y_unit * offset_y_pixels as f64 * pixel_size);
            let ray = Ray::new(
                world.camera.position,
                (image_point - world.camera.position).unit(),
            );
            let trace_result = tracer.trace(&ray);
            let color = match trace_result {
                Some(result) => result.shape.get_color(
                    &ray,
                    result.collision_position,
                    &tracer,
                ),
                None => world.background_color,
            };
            image.put_pixel(
                image_x as u32,
                image_y as u32,
                Rgb([color.red, color.green, color.blue]),
            )
        }
    }
    // TODO: Replace with Result<>
    image.save(output_path).expect("Failed to save image");
}
