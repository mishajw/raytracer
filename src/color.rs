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

    /// Merge a list of colors by taking the average
    pub fn merge(colors: &[Color]) -> Color {
        let len = colors.len() as u8;
        if len == 0 {
            Color::black();
        }
        let red: u8 = colors.iter().map(|c| c.red).sum();
        let green: u8 = colors.iter().map(|c| c.green).sum();
        let blue: u8 = colors.iter().map(|c| c.blue).sum();
        Color::new(red / len, green / len, blue / len)
    }
}
