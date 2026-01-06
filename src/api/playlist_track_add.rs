use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "playlist/track/add";

/// 收藏单曲到歌单
pub async fn playlist_track_add(
    pid: u32,
    ids: Vec<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "pid": pid,
        "tracks": ids.iter().map(|id| {
        json!({
            "type": 3,
            "id": id,
        })
    }).collect::<Vec<Value>>()
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_playlist_track_add() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_track_add(114514, vec![114514], None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_track_add() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_track_add(114514, vec![114514], None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_track_add() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_track_add(114514, vec![114514], None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_track_add() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_track_add(114514, vec![114514], None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
