use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiRequestWithNonce<T: Serialize> {
    pub nonce: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp: Option<String>,
    #[serde(flatten)]
    pub data: Option<T>,
}

impl<T: Serialize> ApiRequestWithNonce<T> {
    pub fn new(data: Option<T>, tfa_pass: &Option<String>) -> Self {
        let otp = match tfa_pass {
            Some(pass) => Some(pass.to_owned()),
            None => None,
        };

        Self {
            nonce: Utc::now().timestamp_millis(),
            otp,
            data,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetPairInfo {
    Info,
    Leverage,
    Fees,
    Margin,
}

#[derive(Serialize, Default)]
pub struct EmptyRequest {}

#[derive(Debug, Serialize, Default)]
pub struct AssetPairsRequest {
    pub pair: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<AssetPairInfo>,
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

pub trait GenericApiRequest {}

impl GenericApiRequest for AssetPairsRequest {}
