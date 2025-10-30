use super::{Cookie, login_request};
use crate::constant::*;
use md5;
use serde_json::{Value, json};

const API: &str = "login/cellphone";

/// 手机密码登录
pub async fn login_cellphone_with_password(
    phone: String,
    countrycode: Option<String>,
    password: String,
    crypto: Option<Crypto>,
) -> Result<(Value, Cookie), Box<dyn std::error::Error>> {
    let data = json!({
      "phone": phone,
      "countrycode": countrycode.unwrap_or("86".to_string()),
      "password": format!("{:x}", md5::compute(password.as_bytes())),
      "rememberLogin": "true",
    });
    let resp = login_request(API.to_string(), data, None, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

/// 手机验证码登录
pub async fn login_cellphone_with_captcha(
    phone: String,
    countrycode: Option<String>,
    captcha: String,
    crypto: Option<Crypto>,
) -> Result<(Value, String), Box<dyn std::error::Error>> {
    let data = json!({
      "phone": phone,
      "countrycode": countrycode.unwrap_or("86".to_string()),
      "captcha": captcha,
      "rememberLogin": "true",
    });
    let resp = login_request(API.to_string(), data, None, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_login_cellphone_with_password() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_cellphone_with_password(
            "1145141919810".to_string(),
            None,
            "1145141919810".to_string(),
            Some(Crypto::Api),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_api_login_cellphone_with_captcha() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_cellphone_with_captcha(
            "1145141919810".to_string(),
            None,
            "114514".to_string(),
            Some(Crypto::Api),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_login_with_password() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_cellphone_with_password(
            "1145141919810".to_string(),
            None,
            "1145141919810".to_string(),
            Some(Crypto::Linuxapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_login_with_captcha() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_cellphone_with_captcha(
            "1145141919810".to_string(),
            None,
            "114514".to_string(),
            Some(Crypto::Linuxapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_login_with_password() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_cellphone_with_password(
            "1145141919810".to_string(),
            None,
            "1145141919810".to_string(),
            Some(Crypto::Weapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_login_with_captcha() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_cellphone_with_captcha(
            "1145141919810".to_string(),
            None,
            "114514".to_string(),
            Some(Crypto::Weapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_login_with_password() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_cellphone_with_password(
            "1145141919810".to_string(),
            None,
            "1145141919810".to_string(),
            Some(Crypto::Eapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_login_with_captcha() -> Result<(), Box<dyn std::error::Error>> {
        let resp = login_cellphone_with_captcha(
            "1145141919810".to_string(),
            None,
            "114514".to_string(),
            Some(Crypto::Eapi),
        )
        .await?;
        println!("{}", resp.0);

        Ok(())
    }
}
