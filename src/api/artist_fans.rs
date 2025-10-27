use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "artist/fans/get";

/// 歌手粉丝
pub async fn artist_fans(
    id: String,
    limit: Option<u32>,
    offset: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "id": id,
        "limit": limit.unwrap_or(20),
        "offset": offset.unwrap_or(0),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_artist_fans() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_fans("30229793".into(), None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_fans() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_fans("30229793".into(), None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_fans() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_fans("30229793".into(), None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_fans() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_fans("30229793".into(), None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
