use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "album/new";

pub enum Area {
    All,
    Zh,
    Ea,
    Kr,
    Jp,
}

/// 全部新碟
pub async fn album_new(
    limit: Option<u32>,
    offset: Option<u32>,
    area: Option<Area>, // All: 全部, Zh: 华语, Ea: 欧美, Kr: 韩国, Jp: 日本
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let area = match area.unwrap_or(Area::All) {
        Area::All => "ALL",
        Area::Zh => "ZH",
        Area::Ea => "EA",
        Area::Kr => "KR",
        Area::Jp => "JP",
    };
    let data = json!({
        "limit": limit.unwrap_or(30),
        "offset": offset.unwrap_or(0),
        "total": true,
        "area": area,
    });
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_album_new() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_new(None, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_album_new() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_new(None, None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_album_new() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_new(None, None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_album_new() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_new(None, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
