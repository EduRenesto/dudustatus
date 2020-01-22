use std::fmt;

pub mod bspwm;
pub mod cpu;
pub mod pamixer;
pub mod ram;
pub mod spacer;
pub mod spotify;
pub mod time;
pub mod weather;

/// Encapsulates a message.
pub struct Message {
    /// The text itself.
    text: String,

    ///// The font index used for this message
    //font: u8,
    /// The background color
    bg: Option<(&'static str, u8)>,

    /// The foreground color.
    fg: Option<(&'static str, u8)>,

    /// The underline color. Use `None` to disable underline.
    underline: Option<(&'static str, u8)>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some((bg, opacity)) = self.bg {
            write!(f, "%{{B#{:x}{}}}", opacity, bg)?;
        } else {
            write!(f, "%{{B-}}")?;
        }

        if let Some((fg, opacity)) = self.fg {
            write!(f, "%{{F#{:x}{}}}", opacity, fg)?;
        } else {
            write!(f, "%{{F-}}")?;
        }

        if let Some((underline, opacity)) = self.underline {
            write!(f, "%{{+u}}")?;
            write!(f, "%{{U#{:x}{}}}", opacity, underline)?;
        } else {
            write!(f, "%{{-u}}")?;
        }

        //write!(f, "%{{T{}}}", self.font)?;

        write!(f, "{}", self.text)
    }
}

pub trait Module {
    fn render(&self) -> Vec<Message>;
}
