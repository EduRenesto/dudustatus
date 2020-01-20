use super::{Message, Module};

use crate::colors;

pub struct Time;

impl Module for Time {
    fn render(&self) -> Vec<Message> {
        let now = time::Time::now() - time::Duration::hours(3);
        let today = time::Date::today().with_time(now);

        vec![
            Message {
                text: " \u{e9fe}".to_string(),
                bg: None,
                fg: None,
                underline: Some((colors::gruvbox::NEUTRAL_RED, 255)),
            },
            Message {
                text: today.format("%H:%M %d/%m/%y "),
                bg: None,
                fg: None,
                underline: Some((colors::gruvbox::NEUTRAL_RED, 255)),
            },
        ]
    }
}
