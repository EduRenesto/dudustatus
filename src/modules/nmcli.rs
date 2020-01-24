use std::process::{Command, Stdio};

use super::{Message, Module};
use crate::colors;

/// Gets network data using `nmcli`.
pub struct Nmcli;

impl Module for Nmcli {
    fn render(&self) -> Vec<Message> {
        let nmcli = Command::new("nmcli")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .arg("--colors")
            .arg("no")
            .arg("--fields")
            .arg("NAME")
            .arg("conn")
            .arg("show")
            .arg("--active")
            .spawn()
            .unwrap();

        let output = nmcli.wait_with_output().unwrap();
        let output = String::from_utf8_lossy(output.stdout.as_slice());

        let conn = output
            .trim()
            .split("\n")
            .skip(1)
            .next()
            .unwrap_or("No connection");

        vec![Message {
            text: format!(" \u{e9be} {:02}% ", conn),
            fg: None,
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_YELLOW, 255)),
        }]
    }
}
