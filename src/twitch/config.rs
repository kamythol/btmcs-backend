use toml;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    config: Config,
}
#[derive(Deserialize)]
pub struct Config {
    oauth: String,
    clientid: String,
}

pub fn get_oauth() -> String {
    let filename = "config.toml";
    let file = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read config");
            panic!("Make sure config is correct.")
        }
    };

    let data: Data = match toml::from_str(&file) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not get config");
            panic!("Make sure config is correct.")
        }
    };

    return data.config.oauth
}
pub fn get_clientid() -> String {
    let filename = "config.toml";
    let file = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read config");
            panic!("Make sure config is correct.")
        }
    };

    let data: Data = match toml::from_str(&file) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not get config");
            panic!("Make sure config is correct.")
        }
    };

    return data.config.clientid
}