use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "user/bindingCellphone";

/// 绑定手机
pub async fn user_bindingcellphone(
    phone: String,
    countrycode: Option<String>,
    captcha: Option<String>,
    password: Option<String>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "phone": phone,
        "countrycode": countrycode.unwrap_or_else(|| "86".to_string()),
        "captcha": captcha,
        "password": password.unwrap_or_default(),
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_bindingcellphone() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_bindingcellphone(
            "12345678901".into(),
            None,
            None,
            None,
            None,
            Some(Crypto::Api),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_bindingcellphone() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_bindingcellphone(
            "12345678901".into(),
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
    async fn test_weapi_user_bindingcellphone() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_bindingcellphone(
            "12345678901".into(),
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
    async fn test_eapi_user_bindingcellphone() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_bindingcellphone(
            "12345678901".into(),
            None,
            None,
            None,
            None,
            Some(Crypto::Eapi),
        )
        .await?;
        println!("{}", resp);
        Ok(())
    }
}
