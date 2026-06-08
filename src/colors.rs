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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_distance() {
        let black = Color::new(0, 0, 0);
        let red = Color::new(255, 34, 10);

        let dark_blue = Color::new(40, 40, 100);

        assert!(dark_blue.distance(&black) < dark_blue.distance(&red));
    }

    #[test]
    fn test_color_convert_to_decimal() {
        let rgb = (128, 50, 43);
        let decimal_rgb = (
            rgb.0 as f64 / 255.0,
            rgb.1 as f64 / 255.0,
            rgb.2 as f64 / 255.0,
        );

        let color = Color::new_from_decimal_tuple(decimal_rgb);

        assert_eq!(decimal_rgb, color.convert_to_decimal());
    }

    #[test]
    fn test_color_new_from_decimal() {
        let rgb = (128, 50, 43);
        let decimal_rgb = (
            rgb.0 as f64 / 255.0,
            rgb.1 as f64 / 255.0,
            rgb.2 as f64 / 255.0,
        );

        let color = Color::new_from_decimal(decimal_rgb.0, decimal_rgb.1, decimal_rgb.2);

        assert_eq!(color.red, rgb.0);
        assert_eq!(color.green, rgb.1);
        assert_eq!(color.blue, rgb.2);
    }

    #[test]
    fn test_color_new_from_decimal_tuple() {
        let rgb = (128, 50, 43);
        let decimal_rgb = (
            rgb.0 as f64 / 255.0,
            rgb.1 as f64 / 255.0,
            rgb.2 as f64 / 255.0,
        );

        let color = Color::new_from_decimal_tuple(decimal_rgb);

        assert_eq!(color.red, rgb.0);
        assert_eq!(color.green, rgb.1);
        assert_eq!(color.blue, rgb.2);
    }

    #[test]
    fn test_color_new_from_tuple() {
        let rgb = (255, 12, 65);
        let color = Color::new_from_tuple(rgb);

        assert_eq!(rgb.0, color.red);
        assert_eq!(rgb.1, color.green);
        assert_eq!(rgb.2, color.blue);
    }

    #[test]
    fn test_color_new() {
        let rgb = (255, 12, 65);
        let color = Color::new(rgb.0, rgb.1, rgb.2);

        assert_eq!(rgb.0, color.red);
        assert_eq!(rgb.1, color.green);
        assert_eq!(rgb.2, color.blue);
    }

    #[test]
    fn test_decimal_to_u8() {
        let rgb = (128, 50, 43);
        let decimal_rgb = (
            rgb.0 as f64 / 255.0,
            rgb.1 as f64 / 255.0,
            rgb.2 as f64 / 255.0,
        );

        let deciaml_to_rgb = (
            decimal_to_u8(decimal_rgb.0),
            decimal_to_u8(decimal_rgb.1),
            decimal_to_u8(decimal_rgb.2),
        );

        assert_eq!(rgb, deciaml_to_rgb);
    }
}
