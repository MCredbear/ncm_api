use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "feealbum/songsaleboard";

pub enum BoardType {
    Daily,
    Week,
    Year,
    Total,
}

/// 数字专辑&数字单曲-榜单
pub async fn album_songsaleboard(
    album_type: Option<u8>, // 0: 数字专辑, 1: 数字单曲
    board_type: Option<BoardType>,
    year: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut data = json!({
        "albumType": album_type.unwrap_or(0),
    });

    let board_type = match board_type.unwrap_or(BoardType::Daily) {
        BoardType::Daily => "daily",
        BoardType::Week => "week",
        BoardType::Year => {
            if let Some(y) = year {
                data["year"] = json!(y);
            }
            "year"
        }
        BoardType::Total => "total",
    };

    let resp = request(
        format!("{}/{}/type", API, board_type),
        data,
        cookie,
        crypto.unwrap_or(Crypto::Api),
    )
    .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_album_songsaleboard() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_songsaleboard(None, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_album_songsaleboard() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_songsaleboard(None, None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_album_songsaleboard() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_songsaleboard(None, None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_album_songsaleboard() -> Result<(), Box<dyn std::error::Error>> {
        let resp = album_songsaleboard(None, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
