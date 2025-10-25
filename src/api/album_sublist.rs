use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "album/sublist";

/// 已收藏专辑列表
pub async fn album_sublist(
    limit: Option<u32>,
    offset: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "limit": limit.unwrap_or(25),
        "offset": offset.unwrap_or(0),
        "total": true,
    });
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_album_sublist() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_sublist(None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_album_sublist() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_sublist(None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_album_sublist() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_sublist(None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_album_sublist() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_sublist(None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
