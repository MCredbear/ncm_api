use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "v1/cloud/get/byids";

/// 云盘数据详情
pub async fn user_cloud_detail(
    ids: Vec<String>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "songIds": ids,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_cloud_detail() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            user_cloud_detail(vec!["123".into(), "456".into()], None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_cloud_detail() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_cloud_detail(vec!["123".into()], None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_cloud_detail() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_cloud_detail(vec!["123".into()], None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_cloud_detail() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_cloud_detail(vec!["123".into()], None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
