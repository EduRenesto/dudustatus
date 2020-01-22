use std::process::{Command, Stdio};
use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use std::thread;
use std::time::Duration;

use super::{Message, Module};
use crate::colors;

/// Gets music info from `sp`.
/// `sp` sometimes hangs, freezing the whole bar. So, we fire another thread
/// and wait for its output separately from the bar.
pub struct Spotify {
    current_title: Arc<RwLock<String>>,
    current_artist: Arc<RwLock<String>>
}

impl Spotify {
    pub fn new() -> Spotify {
        let title = Arc::new(RwLock::new("".to_string()));
        let artist = Arc::new(RwLock::new("".to_string()));

        let title_clone = title.clone();
        let artist_clone = artist.clone();

        thread::spawn(move || {
            // TODO listen to spotifyd on_change triggers
            // For now, we're just polling.

            loop {
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
                    if metadata == "" {
                        break;
                    }

                    let mut it = metadata.split("|");

                    if let Some(key) = it.next() {
                        let val = it.next().expect("sp returned key but not value, wtf!");

                        data.insert(key, val);
                    }
                }

                if let Some(title) = data.get("title") {
                    let mut title_ptr = title_clone.write().unwrap();
                    *title_ptr = title.to_string();
                }

                if let Some(artist) = data.get("artist") {
                    let mut artist_ptr = artist_clone.write().unwrap();
                    *artist_ptr = artist.to_string();
                }

                thread::sleep(Duration::from_secs(2));
            }
        });

        Spotify {
            current_title: title,
            current_artist: artist
        }
    }
}

impl Module for Spotify {
    fn render(&self) -> Vec<Message> {
        let artist = self.current_artist.read().unwrap();
        let title = self.current_title.read().unwrap();

        vec![Message {
            text: format!(" \u{e99e} {} - {} ", artist, title),
            fg: None,
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_PURPLE, 255)),
        }]
    }
}
