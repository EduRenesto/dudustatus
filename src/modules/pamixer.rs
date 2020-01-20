use std::process::{Command, Stdio};

use super::{Message, Module};
use crate::colors;

/// Gets pulseaudio volume info from `pamixer`.
pub struct Pamixer;

impl Module for Pamixer {
    fn render(&self) -> Vec<Message> {
        let pamixer = Command::new("pamixer")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .arg("--get-volume")
            .spawn()
            .unwrap();

        let output = pamixer.wait_with_output().unwrap();
        let output = String::from_utf8_lossy(output.stdout.as_slice());

        let vol = output.trim().parse::<u8>().unwrap();

        let color = if vol > 80 {
            colors::gruvbox::BRIGHT_ORANGE
        } else {
            colors::gruvbox::LIGHT0
        };

        let icon = if vol <= 33 {
            "\u{e9fa}"
        } else if vol <= 66 {
            "\u{e9fb}"
        } else {
            "\u{e9fc}"
        };

        vec![Message {
            text: format!(" {} {}% ", icon, vol),
            fg: Some((color, 255)),
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_GREEN, 255)),
        }]
    }
}
