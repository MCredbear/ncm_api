use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "search/get";

pub async fn search(
    keywords: String,
    search_type: Option<u32>,
    limit: Option<u32>,
    offset: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
      "s": keywords,
      "type": search_type.unwrap_or(1), // 1: 单曲, 10: 专辑, 100: 歌手, 1000: 歌单, 1002: 用户, 1004: MV, 1006: 歌词, 1009: 电台, 1014: 视频
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
        let resp = search(
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
        let resp = search(
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
        let resp = search(
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
        let resp = search(
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
