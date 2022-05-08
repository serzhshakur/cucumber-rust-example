use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::{Digest, Sha256, Sha512};

use crate::requests::ApiRequestWithNonce;

type HmacSha512 = Hmac<Sha512>;

pub fn sign_request<T>(
    path: &str,
    req: &ApiRequestWithNonce<T>,
    priv_key: &str,
) -> anyhow::Result<String>
where
    T: Serialize,
{
    let data = format!("{}{}", req.nonce, serde_urlencoded::to_string(req)?);

    let mut sha256 = Sha256::new();
    sha256.update(&data.as_bytes());
    let message_hash = sha256.finalize();

    let mut full = path.as_bytes().to_vec();
    full.extend(message_hash);

    let base64_decoded_key = base64::decode(priv_key)?;
    let mut mac = HmacSha512::new_from_slice(&base64_decoded_key)?;
    mac.update(&full);
    let hmac_result = mac.finalize();

    let res = base64::encode(&hmac_result.into_bytes());
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::requests::AddOrderRequest;

    use super::*;

    #[test]
    fn test_sign() -> anyhow::Result<()> {
        let uri_path = "/0/private/AddOrder";
        let priv_key  = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";

        let data = AddOrderRequest {
            ordertype: "limit".to_string(),
            pair: "XBTUSD".to_string(),
            price: "37500".to_string(),
            order_direction: "buy".to_string(),
            volume: "1.25".to_string(),
        };

        let req = ApiRequestWithNonce {
            nonce: 1616492376594,
            data: Some(data),
            otp: None,
        };

        let res = sign_request(uri_path, &req, priv_key)?;
        assert_eq!(res, "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==");
        Ok(())
    }
}
