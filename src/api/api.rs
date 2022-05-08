use async_trait::async_trait;

use crate::{
    config::Config,
    requests::OrderRequest,
    responses::{ApiResponse, OpenOrderResponse, ServerTimeResponse},
};

use super::client::ApiClient;

#[derive(Debug)]
pub struct Api {
    client: ApiClient,
}

impl Api {
    pub fn new(env: &Config) -> Self {
        let client = ApiClient::new(env);
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

#[async_trait]
pub trait UserApi {
    async fn get_open_orders(&self) -> anyhow::Result<OpenOrderResponse>;
}

#[async_trait]
impl UserApi for Api {
    async fn get_open_orders(&self) -> anyhow::Result<OpenOrderResponse> {
        let req = OrderRequest::default();
        let res = self
            .client
            .post_private::<OrderRequest, OpenOrderResponse>("0/private/OpenOrders", Some(req))
            .await?;

        Ok(res)
    }
}
