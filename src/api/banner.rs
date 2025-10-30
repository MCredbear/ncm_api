use super::request;
use crate::constant::*;
use serde_json::{Value, json};

const API: &str = "v2/banner/get";

pub enum BannerType {
    Pc,
    Android,
    IPhone,
    IPad,
}

/// 首页轮播图
pub async fn banner(
    banner_type: Option<BannerType>,
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let client_type = match banner_type.unwrap_or(BannerType::Pc) {
        BannerType::Pc => "pc",
        BannerType::Android => "android",
        BannerType::IPhone => "iphone",
        BannerType::IPad => "ipad",
    };

    let data = json!({
        "clientType": client_type,
    });

    let resp = request(API.to_string(), data, cookie, crypto.unwrap_or(Crypto::Api)).await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_banner() -> Result<(), Box<dyn std::error::Error>> {
        let resp = banner(None, None, Some(Crypto::Api)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_banner() -> Result<(), Box<dyn std::error::Error>> {
        let resp = banner(None, None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_banner() -> Result<(), Box<dyn std::error::Error>> {
        let resp = banner(None, None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_banner() -> Result<(), Box<dyn std::error::Error>> {
        let resp = banner(None, None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);

        Ok(())
    }
}
