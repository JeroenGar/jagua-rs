use serde::{Deserialize, Serialize};
use jaguars::N_QUALITIES;
#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SvgDrawOptions {
    #[serde(default)]
    pub theme: SvgLayoutThemes,
    pub quadtree: bool, //draws the quadtree
    pub haz_prox_grid: bool, //draws the hazard proximity grid
    pub ff_surrogate: bool, //draws the fail fast surrogate for each item
}

impl Default for SvgDrawOptions {
    fn default() -> Self {
        Self{
            theme: SvgLayoutThemes::default(),
            quadtree: false,
            haz_prox_grid: false,
            ff_surrogate: false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SvgLayoutThemes{
    EarthTones,
    Gray,
}

impl SvgLayoutThemes{
    pub fn get_theme(&self) -> SvgLayoutTheme{
        match self{
            SvgLayoutThemes::EarthTones => EARTH_TONES_THEME,
            SvgLayoutThemes::Gray => GRAY_THEME,
        }
    }

}

impl Default for SvgLayoutThemes{
    fn default() -> Self{
        SvgLayoutThemes::EarthTones
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SvgLayoutTheme {
    pub stroke_width_multiplier: f64,
    pub bin_fill: &'static str,
    pub item_fill: &'static str,
    pub hole_fill: &'static str,
    pub qz_fill: [&'static str; N_QUALITIES],
    pub qz_stroke_opac: f64,
}

impl Default for SvgLayoutTheme {
    fn default() -> Self {
        EARTH_TONES_THEME
    }
}

pub static EARTH_TONES_THEME: SvgLayoutTheme = SvgLayoutTheme {
    stroke_width_multiplier: 2.0,
    bin_fill: "#CC824A",
    item_fill: "#FFC879",
    hole_fill: "#2D2D2D",
    qz_fill: [
        "#000000",  //BLACK
        "#FF0000",  //RED
        "#FF5E00",  //ORANGE
        "#FFA500",  //LIGHT ORANGE
        "#FFAD00",  //DARK YELLOW
        "#FFFF00",  //YELLOW
        "#CBFF00",  //GREEN
        "#CBFF00",  //GREEN
        "#CBFF00",  //GREEN
        "#CBFF00"   //GREEN
    ],
    qz_stroke_opac: 0.5,
};
pub static GRAY_THEME: SvgLayoutTheme = SvgLayoutTheme {
    stroke_width_multiplier: 2.5,
    bin_fill: "#C3C3C3",
    item_fill: "#8F8F8F",
    hole_fill: "#FFFFFF",
    qz_fill: ["#636363"; N_QUALITIES],
    qz_stroke_opac: 0.9,
};

pub fn change_brightness(color: &str, fraction: f64) -> String {
    let mut color = color.to_string();
    if color.starts_with('#') {
        color.remove(0);
    }
    let mut r = u8::from_str_radix(&color[0..2], 16).unwrap();
    let mut g = u8::from_str_radix(&color[2..4], 16).unwrap();
    let mut b = u8::from_str_radix(&color[4..6], 16).unwrap();
    r = (r as f64 * fraction) as u8;
    g = (g as f64 * fraction) as u8;
    b = (b as f64 * fraction) as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn blend_colors(color_1: &str, color_2: &str) -> String {
    //blend color_1 and color_2
    let mut color_1 = color_1.to_string();
    let mut color_2 = color_2.to_string();
    if color_1.starts_with('#') {
        color_1.remove(0);
    }
    if color_2.starts_with('#') {
        color_2.remove(0);
    }

    let r_1 = u8::from_str_radix(&color_1[0..2], 16).unwrap();
    let g_1 = u8::from_str_radix(&color_1[2..4], 16).unwrap();
    let b_1 = u8::from_str_radix(&color_1[4..6], 16).unwrap();
    let r_2 = u8::from_str_radix(&color_2[0..2], 16).unwrap();
    let g_2 = u8::from_str_radix(&color_2[2..4], 16).unwrap();
    let b_2 = u8::from_str_radix(&color_2[4..6], 16).unwrap();

    let r = ((r_1 as f64 * 0.5) + (r_2 as f64 * 0.5)) as u8;
    let g = ((g_1 as f64 * 0.5) + (g_2 as f64 * 0.5)) as u8;
    let b = ((b_1 as f64 * 0.5) + (b_2 as f64 * 0.5)) as u8;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}