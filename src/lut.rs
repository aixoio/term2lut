use crate::{colors::Color, parser::ColorPallet};

pub struct LUT {
    pub size: i32,
    pub pallet: ColorPallet,
}

impl LUT {
    pub fn new(size: i32, pallet: ColorPallet) -> LUT {
        LUT { size, pallet }
    }

    pub fn generate_lut_to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!("TITLE \"{}\"\n", self.pallet.name));
        result.push_str(&format!("LUT_3D_SIZE \"{}\"\n", self.size));
        result.push_str("DOMAIN_MIN 0.0 0.0 0.0\n");
        result.push_str("DOMAIN_MAX 1.0 1.0 1.0\n");

        for b in 0..self.size {
            for g in 0..self.size {
                for r in 0..self.size {
                    let sample = Color::new_from_decimal(
                        r as f64 / (self.size - 1) as f64,
                        g as f64 / (self.size - 1) as f64,
                        b as f64 / (self.size - 1) as f64,
                    );

                    let nearest = self.pallet.closest_pallet_color(&sample);
                    let nearest_decimal = nearest.convert_to_decimal();

                    let line = format!(
                        "{:.6} {:.6} {:.6}\n",
                        nearest_decimal.0, nearest_decimal.1, nearest_decimal.2
                    );

                    result.push_str(&line);
                }
            }
        }

        result
    }
}
