use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub error: Vec<String>,
    pub result: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTimeResponse {
    pub unixtime: i64,
    pub rfc1123: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrderResponse {
    pub open: HashMap<String, OrderResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub refid: Option<String>,
}
