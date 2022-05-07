pub mod api;
pub mod config;
pub mod responses;
pub mod requests;

use api::{client::ApiClient, api::Api};
use config::Config;

#[derive(Debug)]
pub struct Deps {
    pub env: Config,
    pub api: Api,
}

impl Deps {
    pub async fn init() -> Self {
        dotenv::dotenv().expect("unable to setup env vars");
        let env = match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("Some environment variables are missing\n{:#?}", error),
        };

        let client = ApiClient::new(&env);
        let api = Api::new(client);
        Deps { env, api }
    }
}
