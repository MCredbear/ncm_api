use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "user/replaceCellphone";

/// 更换绑定手机
pub async fn user_replacephone(
    phone: String,
    captcha: String,
    oldcaptcha: String,
    countrycode: Option<String>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "phone": phone,
        "captcha": captcha,
        "oldcaptcha": oldcaptcha,
        "countrycode": countrycode.unwrap_or_else(|| "86".to_string()),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_replacephone() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_replacephone(
            "123".into(),
            "1111".into(),
            "2222".into(),
            None,
            None,
            Some(Crypto::Api),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_replacephone() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_replacephone(
            "123".into(),
            "1111".into(),
            "2222".into(),
            None,
            None,
            Some(Crypto::Linuxapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_replacephone() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_replacephone(
            "123".into(),
            "1111".into(),
            "2222".into(),
            None,
            None,
            Some(Crypto::Weapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_replacephone() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_replacephone(
            "123".into(),
            "1111".into(),
            "2222".into(),
            None,
            None,
            Some(Crypto::Eapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
}
