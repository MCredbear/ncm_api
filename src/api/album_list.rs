use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "vipmall/albumproduct/list";

pub enum Area {
    All,
    Zh,
    Ea,
    Kr,
    Jp,
}

/// 数字专辑-新碟上架
pub async fn album_list(
    limit: Option<u32>,
    offset: Option<u32>,
    area: Option<Area>, // All: 全部, Zh: 华语, Ea: 欧美, Kr: 韩国, Jp: 日本
    album_type: Option<u32>,
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
        "type": album_type.unwrap_or(0),
    });
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_album_list() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_list(None, None, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_album_list() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_list(None, None, None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_album_list() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_list(None, None, None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_album_list() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_list(None, None, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
