use super::{Cookie, login_request};
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "login/qrcode/unikey";

pub async fn login_qr_key(
    crypto: Option<Crypto>,
) -> Result<(Value, Cookie), Box<dyn std::error::Error>> {
    let data = json!({
      "type": 1,
    });
    let resp = login_request(API.to_string(), data, None, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_login_qr_key() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_qr_key(Some(Crypto::Api)).await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_login_qr_key() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_qr_key(Some(Crypto::Linuxapi)).await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_login_qr_key() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_qr_key(Some(Crypto::Weapi)).await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_login_qr_key() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_qr_key(Some(Crypto::Eapi)).await?;
        println!("{}", resp.0);

        Ok(())
    }
}
