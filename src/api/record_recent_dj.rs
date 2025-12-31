use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "play-record/djradio/list";

/// 最近播放电台
pub async fn record_recent_dj(
    limit: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "limit": limit.unwrap_or(100),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_record_recent_dj() -> Result<(), Box<dyn std::error::Error>> {
        let resp = record_recent_dj(None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_record_recent_dj() -> Result<(), Box<dyn std::error::Error>> {
        let resp = record_recent_dj(None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_record_recent_dj() -> Result<(), Box<dyn std::error::Error>> {
        let resp = record_recent_dj(None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_record_recent_dj() -> Result<(), Box<dyn std::error::Error>> {
        let resp = record_recent_dj(None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
