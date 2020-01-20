use std::process::{Command, Stdio};

use super::{Message, Module};
use crate::colors;

/// Gets music info from playerctl
pub struct Playerctl;

impl Module for Playerctl {
    fn render(&self) -> Vec<Message> {
        let playerctl = Command::new("playerctl")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .arg("metadata")
            .arg("artist")
            .spawn()
            .unwrap();

        let artist = playerctl.wait_with_output().unwrap();
        let artist = String::from_utf8_lossy(artist.stdout.as_slice());
        let artist = artist.trim().to_owned();

        let playerctl = Command::new("playerctl")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .arg("metadata")
            .arg("title")
            .spawn()
            .unwrap();

        let title = playerctl.wait_with_output().unwrap();
        let title = String::from_utf8_lossy(title.stdout.as_slice());
        let title = title.trim().to_owned();

        vec![Message {
            text: format!(" \u{e99e} {} - {} ", artist, title),
            fg: None,
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_PURPLE, 255)),
        }]
    }
}
