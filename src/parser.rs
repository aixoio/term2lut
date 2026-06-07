use std::{error, fs};

use crate::colors::Color;
use serde::Deserialize;

#[derive(Debug)]
pub struct ColorPallet {
    pub name: String,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub blue: Color,
    pub purple: Color,
    pub cyan: Color,
    pub white: Color,
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_blue: Color,
    pub bright_purple: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub selection: Color,
}

#[derive(Debug, Deserialize)]
pub struct ColorPalletVHS {
    name: String,
    black: String,
    red: String,
    green: String,
    blue: String,
    purple: String,
    cyan: String,
    white: String,
    #[serde(rename = "brightBlack")]
    bright_black: String,
    #[serde(rename = "brightRed")]
    bright_red: String,
    #[serde(rename = "brightGreen")]
    bright_green: String,
    #[serde(rename = "brightBlue")]
    bright_blue: String,
    #[serde(rename = "brightPurple")]
    bright_purple: String,
    #[serde(rename = "brightCyan")]
    bright_cyan: String,
    #[serde(rename = "brightWhite")]
    bright_white: String,
    background: String,
    foreground: String,
    cursor: String,
    selection: String,
}

fn parse_hex_color(s: &str) -> Result<(u8, u8, u8), std::num::ParseIntError> {
    let hex = s.strip_prefix("#").unwrap_or(s);

    let value = u32::from_str_radix(hex, 16)?;

    let r = ((value >> 16) & 0xff) as u8;
    let g = ((value >> 8) & 0xff) as u8;
    let b = (value & 0xff) as u8;

    Ok((r, g, b))
}

impl ColorPalletVHS {
    pub fn load_file_to_color_pallet(path: &str) -> Result<ColorPallet, Box<dyn error::Error>> {
        let data = fs::read_to_string(path)?;
        let vhs_pallet: ColorPalletVHS = serde_json::from_str(&data)?;
        Ok(vhs_pallet.convert()?)
    }

    pub fn convert(self) -> Result<ColorPallet, std::num::ParseIntError> {
        let color_pallet = ColorPallet {
            name: self.name,
            black: Color::new_from_tuple(parse_hex_color(&self.black)?),
            red: Color::new_from_tuple(parse_hex_color(&self.red)?),
            green: Color::new_from_tuple(parse_hex_color(&self.green)?),
            blue: Color::new_from_tuple(parse_hex_color(&self.blue)?),
            purple: Color::new_from_tuple(parse_hex_color(&self.purple)?),
            cyan: Color::new_from_tuple(parse_hex_color(&self.cyan)?),
            white: Color::new_from_tuple(parse_hex_color(&self.white)?),
            bright_black: Color::new_from_tuple(parse_hex_color(&self.bright_black)?),
            bright_red: Color::new_from_tuple(parse_hex_color(&self.bright_red)?),
            bright_green: Color::new_from_tuple(parse_hex_color(&self.bright_green)?),
            bright_blue: Color::new_from_tuple(parse_hex_color(&self.bright_blue)?),
            bright_purple: Color::new_from_tuple(parse_hex_color(&self.bright_purple)?),
            bright_cyan: Color::new_from_tuple(parse_hex_color(&self.bright_cyan)?),
            bright_white: Color::new_from_tuple(parse_hex_color(&self.bright_white)?),
            background: Color::new_from_tuple(parse_hex_color(&self.background)?),
            foreground: Color::new_from_tuple(parse_hex_color(&self.foreground)?),
            cursor: Color::new_from_tuple(parse_hex_color(&self.cursor)?),
            selection: Color::new_from_tuple(parse_hex_color(&self.selection)?),
        };

        Ok(color_pallet)
    }
}
