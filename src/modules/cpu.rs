use std::process::{Command, Stdio};

use super::{Message, Module};
use crate::colors;

/// Gets pulseaudio volume info from `pamixer`.
pub struct Cpu;

impl Module for Cpu {
    fn render(&self) -> Vec<Message> {
        let pamixer = Command::new("ps")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .arg("axo")
            .arg("pcpu")
            .spawn()
            .unwrap();

        let output = pamixer.wait_with_output().unwrap();
        let output = String::from_utf8_lossy(output.stdout.as_slice());

        let usage = output
            .trim()
            .split("\n")
            //.iter()
            .skip(1)
            .map(|c| c.trim().parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
            .iter()
            .sum::<f32>()
            .round();

        vec![Message {
            text: " \u{e900}".to_string(),
            fg: Some((colors::gruvbox::BRIGHT_PURPLE, 255)),
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_PURPLE, 255)),
        }, Message {
            text: format!(" {:02}% ", usage),
            fg: None,
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_PURPLE, 255)),
        }]
    }
}
