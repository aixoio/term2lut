use crate::colors::Color;
use serde::Deserialize;

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
struct ColorPalletVHS {
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

impl ColorPalletVHS {
    pub fn load_to_color_pallet(path: String) -> ColorPallet {
        unimplemented!();
    }
}
