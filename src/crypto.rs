use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit, KeyIvInit, block_padding::Pkcs7};
use base64::Engine;
use md5;
use num_bigint::BigUint;
use rand::{Rng, distr::Alphanumeric};
use rsa::{RsaPublicKey, pkcs8::DecodePublicKey, traits::PublicKeyParts};
use serde_json::Value;
use std::str;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

const IV: &str = "0102030405060708";
const PRESET_KEY: &str = "0CoJUm6Qyw8W8jud";
const LINUX_API_KEY: &str = "rFgB&h#%2?^eDg:Q";
const PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ3
7BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvakl
V8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44o
ncaTWz7OBGLbCiK45wIDAQAB
-----END PUBLIC KEY-----";
const EAPI_KEY: &str = "e82ckenh8dichen8";

enum AesMode {
    Cbc,
    Ecb,
}

fn aes_encrypt(text: &str, mode: &AesMode, key: &str, iv: &str) -> Vec<u8> {
    let mut buffer = text.as_bytes().to_vec();
    let key = key.as_bytes();
    let iv = iv.as_bytes();
    let pos = buffer.len();
    buffer.extend_from_slice(&[0; 16][..16 - pos % 16]);

    match mode {
        AesMode::Cbc => {
            let cipher = Aes128CbcEnc::new(key.into(), iv.into());
            cipher
                .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
                .unwrap()
                .to_vec()
        }
        AesMode::Ecb => {
            let cipher = Aes128EcbEnc::new(key.into());
            cipher
                .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
                .unwrap()
                .to_vec()
        }
    }
}

fn rsa_encrypt(text: &str, public_key: &str) -> String {
    let public_key = RsaPublicKey::from_public_key_pem(public_key).unwrap();
    let m = BigUint::from_bytes_be(text.as_bytes());
    let c = m.modpow(
        &num_bigint::BigUint::from_bytes_be(&public_key.e().to_bytes_be()),
        &num_bigint::BigUint::from_bytes_be(&public_key.n().to_bytes_be()),
    );
    let encrypted = c.to_bytes_be();
    hex::encode(encrypted)
}

pub fn weapi(object: &Value) -> (String, String) {
    let text = serde_json::to_string(object).unwrap();
    let secret_key: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    let params = base64::engine::general_purpose::STANDARD.encode(aes_encrypt(
        &base64::engine::general_purpose::STANDARD.encode(aes_encrypt(
            &text,
            &AesMode::Cbc,
            PRESET_KEY,
            IV,
        )),
        &AesMode::Cbc,
        &secret_key,
        IV,
    ));
    let enc_sec_key = rsa_encrypt(&secret_key.chars().rev().collect::<String>(), PUBLIC_KEY);
    (params, enc_sec_key)
}

pub fn linuxapi(object: &Value) -> String {
    let text = serde_json::to_string(object).unwrap();
    println!("linuxapi text: {}", text);
    hex::encode_upper(aes_encrypt(&text, &AesMode::Ecb, LINUX_API_KEY, ""))
}

pub fn eapi(url: &str, object: &Value) -> String {
    let text = if object.is_object() {
        serde_json::to_string(object).unwrap()
    } else {
        object.as_str().unwrap().to_string()
    };
    let message = format!("nobody{}use{}md5forencrypt", url, text);
    let digest = format!("{:x}", md5::compute(message.as_bytes()));
    let data = format!("{}-36cd479b6b5-{}-36cd479b6b5-{}", url, text, digest);

    hex::encode_upper(aes_encrypt(&data, &AesMode::Ecb, EAPI_KEY, ""))
}

fn decrypt(cipher: &mut str) -> String {
    let mut buffer = cipher.as_bytes().to_vec();
    let decipher = Aes128EcbDec::new(EAPI_KEY.as_bytes().into());
    let decrypted = decipher.decrypt_padded_mut::<Pkcs7>(&mut buffer).unwrap();
    str::from_utf8(decrypted).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_weapi() {
        let data = json!({"id": "114514"});
        println!("{}", data);
        let (params, enc_sec_key) = weapi(&data);
        println!("params: {}", params);
        println!("encSecKey: {}", enc_sec_key);
    }

    #[test]
    fn test_linuxapi() {
        let data = json!({"id": "114514"});
        let result = linuxapi(&data);
        assert!(result == "1955BCAC5477605A86DE37CEF37813A8");
    }

    #[test]
    fn test_eapi() {
        let data = json!({"id": "114514"});
        let result = eapi("/yjsp", &data);
        assert!(
            result
                == "806D58DAE28649A6A1C560EE629387D4C223F814330F202D4990245599399A4681C80B18BFA7DED82F26130A7528CAFD8B43215993187748E417901E315D6F94DC58140A5C7BB6A0003F09BAAA3DD663"
        );
    }
}
