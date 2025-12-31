use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "comment/user/comment/history";

/// 用户评论历史
pub async fn user_comment_history(
    uid: u64,
    limit: Option<u32>,
    time: Option<u64>,
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
        "compose_reminder": "true",
        "compose_hot_comment": "true",
        "limit": limit.unwrap_or(10),
        "user_id": uid,
        "time": time.unwrap_or(0),
    });

    let resp = request(
        API.to_string(),
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
    async fn test_api_user_comment_history() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_comment_history(123456, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_comment_history() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_comment_history(123456, None, None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_comment_history() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_comment_history(123456, None, None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_comment_history() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_comment_history(123456, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
