use log::debug;
use reqwest::ClientBuilder;
use serde::de::DeserializeOwned;

use crate::config::Config;

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(config: &Config) -> Self {
        debug!("Starting new API client instance");

        let client = ClientBuilder::new()
            .build()
            .expect("unable to build an API client");

        Self {
            client,
            base_url: config.api_url.to_owned(),
        }
    }

    pub async fn get_public<T>(&self, path: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, path);
        let res = self.client.get(&url).send().await?;

        Ok(res.json::<T>().await?)
    }
}
