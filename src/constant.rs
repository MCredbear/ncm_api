pub const API_END_POINT: &str = "https://music.163.com/api/";
pub const WEAPI_END_POINT: &str = "https://music.163.com/weapi/";
pub const EAPI_END_POINT: &str = "https://music.163.com/eapi/";
pub const LINUXAPI_END_POINT: &str = "https://music.163.com/api/linux/forward";

pub enum Crypto {
    Api,
    Weapi,
    Eapi,
    Linuxapi,
}
