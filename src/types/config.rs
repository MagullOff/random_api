use serde::Deserialize;
use crate::types::config::Config as ConfigType;
use std::fs;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct Config {
    pub tag: String,
    pub rocket_port: u16,
    pub db_url: String 
}

#[derive(Deserialize)]
pub struct FileStruct{
    pub config: Config
}

lazy_static! {
    static ref SUBREDDIT: Mutex<String> = Mutex::new(String::new());
}
impl Config {
    pub fn get_subreddit() -> String{
        SUBREDDIT.lock().unwrap().clone()
    }

    pub fn get_config() -> ConfigType {
        let config_file = fs::read_to_string("Config.toml").unwrap();
        let config: FileStruct= toml::from_str(&config_file).unwrap();
        *SUBREDDIT.lock().unwrap() = config.config.tag.clone();
        return config.config;
    }
}
