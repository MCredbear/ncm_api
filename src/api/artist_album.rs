use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "artist/albums";

/// 歌手专辑列表
pub async fn artist_album(
    id: String,
    offset: Option<u32>,
    limit: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "offset": offset.unwrap_or(0),
        "limit": limit.unwrap_or(100),
        "total": true,
    });
    let resp = request(
        format!("{}/{}", API, id),
        data,
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
    async fn test_api_artist_album() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_album("30229793".into(), None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_album() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            artist_album("30229793".into(), None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_album() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_album("30229793".into(), None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_album() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_album("30229793".into(), None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
