pub mod api;
pub mod config;
pub mod requests;
pub mod responses;

use api::api::Api;
use config::Config;

#[derive(Debug)]
pub struct Deps {
    pub env: Config,
    pub api: Api,
}

impl Deps {
    pub async fn init() -> anyhow::Result<Self> {
        let env = envy::from_env::<Config>()?;
        let api = Api::new(&env)?;
        Ok(Deps { env, api })
    }
}
