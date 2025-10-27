use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "v1/artist/songs";

pub enum Order {
    Hot,
    Time,
}

/// 歌手单曲
pub async fn artist_songs(
    id: String,
    order: Option<Order>,
    offset: Option<u32>,
    limit: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "id": id,
        "private_cloud": true,
        "work_type": 1,
        "order": match order.unwrap_or(Order::Hot) {
            Order::Hot => "hot",
            Order::Time => "time",
        },
        "offset": offset.unwrap_or(0),
        "limit": limit.unwrap_or(100),
    });
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_artist_songs() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            artist_songs("30229793".into(), None, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_songs() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_songs(
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
    async fn test_weapi_artist_songs() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_songs(
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
    async fn test_eapi_artist_songs() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_songs(
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
