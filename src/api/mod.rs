use crate::constant::*;
use once_cell::sync::Lazy;
use serde_json::{Value, json};
use reqwest::Client;

pub mod album;
pub mod album_detail;
pub mod album_detail_dynamic;
pub mod album_list;
pub mod album_list_style;
pub mod album_new;
pub mod album_newest;
pub mod album_songsaleboard;
pub mod album_sub;
pub mod album_unsub;
pub mod album_sublist;
pub mod captcha_sent;
pub mod captcha_verify;
pub mod cloudsearch;
pub mod login;
pub mod login_cellphone;
pub mod lyric;
pub mod search;
pub mod search_defailt;
pub mod search_voice;
pub mod user_account;
pub mod user_detail;

type Cookie = String;

static HTTP_CLIENT: Lazy<Client> =
    Lazy::new(|| Client::builder().pool_max_idle_per_host(4).build().unwrap());

async fn request(
    api: String,
    data: serde_json::Value,
    cookie: Option<String>,
    crypto: Crypto,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let resp = match crypto {
        Crypto::Api => {
            HTTP_CLIENT
                .get(format!(
                    "{}{}?{}",
                    API_END_POINT,
                    api,
                    serde_urlencoded::to_string(data)?
                ))
                .header("Cookie", cookie.unwrap_or_default())
                .send()
                .await?
                .json::<Value>()
                .await?
        }
        Crypto::Weapi => {
            let (params, enc_sec_key) = crate::crypto::weapi(&data);
            let data = serde_urlencoded::to_string(&json!({
                "params": params,
                "encSecKey": enc_sec_key,
            }))?;
            HTTP_CLIENT
                .post(format!("{}{}", WEAPI_END_POINT, api))
                .body(data)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Cookie", cookie.unwrap_or_default())
                .send()
                .await?
                .json::<Value>()
                .await?
        }
        Crypto::Eapi => {
            let params = crate::crypto::eapi(format!("/api/{}", api).as_str(), &data);
            let data = serde_urlencoded::to_string(&json!({
                "params": params,
            }))?;
            HTTP_CLIENT
                .post(format!("{}{}", EAPI_END_POINT, api))
                .body(data)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Cookie", cookie.unwrap_or_default())
                .send()
                .await?
                .json::<Value>()
                .await?
        }
        Crypto::Linuxapi => {
            let eparams = crate::crypto::linuxapi(&json!({
                "method": "POST",
                "url": format!("{}{}", API_END_POINT, api),
                "params": data,
            }));
            let data = serde_urlencoded::to_string(&json!({
                "eparams": eparams,
            }))?;
            HTTP_CLIENT
                .post(LINUXAPI_END_POINT)
                .body(data)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Cookie", cookie.unwrap_or_default())
                .send()
                .await?
                .json::<Value>()
                .await?
        }
    };

    Ok(resp)
}

async fn login_request(
    api: String,
    data: serde_json::Value,
    crypto: Crypto,
) -> Result<(serde_json::Value, Cookie), Box<dyn std::error::Error>> {
    let resp = match crypto {
        Crypto::Api => {
            HTTP_CLIENT
                .get(format!(
                    "{}{}?{}",
                    API_END_POINT,
                    api,
                    serde_urlencoded::to_string(data)?
                ))
                .header("Cookie", "NMTID=; MUSIC_U=; __remember_me=true; os=pc")
                .send()
                .await?
        }
        Crypto::Weapi => {
            let (params, enc_sec_key) = crate::crypto::weapi(&data);
            let data = serde_urlencoded::to_string(&json!({
                "params": params,
                "encSecKey": enc_sec_key,
            }))?;
            HTTP_CLIENT
                .post(format!("{}{}", WEAPI_END_POINT, api))
                .body(data)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Cookie", "NMTID=; MUSIC_U=; __remember_me=true; os=pc")
                .send()
                .await?
        }
        Crypto::Eapi => {
            let params = crate::crypto::eapi(format!("/api/{}", api).as_str(), &data);
            let data = serde_urlencoded::to_string(&json!({
                "params": params,
            }))?;
            HTTP_CLIENT
                .post(format!("{}{}", EAPI_END_POINT, api))
                .body(data)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Cookie", "NMTID=; MUSIC_U=; __remember_me=true; os=pc")
                .send()
                .await?
        }
        Crypto::Linuxapi => {
            let eparams = crate::crypto::linuxapi(&json!({
                "method": "POST",
                "url": format!("{}{}", API_END_POINT, api),
                "params": data,
            }));
            let data = serde_urlencoded::to_string(&json!({
                "eparams": eparams,
            }))?;
            HTTP_CLIENT
                .post(LINUXAPI_END_POINT)
                .body(data)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Cookie", "NMTID=; MUSIC_U=; __remember_me=true; os=pc")
                .send()
                .await?
        }
    };

    let cookie = resp
        .headers()
        .get_all("Set-Cookie")
        .iter()
        .map(|v| v.to_str().unwrap().split(';').next().unwrap().trim())
        .collect::<Vec<&str>>()
        .join("; ");
    Ok((resp.json::<Value>().await?, cookie))
}
