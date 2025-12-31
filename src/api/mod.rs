use crate::constant::*;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::{Value, json};

pub mod album;
pub mod album_detail;
pub mod album_detail_dynamic;
pub mod album_list;
pub mod album_list_style;
pub mod album_new;
pub mod album_newest;
pub mod album_songsaleboard;
pub mod album_sub;
pub mod album_sublist;
pub mod album_unsub;
pub mod artist_album;
pub mod artist_desc;
pub mod artist_detail;
pub mod artist_fans;
pub mod artist_follow_count;
pub mod artist_list;
pub mod artist_mv;
pub mod artist_new_mv;
pub mod artist_new_song;
pub mod artist_songs;
pub mod artist_sub;
pub mod artist_top_song;
pub mod artist_unsub;
pub mod artist_video;
pub mod artists;
pub mod banner;
pub mod captcha_sent;
pub mod captcha_verify;
pub mod cellphone_existence_check;
pub mod cloudsearch;
pub mod login;
pub mod login_cellphone;
pub mod login_qr_check;
pub mod login_qr_key;
pub mod login_refresh;
pub mod login_status;
pub mod logout;
pub mod lyric;
pub mod lyric_new;
pub mod recommend_resource;
pub mod recommend_songs;
pub mod recommend_songs_dislike;
pub mod record_recent_album;
pub mod record_recent_dj;
pub mod record_recent_playlist;
pub mod record_recent_song;
pub mod record_recent_video;
pub mod record_recent_voice;
pub mod search;
pub mod search_defailt;
pub mod search_voice;
pub mod user_account;
pub mod user_audio;
pub mod user_binding;
pub mod user_bindingcellphone;
pub mod user_cloud;
pub mod user_cloud_del;
pub mod user_cloud_detail;
pub mod user_comment_history;
pub mod user_detail;
pub mod user_dj;
pub mod user_event;
pub mod user_followeds;
pub mod user_follows;
pub mod user_level;
pub mod user_playlist;
pub mod user_record;
pub mod user_replacephone;
pub mod user_subcount;
pub mod user_update;

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
    cookie: Option<Cookie>,
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
                .header(
                    "Cookie",
                    cookie.unwrap_or("NMTID=; MUSIC_U=; __remember_me=true; os=pc".to_string()),
                )
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
