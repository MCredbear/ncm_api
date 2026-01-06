use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "playlist/tags/update";

/// 编辑歌单
pub async fn playlist_update(
    id: String,
    name: String,
    desc: Option<String>,
    tags: Option<String>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "/api/playlist/desc/update": format!(r#"{{"id":{},"desc":"{}"}}"#, id, desc.unwrap_or_default()),
        "/api/playlist/tags/update": format!(r#"{{"id":{},"tags":"{}"}}"#, id, tags.unwrap_or_default()),
        "/api/playlist/update/name": format!(r#"{{"id":{},"name":"{}"}}"#, id, name),
    });
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_playlist_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_update(
            "123".into(),
            "歌单名".into(),
            None,
            None,
            None,
            Some(Crypto::Api),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_update(
            "123".into(),
            "歌单名".into(),
            None,
            None,
            None,
            Some(Crypto::Linuxapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_update(
            "123".into(),
            "歌单名".into(),
            None,
            None,
            None,
            Some(Crypto::Weapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_update(
            "123".into(),
            "歌单名".into(),
            None,
            None,
            None,
            Some(Crypto::Eapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
}
