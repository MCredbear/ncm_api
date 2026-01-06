use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "playlist/create";

pub enum Privacy {
    Public = 0,
    Private = 10,
}

pub enum PlaylistType {
    Normal,
    Video,
    Shared,
}

impl PlaylistType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlaylistType::Normal => "NORMAL",
            PlaylistType::Video => "VIDEO",
            PlaylistType::Shared => "SHARED",
        }
    }
}

/// 创建歌单
pub async fn playlist_create(
    name: String,
    privacy: Option<Privacy>, // Public: 普通歌单, Private: 隐私歌单
    playlist_type: Option<PlaylistType>, // Normal|Video|Shared
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "name": name,
        "privacy": privacy.unwrap_or(Privacy::Public) as u8,
        "type": playlist_type.unwrap_or(PlaylistType::Normal).as_str(),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_playlist_create() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_create("YYUT".into(), None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_create() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_create("YYUT".into(), None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_create() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_create("YYUT".into(), None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_create() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_create("YYUT".into(), None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
