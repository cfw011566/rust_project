use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::prelude::*;
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Favorite {
    id: usize,
    category: usize,
    icon_type: usize,
    name: String,
    address: String,
    latitude: f64,
    longitude: f64,
    access_latitude: Option<f64>,
    access_longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Security {
    key: String,
    iv: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cipher {
    security_key: String,
    cipher: String,
}

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

static URL_PREFIX: &str = "https://dev-portal.connectsmartx.com";
const AES256_KEY_SIZE: usize = 32;
const AES128_BLOCK_SIZE: usize = 16;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    // generate key & iv
    // AES256 key size
    let key_data: [u8; AES256_KEY_SIZE] = rng.gen();
    // AES128 block size
    let iv_data: [u8; AES128_BLOCK_SIZE] = rng.gen();

    let pub_key = fs::read_to_string("pub_key.pem").expect("Something went wrong reading the file");

    // openid login
    let client = reqwest::Client::new();
    //    let url = format!("{URL_PREFIX}/api/v1/openid_login/gplus");
    //    let (security_base64, cipher_base64) = login_gplus_payload(&key_data, &iv_data, &pub_key);
    let url = format!("{URL_PREFIX}/api/v2/login");
    let (security_base64, cipher_base64) = login_email_payload(&key_data, &iv_data, &pub_key);
    println!("security = {}", security_base64);
    println!("cipher = {}", cipher_base64);
    let cipher_json = Cipher {
        security_key: security_base64,
        cipher: cipher_base64,
    };
    let pay_load = serde_json::to_value(cipher_json).unwrap();
    println!("{:?}", pay_load);
    let res = client.post(url).json(&pay_load).send().await?;
    //    println!("{:#?}", res);
    println!("Response: {:?} {}", res.version(), res.status());
    //    println!("Headers: {:#?}\n", res.headers());
    let headers = res.headers();
    let metropia_token = if let Some(token) = headers.get("access-token") {
        format!("{}", token.to_str().unwrap_or(""))
    } else {
        "".to_string()
    };
    println!("Token = {}", metropia_token);

    let body = res.text().await?;
    let v: Value = serde_json::from_str(&body)?;

    // Access parts of the data by indexing with square brackets.
    println!("result = {}", v["result"]);
    println!("data = {}", v["data"]);
    /*
    println!("data[id] = {}", v["data"]["id"]);
    let data = &v["data"];
    let id = &data["id"];
    if let Some(mut x) = id.as_u64() {
        x = x + 1;
        println!("id + 1 = {}", x);
    }
    */

    let cipher_base64 = &v["data"]["cipher"];
    if let Some(cipher_base64) = cipher_base64.as_str() {
        let cipher_data = base64::decode(cipher_base64).unwrap();
        let buf = cipher_data.as_slice();
        let plain_data = Aes256CbcDec::new(&key_data.into(), &iv_data.into())
            .decrypt_padded_vec_mut::<Pkcs7>(buf)
            .unwrap();
        //        println!("plain_data = {:?}", plain_data);
        let plain_text = String::from_utf8_lossy(&plain_data);
        println!("plain_text = {}", plain_text);

        let login_json: Value = serde_json::from_str(&plain_text)?;
        println!("id = {}", login_json["id"]);
        let id = &login_json["id"];
        if let Some(mut x) = id.as_u64() {
            x += 1;
            println!("id + 1 = {}", x);
        }
    }

    // get profile
    let client = reqwest::Client::new();
    let url = format!("{URL_PREFIX}/api/v1/profile");
    let resp = client.get(url).bearer_auth(&metropia_token).send().await?;
    let body = resp.text().await?;
    let v: Value = serde_json::from_str(&body)?;
    println!("result = {}", v["result"]);
    println!("data = {}", v["data"]);
    let cipher_base64 = &v["data"]["cipher"];
    if let Some(cipher_base64) = cipher_base64.as_str() {
        let cipher_data = base64::decode(cipher_base64).unwrap();
        let buf = cipher_data.as_slice();
        let plain_data = Aes256CbcDec::new(&key_data.into(), &iv_data.into())
            .decrypt_padded_vec_mut::<Pkcs7>(buf)
            .unwrap();
        //        println!("plain_data = {:?}", plain_data);
        let plain_text = String::from_utf8_lossy(&plain_data);
        println!("plain_text = {}", plain_text);

        let profile_json: Value = serde_json::from_str(&plain_text)?;
        println!("rating = {}", profile_json["rating"]);
    }

    // get reservation today
    let client = reqwest::Client::new();
    let url = format!("{URL_PREFIX}/api/v2/reservation?travel_mode=1,2,3,4,5,6,100&is_today=true");
    let resp = client.get(url).bearer_auth(&metropia_token).send().await?;
    let body = resp.text().await?;
    let v: Value = serde_json::from_str(&body)?;
    println!("result = {}", v["result"]);
    println!("data = {}", v["data"]);
    let cipher_base64 = &v["data"]["cipher"];
    if let Some(cipher_base64) = cipher_base64.as_str() {
        let cipher_data = base64::decode(cipher_base64).unwrap();
        let buf = cipher_data.as_slice();
        let plain_data = Aes256CbcDec::new(&key_data.into(), &iv_data.into())
            .decrypt_padded_vec_mut::<Pkcs7>(buf)
            .unwrap();
        //        println!("plain_data = {:?}", plain_data);
        let plain_text = String::from_utf8_lossy(&plain_data);
        println!("plain_text = {}", plain_text);

        let reservation_json: Value = serde_json::from_str(&plain_text)?;
        println!(
            "{}",
            serde_json::to_string_pretty(&reservation_json).unwrap()
        );
        //        println!("rating = {}", profile_json["rating"]);
    }

    // get favorites
    // get_favorites(&metropia_token).await?;

    Ok(())
}

async fn get_favorites(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{URL_PREFIX}/api/v1/favorites");
    let resp = client.get(url).bearer_auth(token).send().await?;
    let body = resp.text().await?;
    //    println!("favorites = {}", body);
    let v: Value = serde_json::from_str(&body)?;
    let v_str = serde_json::to_string(&v["data"]["favorites"]).unwrap();
    //    println!("v_str = {}", v_str);
    let favorites: Vec<Favorite> = serde_json::from_str(&v_str)?;
    println!("favorites = {:#?}", favorites);

    Ok(())
}

fn login_email_payload(
    key_buf: &[u8; 32],
    iv_buf: &[u8; 16],
    crypto_pub_key: &str,
) -> (String, String) {
    let mut rng = rand::thread_rng();

    // AES256 key size
    //    let key_buf: [u8; 32] = rng.gen();
    // AES128 block size
    //    let iv_buf: [u8; 16] = rng.gen();

    // build { "key": key_base64, "iv": iv_base64 }
    let key_base64 = base64::encode(key_buf);
    let iv_base64 = base64::encode(iv_buf);
    //    println!("{:?}", key_buf);
    //    println!("{:?}", iv_buf);
    //    println!("key = {}, iv = {}", key_base64, iv_base64);
    let security = Security {
        key: key_base64,
        iv: iv_base64,
    };
    let serialized_security = serde_json::to_string(&security).unwrap();
    //    println!("security json = {}", serialized_security);
    let public_key = RsaPublicKey::from_public_key_pem(crypto_pub_key).unwrap();
    //    println!("public key = {:?}", public_key);

    //    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let padding = PaddingScheme::new_oaep::<sha1::Sha1>();
    let security_enc_data = public_key
        .encrypt(&mut rng, padding, serialized_security.as_bytes())
        .expect("failed to encrypt");
    let security_base64 = base64::encode(security_enc_data);
    //    println!("seucurity = {}", security_base64);

    /* test data */
    let account_data = r#"
{
    "email": "cf.wang+1@metropia.com",
    "password": "Ab123456"
}"#;
    let account_json: Value = serde_json::from_str(account_data).unwrap();
    let serialized_account = account_json.to_string();
    //    println!("account = {}", serialized_account);
    //    println!("account len = {}", serialized_account.len());
    /*
    let mut buf = [0u8; 256];
    let pt_len = serialized_account.len();
    buf[..pt_len].copy_from_slice(&serialized_account.as_bytes());
    let cipher_data = Aes256CbcEnc::new(key_buf.into(), iv_buf.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();
    */
    let buf = serialized_account.as_bytes();
    let cipher_data =
        Aes256CbcEnc::new(key_buf.into(), iv_buf.into()).encrypt_padded_vec_mut::<Pkcs7>(buf);
    //    println!("cipher data = {:?}", cipher_data);
    //    println!("cipher data len = {}", cipher_data.len());
    let cipher_base64 = base64::encode(cipher_data);
    //    println!("cipher = {}", cipher_base64);

    (security_base64, cipher_base64)
}

fn login_gplus_payload(
    key_buf: &[u8; 32],
    iv_buf: &[u8; 16],
    crypto_pub_key: &str,
) -> (String, String) {
    let mut rng = rand::thread_rng();

    // AES256 key size
    //    let key_buf: [u8; 32] = rng.gen();
    // AES128 block size
    //    let iv_buf: [u8; 16] = rng.gen();

    // build { "key": key_base64, "iv": iv_base64 }
    let key_base64 = base64::encode(key_buf);
    let iv_base64 = base64::encode(iv_buf);
    //    println!("{:?}", key_buf);
    //    println!("{:?}", iv_buf);
    //    println!("key = {}, iv = {}", key_base64, iv_base64);
    let security = Security {
        key: key_base64,
        iv: iv_base64,
    };
    let serialized_security = serde_json::to_string(&security).unwrap();
    //    println!("security json = {}", serialized_security);
    let public_key = RsaPublicKey::from_public_key_pem(crypto_pub_key).unwrap();
    //    println!("public key = {:?}", public_key);

    //    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let padding = PaddingScheme::new_oaep::<sha1::Sha1>();
    let security_enc_data = public_key
        .encrypt(&mut rng, padding, serialized_security.as_bytes())
        .expect("failed to encrypt");
    let security_base64 = base64::encode(security_enc_data);
    //    println!("seucurity = {}", security_base64);

    /* test data */
    let account_data = r#"
{
    "longitude": 121.5571042,
    "email": "cf.wang@metropia.com",
    "first_name": "Ching-Feng",
    "id": "118093826874656609939",
    "latitude": 25.0363898,
    "last_name": "Wang"
}"#;
    let account_json: Value = serde_json::from_str(account_data).unwrap();
    let serialized_account = account_json.to_string();
    //    println!("account = {}", serialized_account);
    //    println!("account len = {}", serialized_account.len());
    /*
    let mut buf = [0u8; 256];
    let pt_len = serialized_account.len();
    buf[..pt_len].copy_from_slice(&serialized_account.as_bytes());
    let cipher_data = Aes256CbcEnc::new(key_buf.into(), iv_buf.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();
    */
    let buf = serialized_account.as_bytes();
    let cipher_data =
        Aes256CbcEnc::new(key_buf.into(), iv_buf.into()).encrypt_padded_vec_mut::<Pkcs7>(buf);
    //    println!("cipher data = {:?}", cipher_data);
    //    println!("cipher data len = {}", cipher_data.len());
    let cipher_base64 = base64::encode(cipher_data);
    //    println!("cipher = {}", cipher_base64);

    (security_base64, cipher_base64)
}
