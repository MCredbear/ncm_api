use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "playlist/detail/dynamic";

/// 歌单详情动态
pub async fn playlist_detail_dynamic(
    id: u32,
    s: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "id": id,
        "n": 100000,
        "s": s.unwrap_or(8),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_playlist_detail_dynamic() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_detail_dynamic(114514, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_detail_dynamic() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_detail_dynamic(114514, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_detail_dynamic() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_detail_dynamic(114514, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_detail_dynamic() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_detail_dynamic(114514, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
