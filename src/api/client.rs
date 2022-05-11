use anyhow::{bail, Context};
use log::debug;
use reqwest::ClientBuilder;
use serde::{de::DeserializeOwned, Serialize};

use crate::{config::Config, requests::ApiRequestWithNonce, responses::ApiResponse};

use super::signer::sign_request;

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    env: Config,
}

impl ApiClient {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        debug!("Starting a new API client instance");

        let client = ClientBuilder::new().build()?;
        Ok(Self {
            client,
            env: config.clone(),
        })
    }

    pub async fn get_public<R: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        request_data: R,
    ) -> anyhow::Result<T> {
        let url = format!("{}/{}", self.env.api_url, normalize_path(path));
        let res = self.client.get(&url).query(&request_data).send().await?;

        Ok(res.json().await?)
    }

    pub async fn post_private<R, T>(&self, path: &str, data: Option<R>) -> anyhow::Result<T>
    where
        R: Serialize,
        T: DeserializeOwned + Serialize,
    {
        let request_data = ApiRequestWithNonce::new(data, &self.env.tfa_password);
        let path = normalize_path(path);
        let url = format!("{}/{}", self.env.api_url, path);
        let api_sign = sign_request(&path, &request_data, &self.env.priv_key)?;

        let res = self
            .client
            .post(&url)
            .form(&request_data)
            .header("API-Key", &self.env.api_key)
            .header("API-Sign", &api_sign)
            .send()
            .await?;

        if res.status().is_success() {
            let api_response = res.json::<ApiResponse<T>>().await?;
            if !api_response.error.is_empty() {
                bail!("API call failed with errors: {:?}", api_response.error);
            }
            Ok(api_response
                .result
                .context("can't parse 'result' in API response")?)
        } else {
            bail!("unable to retrieve data from the API")
        }
    }
}

fn normalize_path(url: &str) -> String {
    format!("/{}", url.trim_start_matches('/'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("/some/url"), "/some/url");
    }

    #[test]
    fn test_normalize_path_without_leading_slash() {
        assert_eq!(normalize_path("some/url"), "/some/url");
    }
    #[test]
    fn test_normalize_path_with_multiple_leading_slashes() {
        assert_eq!(normalize_path("///some/url"), "/some/url");
    }
}
