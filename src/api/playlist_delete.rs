use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "playlist/remove";

/// 删除歌单
pub async fn playlist_delete(
    ids: Vec<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "ids": ids,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_playlist_delete() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_delete(vec![114514], None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_delete() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_delete(vec![114514], None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_delete() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_delete(vec![114514], None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_delete() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_delete(vec![114514], None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
