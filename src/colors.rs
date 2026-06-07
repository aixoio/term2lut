#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn new_from_tuple(color: (u8, u8, u8)) -> Color {
        Color {
            red: color.0,
            green: color.1,
            blue: color.2,
        }
    }

    pub fn convert_to_decimal(&self) -> (f64, f64, f64) {
        (
            self.red as f64 / 255.0,
            self.green as f64 / 255.0,
            self.blue as f64 / 255.0,
        )
    }
}
