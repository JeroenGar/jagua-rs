use std::convert::Into;
use std::fmt::{Display, Formatter};

use jagua_rs::entities::general::N_QUALITIES;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Copy, Default)]
pub struct SvgDrawOptions {
    ///The theme to use for the svg
    #[serde(default)]
    pub theme: SvgLayoutTheme,
    ///Draw the quadtree on top
    #[serde(default)]
    pub quadtree: bool,
    ///Draw the fail fast surrogate on top of each item
    #[serde(default)]
    pub surrogate: bool,
    ///Draw the modified shapes used internally instead of the original ones
    #[serde(default)]
    pub draw_cd_shapes: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Copy)]
pub struct SvgLayoutTheme {
    pub stroke_width_multiplier: f32,
    pub bin_fill: Color,
    pub item_fill: Color,
    pub hole_fill: Color,
    pub qz_fill: [Color; N_QUALITIES],
    pub qz_stroke_opac: f32,
}

impl Default for SvgLayoutTheme {
    fn default() -> Self {
        Self::earth_tones()
    }
}

impl SvgLayoutTheme {
    pub fn earth_tones() -> Self {
        SvgLayoutTheme {
            stroke_width_multiplier: 2.0,
            bin_fill: "#CC824A".into(),
            item_fill: "#FFC879".into(),
            hole_fill: "#2D2D2D".into(),
            qz_fill: [
                "#000000".into(), //BLACK
                "#FF0000".into(), //RED
                "#FF5E00".into(), //ORANGE
                "#FFA500".into(), //LIGHT ORANGE
                "#C7A900".into(), //DARK YELLOW
                "#FFFF00".into(), //YELLOW
                "#CBFF00".into(), //GREEN
                "#CBFF00".into(), //GREEN
                "#CBFF00".into(), //GREEN
                "#CBFF00".into(), //GREEN
            ],
            qz_stroke_opac: 0.5,
        }
    }

    pub fn gray() -> Self {
        SvgLayoutTheme {
            stroke_width_multiplier: 2.5,
            bin_fill: "#C3C3C3".into(),
            item_fill: "#8F8F8F".into(),
            hole_fill: "#FFFFFF".into(),
            qz_fill: [
                "#000000".into(), //BLACK
                "#636363".into(), //GRAY
                "#636363".into(), //GRAY
                "#636363".into(), //GRAY
                "#636363".into(), //GRAY
                "#636363".into(), //GRAY
                "#636363".into(), //GRAY
                "#636363".into(), //GRAY
                "#636363".into(), //GRAY
                "#636363".into(), //GRAY
            ],
            qz_stroke_opac: 0.9,
        }
    }
}

pub fn change_brightness(color: Color, fraction: f32) -> Color {
    let Color(r, g, b) = color;

    let r = (r as f32 * fraction) as u8;
    let g = (g as f32 * fraction) as u8;
    let b = (b as f32 * fraction) as u8;
    Color(r, g, b)
}

pub fn blend_colors(color_1: Color, color_2: Color) -> Color {
    //blend color_1 and color_2
    let Color(r_1, g_1, b_1) = color_1;
    let Color(r_2, g_2, b_2) = color_2;

    let r = ((r_1 as f32 * 0.5) + (r_2 as f32 * 0.5)) as u8;
    let g = ((g_1 as f32 * 0.5) + (g_2 as f32 * 0.5)) as u8;
    let b = ((b_1 as f32 * 0.5) + (b_2 as f32 * 0.5)) as u8;

    Color(r, g, b)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color(u8, u8, u8);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

impl From<String> for Color {
    fn from(mut s: String) -> Self {
        if s.starts_with('#') {
            s.remove(0);
        }
        let r = u8::from_str_radix(&s[0..2], 16).unwrap();
        let g = u8::from_str_radix(&s[2..4], 16).unwrap();
        let b = u8::from_str_radix(&s[4..6], 16).unwrap();
        Color(r, g, b)
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Color::from(s.to_owned())
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Color::from(s))
    }
}
