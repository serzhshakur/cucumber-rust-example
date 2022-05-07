use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiRequestWithNonce<T: Serialize> {
    pub nonce: i64,
    #[serde(flatten)]
    pub data: Option<T>,
}

impl<T: Serialize> ApiRequestWithNonce<T> {
    pub fn new(data: Option<T>) -> Self {
        Self {
            nonce: Utc::now().timestamp_millis(),
            data,
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct OrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trades: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userref: Option<i32>,
}

#[derive(Debug, Serialize, Default)]
pub struct AddOrderRequest {
    pub ordertype: String,
    pub pair: String,
    pub price: String,
    #[serde(rename(serialize = "type"))]
    pub order_direction: String,
    pub volume: String,
}
