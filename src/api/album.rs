use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "v1/album";

/// 专辑内容
pub async fn album(
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
    async fn test_api_album() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album(81099298, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_album() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album(81099298, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_album() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album(81099298, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_album() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album(81099298, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
