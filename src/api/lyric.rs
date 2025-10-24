use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "song/lyric";

pub async fn lyric(
    id: u32,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
      "id": id,
      "tv": -1,
      "lv": -1,
      "rv": -1,
      "kv": -1,
    });
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_lyric() -> Result<(), Box<dyn std::error::Error>> {
        let resp = lyric(114514, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_lyric() -> Result<(), Box<dyn std::error::Error>> {
        let resp = lyric(114514, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_lyric() -> Result<(), Box<dyn std::error::Error>> {
        let resp = lyric(114514, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_lyric() -> Result<(), Box<dyn std::error::Error>> {
        let resp = lyric(114514, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
