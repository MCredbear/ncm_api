use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "user/profile/update";

/// 编辑用户信息
pub async fn user_update(
    birthday: Option<u64>,
    city: Option<u32>,
    gender: Option<u8>,
    nickname: Option<String>,
    province: Option<u32>,
    signature: Option<String>,
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
        // "avatarImgId": "0",
        "birthday": birthday,
        "city": city,
        "gender": gender,
        "nickname": nickname,
        "province": province,
        "signature": signature,
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
    async fn test_api_user_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_update(None, None, None, None, None, None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_update(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Crypto::Linuxapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_update(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Crypto::Weapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_update() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            user_update(None, None, None, None, None, None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
