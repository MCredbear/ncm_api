use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "discovery/newAlbum";

/// 最新专辑
pub async fn album_newest(
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = request(
        API.to_string(),
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
    async fn test_api_album_newest() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_newest(None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_album_newest() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_newest(None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_album_newest() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_newest(None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_album_newest() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_newest(None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
