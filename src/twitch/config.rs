use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    config: Config,
}
#[derive(Deserialize)]
struct Config {
    oauth: String,
    clientid: String,
}

pub fn get_oauth() -> String {
    let filename = "config.toml";
    let file = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read config");
            panic!("Config file seems to be missing.")
        }
    };

    let data: Data = match toml::from_str(&file) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not get config");
            panic!("Make sure config is correct.")
        }
    };

    data.config.oauth
}
pub fn get_clientid() -> String {
    let filename = "config.toml";
    let file = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read config");
            panic!("Config file seems to be missing.")
        }
    };

    let data: Data = match toml::from_str(&file) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not get config");
            panic!("Make sure config is correct.")
        }
    };

    data.config.clientid
}