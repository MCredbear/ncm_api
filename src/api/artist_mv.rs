use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "artist/mvs";

/// 歌手相关 MV
pub async fn artist_mv(
    artist_id: String,
    limit: Option<u32>,
    offset: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "artistId": artist_id,
        "limit": limit.unwrap_or(30),
        "offset": offset.unwrap_or(0),
        "total": true,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_artist_mv() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_mv("30229793".into(), None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_mv() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_mv("30229793".into(), None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_mv() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_mv("30229793".into(), None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_mv() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_mv("30229793".into(), None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
