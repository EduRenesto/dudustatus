use super::{Message, Module};

pub struct Spacer;

impl Module for Spacer {
    fn render(&self) -> Vec<Message> {
        vec![Message {
            text: " ".to_string(),
            bg: None,
            fg: None,
            underline: None,
        }]
    }
}
