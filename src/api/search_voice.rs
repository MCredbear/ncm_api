use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "search/voice/get";

pub async fn search_voice(
    keywords: String,
    scene: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
      "keyword": keywords,
      "scene": scene.unwrap_or("normal".to_string()),
      "limit": limit.unwrap_or(30),
      "offset": offset.unwrap_or(0),
    });
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_search() -> Result<(), Box<dyn std::error::Error>> {
        let resp = search_voice(
            "risa yuzuki".into(),
            None,
            None,
            None,
            None,
            Some(Crypto::Api),
        )
        .await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_search() -> Result<(), Box<dyn std::error::Error>> {
        let resp = search_voice(
            "risa yuzuki".into(),
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
    async fn test_weapi_search() -> Result<(), Box<dyn std::error::Error>> {
        let resp = search_voice(
            "risa yuzuki".into(),
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
    async fn test_eapi_search() -> Result<(), Box<dyn std::error::Error>> {
        let resp = search_voice(
            "risa yuzuki".into(),
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
