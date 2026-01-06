use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "playlist/highquality/tags";

/// 精品歌单 tags
pub async fn playlist_highquality_tags(
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
    async fn test_api_playlist_highquality_tags() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_highquality_tags(None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_playlist_highquality_tags() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_highquality_tags(None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_playlist_highquality_tags() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_highquality_tags(None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_playlist_highquality_tags() -> Result<(), Box<dyn std::error::Error>> {
        let resp = playlist_highquality_tags(None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
