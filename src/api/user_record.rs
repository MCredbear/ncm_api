use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "v1/play/record";

pub enum RecordType {
    All = 0,
    Week = 1,
}

/// 听歌排行
pub async fn user_record(
    uid: u32,
    record_type: Option<RecordType>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "uid": uid,
        "type": record_type.unwrap_or(RecordType::All) as u8,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_record() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_record(123456, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_record() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_record(123456, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_record() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_record(123456, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_record() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_record(123456, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
