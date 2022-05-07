use async_trait::async_trait;

use crate::responses::{ApiResponse, ServerTimeResponse};

use super::client::ApiClient;

#[derive(Debug)]
pub struct Api {
    client: ApiClient,
}

impl Api {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }
}

#[async_trait]
pub trait MarketApi {
    async fn get_server_time(&self) -> anyhow::Result<Option<ServerTimeResponse>>;
}

#[async_trait]
impl MarketApi for Api {
    async fn get_server_time(&self) -> anyhow::Result<Option<ServerTimeResponse>> {
        let res = self
            .client
            .get_public::<ApiResponse<ServerTimeResponse>>("0/public/Time")
            .await?;

        Ok(res.result)
    }
}
