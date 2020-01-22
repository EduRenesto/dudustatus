use std::process::{Command, Stdio};
use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use super::{Message, Module};
use crate::colors;

/// Gets music info from sp
pub struct Spotify;

impl Module for Spotify {
    fn render(&self) -> Vec<Message> {
        let sp = Command::new("sp")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .arg("metadata")
            .spawn()
            .unwrap();

        let ret = sp.wait_with_output().unwrap();
        let ret = String::from_utf8_lossy(ret.stdout.as_slice()).trim().to_owned();

        let mut data = HashMap::new();

        for metadata in ret.split("\n") {
            let mut it = metadata.split("|");

            let key = it.next();
            let val = it.next();

            data.insert(key.clone(), val.clone());
        }

        let artist = data.get(&Some("artist")).unwrap_or(&Some("")).unwrap_or("");
        let title = data.get(&Some("title")).unwrap_or(&Some("")).unwrap_or("");

        vec![Message {
            text: format!(" \u{e99e} {} - {} ", artist, title),
            fg: None,
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_PURPLE, 255)),
        }]
    }
}
