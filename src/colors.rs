#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn decimal_to_u8(x: f64) -> u8 {
    let x = x.clamp(0.0, 1.0);

    (x * 255.0).round() as u8
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

    pub fn new_from_decimal_tuple(decimal: (f64, f64, f64)) -> Color {
        Color {
            red: decimal_to_u8(decimal.0),
            green: decimal_to_u8(decimal.1),
            blue: decimal_to_u8(decimal.2),
        }
    }

    pub fn new_from_decimal(dr: f64, dg: f64, db: f64) -> Color {
        Color::new_from_decimal_tuple((dr, dg, db))
    }

    pub fn convert_to_decimal(&self) -> (f64, f64, f64) {
        (
            self.red as f64 / 255.0,
            self.green as f64 / 255.0,
            self.blue as f64 / 255.0,
        )
    }

    pub fn distance(&self, other: &Color) -> f64 {
        let decimal = self.convert_to_decimal();
        let other_decimal = other.convert_to_decimal();

        (decimal.0 - other_decimal.0).powf(2.0)
            + (decimal.1 - other_decimal.1).powf(2.0)
            + (decimal.2 - other_decimal.2).powf(2.0)
    }
}
