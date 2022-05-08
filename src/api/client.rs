use anyhow::bail;
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
    pub fn new(config: &Config) -> Self {
        debug!("Starting a new API client instance");

        let client = ClientBuilder::new()
            .build()
            .expect("unable to build an API client");

        Self {
            client,
            env: config.clone(),
        }
    }

    pub async fn get_public<R: DeserializeOwned, T: Serialize>(
        &self,
        path: &str,
        request_data: T,
    ) -> anyhow::Result<R> {
        let url = format!("{}/{}", self.env.api_url, normalize_path(path));
        let res = self.client.get(&url).query(&request_data).send().await?;

        Ok(res.json().await?)
    }

    pub async fn post_private<T, D>(&self, path: &str, data: Option<T>) -> anyhow::Result<D>
    where
        T: Serialize,
        D: DeserializeOwned + Serialize,
    {
        let request_data = ApiRequestWithNonce::new(data);
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
            let api_response = res.json::<ApiResponse<D>>().await?;
            if !api_response.error.is_empty() {
                bail!("API call failed with errors: {:?}", api_response.error);
            } else {
            }
            Ok(api_response.result.unwrap())
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
