use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "user/playlist";

/// 用户歌单
pub async fn user_playlist(
    uid: u32,
    limit: Option<u32>,
    offset: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "uid": uid,
        "limit": limit.unwrap_or(30),
        "offset": offset.unwrap_or(0),
        "includeVideo": true,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_playlist() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_playlist(123456, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_playlist() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_playlist(123456, None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_playlist() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_playlist(123456, None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_playlist() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_playlist(123456, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
