use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "playlist/video/recent";

pub async fn playlist_video_recent(
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = request(
        API.to_string(),
        serde_json::json!({}),
        cookie,
        crypto.unwrap_or(Crypto::Api),
    )
    .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_playlist_video_recent() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_video_recent(None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_video_recent() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_video_recent(None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_video_recent() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_video_recent(None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_video_recent() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_video_recent(None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
