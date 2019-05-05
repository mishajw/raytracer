use crate::Color;

/// Rendered image
#[allow(missing_docs)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Image {
    #[allow(missing_docs)]
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![vec![Color::black(); height]; width],
        }
    }

    /// Get the pixel at `(x, y)`
    pub fn get(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[x][y]
    }

    /// Set the pixel at `(x, y)`
    pub fn put(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[x][y] = color;
    }
}
