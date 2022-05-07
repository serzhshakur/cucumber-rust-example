use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
    pub tfa_password: String,
}
