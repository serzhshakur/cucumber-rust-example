pub mod api;
pub mod config;
pub mod requests;
pub mod responses;

use api::api::Api;
use config::{Config, Profile};

#[derive(Debug)]
pub struct Deps {
    pub env: Config,
    pub api: Api,
}

impl Deps {
    pub async fn init() -> Self {
        if let Profile::Dev = Profile::init() {
            dotenv::dotenv().expect("unable to setup env vars");
        }
        let env = match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("Some environment variables are missing\n{:#?}", error),
        };
        let api = Api::new(&env);
        Deps { env, api }
    }
}
