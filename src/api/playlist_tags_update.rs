use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "playlist/tags/update";

/// 更新歌单标签
pub async fn playlist_tags_update(
    id: String,
    tags: String,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "id": id,
        "tags": tags,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_playlist_tags_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            playlist_tags_update("123".into(), "流行".into(), None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_tags_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            playlist_tags_update("123".into(), "流行".into(), None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_tags_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            playlist_tags_update("123".into(), "流行".into(), None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_tags_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            playlist_tags_update("123".into(), "流行".into(), None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
