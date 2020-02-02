use serde::Deserialize;

/// The position of the bar in the screen
#[derive(Copy, Clone, Deserialize)]
pub enum Position {
    Top,
    Bottom
}

/// Describes the geometry of the bar.
#[derive(Copy, Clone, Deserialize)]
pub struct Geometry {
    pub width: u32,
    pub height: u32,
    pub xoffset: u32,
    pub yoffset: u32
}

#[derive(Deserialize)]
pub struct Config<'a> {
    pub default_update_interval: f32,

    pub fonts: Vec<(String, u8)>,

    pub modules_left: Vec<String>,
    pub modules_center: Vec<String>,
    pub modules_right: Vec<String>,

    pub position: Position,
    pub geometry: Geometry,

    pub battery: crate::modules::battery::Settings,
    pub bspwm: crate::modules::bspwm::Settings,
}
