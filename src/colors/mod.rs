pub mod gruvbox;

use std::fmt;

/// Stores a RGB(A) color.
#[derive(Copy, Clone)]
pub enum Color<'a> {
    Rgb(&'a str),
    Rgba(&'a str, u8)
}

impl<'a> Color<'a> {
    pub fn with_opacity(&self, opacity: u8) -> Color<'a> {
        match *self {
            Color::Rgb(str) | Color::Rgba(str, _) => Color::Rgba(str, opacity)
        }
    }
}

impl<'a> fmt::Display for Color<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Rgb(rgb) => write!(fmt, "#{}", rgb),
            Color::Rgba(rgb, alpha) => write!(fmt, "#{:x}{}", alpha, rgb)
        }
    }
}
