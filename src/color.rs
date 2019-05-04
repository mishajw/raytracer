/// RGB color
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Color { Color { red, green, blue } }

    #[allow(missing_docs)]
    pub fn red() -> Color { Color::new(255, 0, 0) }

    #[allow(missing_docs)]
    pub fn green() -> Color { Color::new(0, 255, 0) }

    #[allow(missing_docs)]
    pub fn blue() -> Color { Color::new(0, 0, 255) }

    #[allow(missing_docs)]
    pub fn black() -> Color { Color::new(0, 0, 0) }
}
