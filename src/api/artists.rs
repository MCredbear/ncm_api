use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "v1/artist";

/// 歌手单曲
pub async fn artists(
    id: u32,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = request(
        format!("{}/{}", API, id),
        serde_json::json!({}),
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
    async fn test_api_artists() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artists(30229793, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artists() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artists(30229793, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artists() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artists(30229793, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artists() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artists(30229793, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
