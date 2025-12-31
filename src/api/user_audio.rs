use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "djradio/get/byuser";

/// 用户创建的电台
pub async fn user_audio(
    uid: u64,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "userId": uid,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_audio() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_audio(123456, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_audio() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_audio(123456, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_audio() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_audio(123456, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_audio() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_audio(123456, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
