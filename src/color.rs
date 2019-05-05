use std::ops::Mul;

/// RGB color
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    #[allow(missing_docs)]
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    #[allow(missing_docs)]
    pub fn red() -> Color { Color::new(1.0, 0.0, 0.0) }

    #[allow(missing_docs)]
    pub fn green() -> Color { Color::new(0.0, 1.0, 0.0) }

    #[allow(missing_docs)]
    pub fn blue() -> Color { Color::new(0.0, 0.0, 1.0) }

    #[allow(missing_docs)]
    pub fn black() -> Color { Color::new(0.0, 0.0, 0.0) }

    /// Merge a list of colors by taking the average
    pub fn merge(colors: &[Color]) -> Color {
        let len = colors.len() as u8;
        if len == 0 {
            Color::black();
        }
        let red = colors.iter().map(|c| c.red).fold(1.0, Mul::mul);
        let green = colors.iter().map(|c| c.green).fold(1.0, Mul::mul);
        let blue = colors.iter().map(|c| c.blue).fold(1.0, Mul::mul);
        Color::new(red, green, blue)
    }
}
