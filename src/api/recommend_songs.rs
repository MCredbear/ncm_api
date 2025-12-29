use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "v3/discovery/recommend/songs";

/// 每日推荐歌曲
pub async fn recommend_songs(
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let cookie = cookie.unwrap_or_default();

    let resp = request(
        API.to_string(),
        serde_json::json!({}),
        Some(cookie),
        crypto.unwrap_or(Crypto::Api),
    )
    .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_recommend_songs() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_songs(None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_recommend_songs() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_songs(None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_recommend_songs() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_songs(None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_recommend_songs() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_songs(None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
