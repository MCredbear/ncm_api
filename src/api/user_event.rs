use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "event/get";

/// 用户动态
pub async fn user_event(
    uid: u32,
    lasttime: Option<i64>,
    limit: Option<u32>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut cookie = cookie.unwrap_or_default();
    if !cookie.contains("os=ios") {
        cookie.push_str("; os=ios");
    }
    if !cookie.contains("appver=8.20.21") {
        cookie.push_str("; appver=8.20.21");
    }
    let data = json!({
        "getcounts": true,
        "time": lasttime.unwrap_or(-1),
        "limit": limit.unwrap_or(30),
        "total": false,
    });

    let resp = request(
        format!("{}/{}", API, uid),
        data,
        Some(cookie),
        crypto.unwrap_or(Crypto::Api),
    )
    .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_event() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_event(123456, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_event() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_event(123456, None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_event() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_event(123456, None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_event() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_event(123456, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
