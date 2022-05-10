use std::env;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
    pub priv_key: String,
    pub tfa_password: Option<String>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Profile {
    Dev,
    Ci,
}

impl Profile {
    pub fn init() -> Profile {
        let profile = env::var("PROFILE").unwrap_or_else(|_| "dev".to_string());

        match profile.as_str() {
            "dev" => Self::Dev,
            "ci" => Self::Ci,
            _ => panic!("unable to determine profile"),
        }
    }
}
