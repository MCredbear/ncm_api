use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "artist/top/song";

/// 歌手热门 50 首歌曲
pub async fn artist_top_song(
    id: String,
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
    async fn test_api_artist_top_song() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_top_song("30229793".into(), None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_top_song() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_top_song("30229793".into(), None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_top_song() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_top_song("30229793".into(), None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_top_song() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_top_song("30229793".into(), None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
