use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "v1/user/bindings";

/// 用户绑定信息
pub async fn user_binding(
    uid: u64,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = request(
        format!("{}/{}", API, uid),
        serde_json::json!({}),
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
    async fn test_api_user_binding() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_binding(123456, None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_linuxapi_user_binding() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_binding(123456, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_weapi_user_binding() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_binding(123456, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_eapi_user_binding() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_binding(123456, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
