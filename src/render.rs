use crate::math;
use crate::Image;
use crate::Ray;
use crate::Tracer;
use crate::World;

/// Render an image of the world
// TODO: Is field_of_view the correct term?
pub fn render(
    world: &World,
    width: usize,
    height: usize,
    field_of_view: f64,
) -> Image
{
    let tracer = Tracer::new(&world);
    let mut image = Image::new(width, height);
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
            // Need to offset to make the centre of the image zero
            let offset_x_pixels = image_x as f64 - (width / 2) as f64;
            let offset_y_pixels = image_y as f64 - (height / 2) as f64;
            // Calculate the ray from the camera centre to the correct point
            // on the image plane
            let image_point = image_plane_centre
                + (image_plane_x_unit * offset_x_pixels as f64 * pixel_size)
                + (image_plane_y_unit * offset_y_pixels as f64 * pixel_size);
            let ray = Ray::new(
                world.camera.position,
                (image_point - world.camera.position).unit(),
            );
            // Get what the ray collides with
            let trace_result = tracer.trace(&ray);
            let color = match trace_result {
                // If it does collide, ask the collided shape what color to draw
                Some(result) => result.renderable.get_color(
                    &ray,
                    result.collision_position,
                    &tracer,
                    &world,
                ),
                // If it doesn't collide, use the background color
                None => world.background_color,
            };
            image.put(image_x, image_y, color)
        }
    }
    image
}
