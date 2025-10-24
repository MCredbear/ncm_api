use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "sms/captcha/verify";

/// 校验验证码
pub async fn captcha_verify(
    cellphone: String,
    ctcode: Option<String>,
    captcha: String,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "cellphone": cellphone,
        "ctcode": ctcode.unwrap_or("86".to_string()),
        "captcha": captcha,
    });
    let resp = request(API.to_string(), data, None, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_captcha_verify() -> Result<(), Box<dyn std::error::Error>> {
        let resp = captcha_verify(
            "11451419198".to_string(),
            None,
            "114514".to_string(),
            Some(Crypto::Api),
        )
        .await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_captcha_verify() -> Result<(), Box<dyn std::error::Error>> {
        let resp = captcha_verify(
            "11451419198".to_string(),
            None,
            "114514".to_string(),
            Some(Crypto::Linuxapi),
        )
        .await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_captcha_verify() -> Result<(), Box<dyn std::error::Error>> {
        let resp = captcha_verify(
            "11451419198".to_string(),
            None,
            "114514".to_string(),
            Some(Crypto::Weapi),
        )
        .await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_captcha_verify() -> Result<(), Box<dyn std::error::Error>> {
        let resp = captcha_verify(
            "11451419198".to_string(),
            None,
            "114514".to_string(),
            Some(Crypto::Eapi),
        )
        .await?;
        println!("{}", resp);

        Ok(())
    }
}
