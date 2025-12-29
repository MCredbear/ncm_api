use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "v2/discovery/recommend/dislike";

/// 每日推荐歌曲-不感兴趣
pub async fn recommend_songs_dislike(
    id: u32,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "resId": id,
        "resType": 4,
        "sceneType": 1,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_recommend_songs_dislike() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_songs_dislike(114514, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_recommend_songs_dislike() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_songs_dislike(114514, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_recommend_songs_dislike() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_songs_dislike(114514, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_recommend_songs_dislike() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_songs_dislike(114514, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
