use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    config::Config,
    requests::{AssetPairsRequest, EmptyRequest, OrderRequest},
    responses::{ApiResponse, AssetPairResponse, OpenOrderResponse, ServerTimeResponse},
};

use super::client::ApiClient;

#[derive(Debug)]
pub struct Api {
    client: ApiClient,
}

impl Api {
    pub fn new(env: &Config) -> anyhow::Result<Self> {
        let client = ApiClient::new(env)?;
        Ok(Self { client })
    }
}

#[async_trait]
pub trait MarketApi {
    async fn get_server_time(&self) -> anyhow::Result<Option<ServerTimeResponse>>;
    async fn get_asset_pairs(
        &self,
        request: AssetPairsRequest,
    ) -> anyhow::Result<HashMap<String, AssetPairResponse>>;
}

#[async_trait]
impl MarketApi for Api {
    async fn get_server_time(&self) -> anyhow::Result<Option<ServerTimeResponse>> {
        let res: ApiResponse<ServerTimeResponse> = self
            .client
            .get_public("0/public/Time", EmptyRequest)
            .await?;

        Ok(res.result)
    }

    async fn get_asset_pairs(
        &self,
        request: AssetPairsRequest,
    ) -> anyhow::Result<HashMap<String, AssetPairResponse>> {
        let res: ApiResponse<HashMap<String, AssetPairResponse>> = self
            .client
            .get_public("0/public/AssetPairs", request)
            .await?;

        Ok(res.result.unwrap())
    }
}

#[async_trait]
pub trait UserApi {
    async fn get_open_orders(&self) -> anyhow::Result<OpenOrderResponse>;
}

#[async_trait]
impl UserApi for Api {
    async fn get_open_orders(&self) -> anyhow::Result<OpenOrderResponse> {
        let req = OrderRequest {
            trades: Some(true),
            userref: None,
        };
        let res = self
            .client
            .post_private("0/private/OpenOrders", Some(req))
            .await?;

        Ok(res)
    }
}
