use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "nuser/account/get";

pub async fn user_account(
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data = json!({});
    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_user_account() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_account(None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_user_account() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_account(None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_user_account() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_account(None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_user_account() -> Result<(), Box<dyn std::error::Error>> {
        let resp = user_account(None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
