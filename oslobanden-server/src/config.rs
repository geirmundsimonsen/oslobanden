use serde::Deserialize;
use std::env::consts::OS;
use std::fs;

#[derive(Clone, Deserialize)]
pub struct Config {
    // important config here
}

pub fn get_config() -> Config {
    let path = if OS == "linux" {
        "/etc/oslobanden/config.toml"
    } else {
        "config.toml"
    };

    toml::from_str(
        &fs::read_to_string(path).expect("Could not read from path")
    ).expect("Config parsing failed")
}