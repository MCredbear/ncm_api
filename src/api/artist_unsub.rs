use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "v1/artist/songs/unsub";

/// 取消收藏歌手
pub async fn artist_unsub(
    id: u32,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "artistId": id,
        "artistIds": format!("[{}]", id),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_artist_sub() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_unsub(30229793, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_sub() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_unsub(30229793, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_sub() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_unsub(30229793, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_sub() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_unsub(30229793, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
