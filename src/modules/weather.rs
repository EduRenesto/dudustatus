use super::{Message, Module};
use crate::colors;

use std::time::Duration;

use std::sync::{Arc, RwLock};
use std::thread;

const OWM_API_URL: &'static str = "https://api.openweathermap.org/data/2.5";

#[derive(Default)]
struct Forecast {
    description: String,
    icon: String,
    temp: f32,
}

pub struct Weather {
    current_forecast: Arc<RwLock<Forecast>>,
}

impl Weather {
    pub fn new(api_key: String, city_id: u32) -> Weather {
        let weather = Weather {
            current_forecast: Arc::new(RwLock::new(Forecast {
                description: "no data :(".to_string(),
                icon: "50d".to_string(),
                temp: 0.0,
            })),
        };

        if api_key == "" {
            return weather;
        }

        let forecast = weather.current_forecast.clone();
        thread::spawn(move || {
            loop {
                let url = format!("{}/weather/?id={}&appid={}", OWM_API_URL, city_id, api_key);

                if let Ok(data) = reqwest::blocking::get(&url) {
                    let parsed: serde_json::Value = serde_json::from_reader(data)
                        .expect("Failed to parse data from OpenWeatherMap");

                    let desc = parsed
                        .pointer("/weather/0/description")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let icon = parsed.pointer("/weather/0/icon").unwrap().as_str().unwrap();
                    let temp = parsed.pointer("/main/temp").unwrap().as_f64().unwrap() as f32;

                    let desc = voca_rs::case::title_case(desc);

                    let mut forecast_ptr = forecast.write().unwrap();
                    forecast_ptr.description = desc;
                    forecast_ptr.icon = icon.to_owned();
                    forecast_ptr.temp = temp;

                    //self.current_forecast.replace(Some(Forecast {
                    //description: desc,
                    //icon: icon.to_owned(),
                    //temp: temp,
                    //}));

                    //self.last_update.replace(Instant::now());
                };

                thread::sleep(Duration::from_secs(15 * 60)); // wait for 15 minutes
            }
        });

        weather
    }
}

impl Module for Weather {
    fn render(&self) -> Vec<Message> {
        let mut ret = Vec::new();

        let forecast = self.current_forecast.read().unwrap();
        let icon = match forecast.icon.as_str() {
            "01d" | "02d" => "\u{ed97}",                 // sun
            "01n" | "02n" => "\u{e99a}",                 // moon
            "03d" | "03n" | "04d" | "04n" => "\u{e93a}", // clouds
            "09d" | "09n" => "\u{e93b}",                 // showers
            "10d" | "10n" => "\u{e93e}",                 // rain
            "11d" | "11n" => "\u{e93c}",                 // thunderstorm
            "13d" | "13n" => "\u{e93f}",                 // snow
            "50d" | "50n" => "\u{ea01}",                 // mist
            _ => unreachable!(),
        }
        .to_owned();

        let desc = forecast.description.clone();
        let temp = (forecast.temp - 273.15f32).round(); // celsius master race

        ret.push(Message {
            text: format!(" {} {}, {}ºC", icon, desc, temp),
            fg: None,
            bg: None,
            underline: Some((colors::gruvbox::BRIGHT_AQUA, 255)),
        });

        ret
    }
}
