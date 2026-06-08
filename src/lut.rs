use crate::parser::ColorPallet;

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

        for b in 0..self.size {
            for g in 0..self.size {
                for r in 0..self.size {
                    unimplemented!()
                }
            }
        }

        result
    }
}
