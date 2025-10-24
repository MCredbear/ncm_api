use super::{Cookie, login_request};
use crate::constant::*;
use md5;
use serde_json::{Value, json};

const API: &str = "login";

/// 邮箱登录
pub async fn login(
    username: String,
    password: String,
    crypto: Option<Crypto>,
) -> Result<(Value, Cookie), Box<dyn std::error::Error>> {
    let data = json!({
      "username": username,
      "password": format!("{:x}", md5::compute(password.as_bytes())),
      "rememberLogin": "true",
    });
    let resp = login_request(API.to_string(), data, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_login() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login(
            "114514@yjsp.com".to_string(),
            "1145141919810".to_string(),
            Some(Crypto::Api),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_login() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login(
            "114514@yjsp.com".to_string(),
            "1145141919810".to_string(),
            Some(Crypto::Linuxapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_login() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login(
            "114514@yjsp.com".to_string(),
            "1145141919810".to_string(),
            Some(Crypto::Weapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_login() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login(
            "114514@yjsp.com".to_string(),
            "1145141919810".to_string(),
            Some(Crypto::Eapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }
}
