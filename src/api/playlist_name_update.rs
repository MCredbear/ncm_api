use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "playlist/update/name";

/// 更新歌单名
pub async fn playlist_name_update(
    id: u32,
    name: String,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "id": id,
        "name": name,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_playlist_name_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_name_update(1919, "YJSP".into(), None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_name_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_name_update(1919, "YJSP".into(), None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_name_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_name_update(1919, "YJSP".into(), None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_name_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_name_update(1919, "YJSP".into(), None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
