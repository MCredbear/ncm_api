use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "artist/introduction";

/// 歌手介绍
pub async fn artist_desc(
    id: u32,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "id": id,
    });
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_artist_desc() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_desc(30229793, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_desc() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_desc(30229793, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_desc() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_desc(30229793, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_desc() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_desc(30229793, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
