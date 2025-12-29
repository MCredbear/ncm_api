use super::request;
use crate::constant::*;
use serde_json::Value;

const API: &str = "v1/discovery/recommend/resource";

/// 每日推荐歌单
pub async fn recommend_resource(
    cookie: Option<String>,
    crypto: Option<Crypto>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = request(
        API.to_string(),
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
    async fn test_api_recommend_resource() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_resource(None, Some(Crypto::Api)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_linuxapi_recommend_resource() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_resource(None, Some(Crypto::Linuxapi)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_weapi_recommend_resource() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_resource(None, Some(Crypto::Weapi)).await?;
        println!("{}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_eapi_recommend_resource() -> Result<(), Box<dyn std::error::Error>> {
        let resp = recommend_resource(None, Some(Crypto::Eapi)).await?;
        println!("{}", resp);
        Ok(())
    }
}
