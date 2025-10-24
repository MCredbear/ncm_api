use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "v1/user/detail/";

/// 用户详情
pub async fn user_detail(
    uid: u32,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({});
    let resp = request(
        format!("{}{}", API.to_string(), uid),
        data,
        cookie,
        crypto.unwrap_or(Crypto::Api),
    )
    .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_detail() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_detail(114514, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_user_detail() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_detail(114514, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_user_detail() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_detail(114514, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_user_detail() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_detail(114514, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
