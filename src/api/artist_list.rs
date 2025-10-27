use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "v1/artist/list";

pub enum ArtistType {
    Male,
    Female,
    Band,
}

pub enum Area {
    All,
    Chinese,
    Western,
    Japanese,
    Korean,
    Other,
}

/// 歌手分类
/// `initial`: a-z/A-Z
pub async fn artist_list(
    initial: Option<char>,
    artist_type: Option<ArtistType>,
    area: Option<Area>,
    offset: Option<u32>,
    limit: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut data = json!({
        "type": match  artist_type.unwrap_or(ArtistType::Male) {
            ArtistType::Male => 1,
            ArtistType::Female => 2,
            ArtistType::Band => 3,
        },
        "area": match area.unwrap_or(Area::All) {
            Area::All => -1,
            Area::Chinese => 7,
            Area::Western => 96,
            Area::Japanese => 8,
            Area::Korean => 16,
            Area::Other => 0,
        },
        "offset": offset.unwrap_or(0),
        "limit": limit.unwrap_or(30),
        "total": true,
    });
    if let Some(initial) = initial {
        if initial.is_ascii_alphabetic() {
            data["initial"] = json!(initial.to_ascii_uppercase() as u8);
        }
    }

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_artist_list() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_list(None, None, None, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_artist_list() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_list(None, None, None, None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_artist_list() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_list(None, None, None, None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_artist_list() -> Result<(), Box<dyn std::error::Error>> {
        let resp = artist_list(None, None, None, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
