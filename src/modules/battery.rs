use std::process::{Command, Stdio};
use std::fs;

use super::{Message, Module};
use crate::colors;

/// Gets batttery status from sysfs
pub struct Battery {
    charge_full: u32,
    path: &'static str
}

impl Battery {
    pub fn new(path: &'static str) -> Battery {
        let full = fs::read_to_string(format!("{}/charge_full", path))
            .unwrap_or("1".to_owned());
        let charge_full = full.trim().parse::<u32>().unwrap();

        Battery {
            charge_full,
            path
        }
    }
}

impl Module for Battery {
    fn render(&self) -> Vec<Message> {
        let cur = fs::read_to_string(format!("{}/charge_now", self.path))
            .unwrap_or("0".to_owned());

        let charge = cur.trim().parse::<f32>().unwrap();

        let percent = (charge / self.charge_full as f32) * 100.0;

        vec![Message {
            text: format!(" \u{e91c} {}% ", percent.round()),
            fg: None,
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_AQUA, 255)),
        }]
    }
}
