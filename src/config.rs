use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub work_duration_mins: u64,
    pub break_duration_mins: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            work_duration_mins: 25,
            break_duration_mins: 5,
        }
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    fs::write("config.json", json)?;
    Ok(())
}

pub fn load_config() -> Config {
    match fs::read_to_string("config.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}
