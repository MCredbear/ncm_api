use super::{Cookie, request};
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "cellphone/existence/check";

/// 检测手机号码是否已注册
pub async fn cellphone_existence_check(
    phone: String,
    countrycode: Option<String>,
    cookie: Option<Cookie>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({
        "cellphone": phone,
        "countrycode": countrycode.unwrap_or_else(|| "86".to_string()),
    });

    let resp = request(
        API.to_string(),
        data,
        cookie,
        crypto.unwrap_or(Crypto::Eapi),
    )
    .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_cellphone_existence_check() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            cellphone_existence_check("11451419198".into(), None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_cellphone_existence_check() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            cellphone_existence_check("11451419198".into(), None, None, Some(Crypto::Linuxapi))
                .await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_cellphone_existence_check() -> Result<(), Box<dyn std::error::Error>> {
        let resp = cellphone_existence_check("11451419198".into(), None, None, Some(Crypto::Weapi))
            .await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_cellphone_existence_check() -> Result<(), Box<dyn std::error::Error>> {
        let resp =
            cellphone_existence_check("11451419198".into(), None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
