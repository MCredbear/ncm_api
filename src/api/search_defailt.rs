use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "search/defaultkeyword/get";

/// 默认搜索关键词
pub async fn search_default(
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({});
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_search() -> Result<(), Box<dyn std::error::Error>> {
        let resp = search_default(None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_search() -> Result<(), Box<dyn std::error::Error>> {
        let resp = search_default(None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_search() -> Result<(), Box<dyn std::error::Error>> {
        let resp = search_default(None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_search() -> Result<(), Box<dyn std::error::Error>> {
        let resp = search_default(None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
