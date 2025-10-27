use super::request;
use crate::constant::*;
use chrono::{DateTime, Utc};
use serde_json::{Value, json};

const API: &str = "sub/artist/new/works/mv/list";

pub async fn artist_new_mv(
    limit: Option<u32>,
    before: Option<DateTime<Utc>>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "limit": limit.unwrap_or(20),
        "startTimestamp": before.unwrap_or_else(|| chrono::Utc::now()).timestamp_millis(),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_artist_new_mv() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_new_mv(None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_new_mv() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_new_mv(None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_new_mv() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_new_mv(None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_new_mv() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_new_mv(None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
