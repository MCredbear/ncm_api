use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "mlog/playlist/mylike/bytime/get";

pub async fn playlist_mylike(
    time: Option<i64>,
    limit: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "time": time.unwrap_or(-1),
        "limit": limit.unwrap_or(12),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_playlist_mylike() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_mylike(None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_mylike() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_mylike(None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_mylike() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_mylike(None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_mylike() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_mylike(None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
