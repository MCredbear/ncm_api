use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "user/level";

/// 获取用户等级信息
pub async fn user_level(
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = request(
        API.to_string(),
        serde_json::json!({}),
        cookie,
        crypto.unwrap_or(Crypto::Api),
    )
    .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_level() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_level(None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_level() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_level(None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_level() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_level(None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_level() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_level(None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
