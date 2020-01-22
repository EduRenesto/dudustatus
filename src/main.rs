mod colors;
mod modules;

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

use modules::*;
use modules::{
    bspwm::Bspwm, cpu::Cpu, pamixer::Pamixer, ram::Ram, spacer::Spacer, spotify::Spotify,
    time::Time, weather::Weather,
};

fn flatten(msgs: Vec<Message>) -> String {
    msgs.iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let mut desktops = Vec::new();
    desktops.push(("I".to_string(), "\u{e972}".to_string()));
    desktops.push(("II".to_string(), "\u{e940}".to_string()));
    desktops.push(("III".to_string(), "\u{e976}".to_string()));
    desktops.push(("IV".to_string(), "\u{e990}".to_string()));
    desktops.push(("V".to_string(), "V".to_string()));
    desktops.push(("VI".to_string(), "VI".to_string()));
    desktops.push(("VII".to_string(), "VII".to_string()));
    desktops.push(("VIII".to_string(), "VIII".to_string()));
    desktops.push(("IX".to_string(), "IX".to_string()));
    desktops.push(("X".to_string(), "X".to_string()));

    let owm_app_key = env::var("OWM_APP_KEY").unwrap_or("".to_string());

    // Add your modules here
    let left: Vec<Box<dyn Module>> = vec![Box::new(Spacer), Box::new(Bspwm { aliases: desktops })];
    let center: Vec<Box<dyn Module>> = vec![Box::new(Spotify::new())];
    let right: Vec<Box<dyn Module>> = vec![
        Box::new(Weather::new(owm_app_key, 3461786)),
        Box::new(Spacer),
        Box::new(Ram),
        Box::new(Spacer),
        Box::new(Cpu),
        Box::new(Spacer),
        Box::new(Pamixer),
        Box::new(Spacer),
        Box::new(Time),
    ];

    let mut proc = Command::new("lemonbar")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .arg("-g")
        .arg("x30")
        .arg("-f")
        .arg("Iosevka-9")
        .arg("-f")
        .arg("icomoon\\-feather-12")
        .arg("-B")
        .arg(format!("#{:x}{}", 127, colors::gruvbox::DARK0))
        .arg("-F")
        .arg(format!("#{:x}{}", 255, colors::gruvbox::LIGHT0))
        .spawn()
        .unwrap();

    loop {
        let stdin = proc.stdin.as_mut().unwrap();

        let text_left = left
            .iter()
            .map(|m| m.render())
            .map(flatten)
            .collect::<Vec<String>>()
            .join(" ");

        let text_center = center
            .iter()
            .map(|m| m.render())
            .map(flatten)
            .collect::<Vec<String>>()
            .join(" ");

        let text_right = right
            .iter()
            .map(|m| m.render())
            .map(flatten)
            .collect::<Vec<String>>()
            .join(" ");

        // If unwrap panics, we quit, which is what we want.
        // Yeah, hacky.
        write!(stdin, "%{{l}}{}", text_left).unwrap();
        write!(stdin, "%{{c}}{}", text_center).unwrap();
        write!(stdin, "%{{r}}{}", text_right).unwrap();

        std::thread::sleep(Duration::from_secs(1)); // sleep for 1 second
    }
}
