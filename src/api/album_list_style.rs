use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "vipmall/appalbum/album/style";

pub enum Area {
    Zh,
    Ea,
    Kr,
    Jp,
}

/// 数字专辑-语种风格馆
pub async fn album_list_style(
    limit: Option<u32>,
    offset: Option<u32>,
    area: Option<Area>, // Zh: 华语, Ea: 欧美, Kr: 韩国, Jp: 日本
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let area = match area.unwrap_or(Area::Zh) {
        Area::Zh => "Z_H",
        Area::Ea => "E_A",
        Area::Kr => "KR",
        Area::Jp => "JP",
    };
    let data = json!({
        "limit": limit.unwrap_or(10),
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
    async fn test_api_album_list_style() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_list_style(None, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_album_list_style() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_list_style(None, None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_album_list_style() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_list_style(None, None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_album_list_style() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_list_style(None, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
