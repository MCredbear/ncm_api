use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "album/sub";

/// 收藏专辑
pub async fn album_sub(
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
    async fn test_api_album_sub() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_sub(81099298, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_album_sub() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_sub(81099298, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_album_sub() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_sub(81099298, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_album_sub() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_sub(81099298, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
