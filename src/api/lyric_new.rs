use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "song/lyric/v1";

/// 新版歌词 - 包含逐字歌词
pub async fn lyric_new(
    id: u32,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "id": id,
        "cp": false,
        "tv": 0,
        "lv": 0,
        "rv": 0,
        "kv": 0,
        "yv": 0,
        "ytv": 0,
        "yrv": 0,
    });

    let resp = request(
        API.to_string(),
        data,
        cookie,
        crypto.unwrap_or(Crypto::Eapi),
    )
    .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_lyric_new() -> Result<(), Box<dyn std::error::Error>> {
        let resp = lyric_new(1386011473, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_lyric_new() -> Result<(), Box<dyn std::error::Error>> {
        let resp = lyric_new(1386011473, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_lyric_new() -> Result<(), Box<dyn std::error::Error>> {
        let resp = lyric_new(1386011473, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_lyric_new() -> Result<(), Box<dyn std::error::Error>> {
        let resp = lyric_new(1386011473, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
