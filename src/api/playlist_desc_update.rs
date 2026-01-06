use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "playlist/desc/update";

/// 更新歌单描述
pub async fn playlist_desc_update(
    id: u32,
    desc: String,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "id": id,
        "desc": desc,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_playlist_desc_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_desc_update(1919, "yjsp".into(), None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_desc_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_desc_update(1919, "yjsp".into(), None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_desc_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_desc_update(1919, "yjsp".into(), None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_desc_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_desc_update(1919, "yjsp".into(), None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
