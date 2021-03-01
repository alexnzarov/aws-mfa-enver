extern crate rusoto_core;
extern crate serde_derive;
extern crate toml;

use crate::util;

use rusoto_core::Region;
use serde_derive::{Deserialize, Serialize};
use std::{env, fmt::Debug, fs, path::Path, str::FromStr};

const CONFIG_PATH: &str = "./config.toml";
const DEFAULT_REGION: Region = Region::EuCentral1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub mfa_serial_number: String,
    pub region: Option<String>,
}

impl Config {
    pub fn populate_process_env(&self) {
        env::set_var("AWS_SESSION_TOKEN", "");
        env::set_var("AWS_ACCESS_KEY_ID", &self.aws_access_key_id);
        env::set_var("AWS_SECRET_ACCESS_KEY", &self.aws_secret_access_key);
    }

    pub fn get_region(&self) -> Region {
        match &self.region {
            Some(region) => {
                Region::from_str(&region).expect("Error: Invalid region specified in config.toml")
            }
            None => DEFAULT_REGION,
        }
    }
}

pub fn save_config(config: &Config) {
    let content = toml::to_string_pretty(&config).expect("Error: Failed to serialize the config.");

    fs::write(CONFIG_PATH, content).expect("Error: Failed to save the config.");
}

pub fn prepare_config() {
    if Path::new(CONFIG_PATH).exists() {
        return;
    }

    println!("Setting up...");

    let access_key_id = util::get_string("Enter the AWS_ACCESS_KEY_ID:");
    let secret_access_key = util::get_string("Enter the AWS_SECRET_ACCESS_KEY:");
    let serial_number = util::get_string("Enter the serial number of your MFA device:");

    save_config(&Config {
        aws_access_key_id: access_key_id,
        aws_secret_access_key: secret_access_key,
        mfa_serial_number: serial_number,
        region: Some(String::from(DEFAULT_REGION.name())),
    });

    let full_path = fs::canonicalize(CONFIG_PATH).unwrap();

    println!(
        "Configuration was saved to: {}",
        full_path.to_str().unwrap()
    )
}

pub fn get_config() -> Config {
    let content = fs::read_to_string(CONFIG_PATH).expect("Error: Failed to read the config.");

    toml::from_str(&content).unwrap()
}
