use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "mlog/artist/video";

/// 歌手相关视频
pub async fn artist_video(
    artist_id: String,
    size: Option<u32>,
    cursor: Option<u64>,
    order: Option<u8>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "artistId": artist_id,
        "page": json!({
            "size": size.unwrap_or(10),
            "cursor": cursor.unwrap_or(0),
        }).to_string(),
        "tab": 0,
        "order": order.unwrap_or(0),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_artist_video() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            artist_video("30229793".into(), None, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_video() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_video(
            "30229793".into(),
            None,
            None,
            None,
            None,
            Some(Crypto::Linuxapi),
        )
        .await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_video() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_video(
            "30229793".into(),
            None,
            None,
            None,
            None,
            Some(Crypto::Weapi),
        )
        .await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_video() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_video(
            "30229793".into(),
            None,
            None,
            None,
            None,
            Some(Crypto::Eapi),
        )
        .await?;
        println!("{}", resp);

        Ok(())
    }
}
