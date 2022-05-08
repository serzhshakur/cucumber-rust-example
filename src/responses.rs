use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub error: Vec<String>,
    pub result: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTimeResponse {
    #[serde(with = "ts_seconds")]
    pub unixtime: DateTime<Utc>,
    pub rfc1123: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPairResponse {
    pub altname: String,
    pub wsname: String,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrderResponse {
    pub open: HashMap<String, OrderResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub refid: Option<String>,
}
