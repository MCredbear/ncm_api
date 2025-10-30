use super::{Cookie, login_request};
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "w/nuser/account/get";

/// 登录刷新
pub async fn login_status(
    cookie: Cookie,
    crypto: Option<Crypto>,
) -> Result<(Value, Cookie), Box<dyn std::error::Error>> {
    let data = json!({});
    let resp = login_request(
        API.to_string(),
        data,
        Some(cookie),
        crypto.unwrap_or(Crypto::Api),
    )
    .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_login_status() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_status("114514".to_string(), Some(Crypto::Api)).await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_login_status() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_status("114514".to_string(), Some(Crypto::Linuxapi)).await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_login_status() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_status("114514".to_string(), Some(Crypto::Weapi)).await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_login_status() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_status("114514".to_string(), Some(Crypto::Eapi)).await?;
        println!("{}", resp.0);

        Ok(())
    }
}
