use std::process::{Command, Stdio};

use super::{Message, Module};
use crate::colors;

pub struct Bspwm {
    pub aliases: Vec<(String, String)>,
}

impl Module for Bspwm {
    fn render(&self) -> Vec<Message> {
        let bspc = Command::new("bspc")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .arg("query")
            .arg("--desktops")
            .arg("--desktop")
            .arg("--names")
            .spawn()
            .unwrap();

        let output = bspc.wait_with_output().unwrap();
        let output = String::from_utf8_lossy(output.stdout.as_slice());

        let active = output.trim();

        let mut ret = Vec::new();

        for (desktop, name) in self.aliases.iter() {
            if desktop == active {
                ret.push(Message {
                    text: format!(" {} ", name),
                    fg: Some((colors::gruvbox::BRIGHT_RED, 255)),
                    bg: None,
                    underline: Some((colors::gruvbox::BRIGHT_RED, 255)),
                })
            } else {
                ret.push(Message {
                    text: format!(" {} ", name),
                    fg: None,
                    bg: None,
                    underline: None,
                })
            }
        }

        ret
    }
}
