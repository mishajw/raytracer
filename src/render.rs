use std::f64;

use crate::get_color_for_ray;
use crate::math;
use crate::Image;
use crate::Ray;
use crate::Tracer;
use crate::World;

/// Render an image of the world
// TODO: Is field_of_view the correct term?
pub fn render(
    world: &World,
    image_width: usize,
    image_height: usize,
    field_of_view: f64,
) -> Image
{
    assert!(field_of_view.is_sign_positive());
    assert!(field_of_view < f64::consts::PI);

    let tracer = Tracer::new(&world);
    let mut image = Image::new(image_width, image_height);
    // The centre and width of the image plane that we project through
    let image_plane_centre = world.camera.position + world.camera.direction;
    let image_plane_width = 2.0 * (field_of_view / 2.0).tan();
    // The size (in world units) of a pixel
    let pixel_size = image_plane_width / image_width as f64;
    // The basis vectors for the image plane
    let (image_plane_x_unit, image_plane_y_unit) =
        math::get_camera_unit_vectors(&world.camera.direction);
    // For every pixel...
    for image_x in 0..image_width {
        for image_y in 0..image_height {
            // Need to offset to make the centre of the image zero
            let offset_x_pixels = image_x as f64 - (image_width / 2) as f64;
            let offset_y_pixels = image_y as f64 - (image_height / 2) as f64;
            // Calculate the ray from the camera centre to the correct point
            // on the image plane
            let image_point = image_plane_centre
                + (image_plane_x_unit * offset_x_pixels as f64 * pixel_size)
                + (image_plane_y_unit * offset_y_pixels as f64 * pixel_size);
            let ray = Ray::new(
                world.camera.position,
                (image_point - world.camera.position).unit(),
            );
            // Get the color that the ray "sees"
            let color = get_color_for_ray(&ray, &tracer, &world);
            image.put(image_x, image_y, color)
        }
    }
    image
}
