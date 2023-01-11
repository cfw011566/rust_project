use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose, Engine as _};
use rand::prelude::*;
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Security {
    key: String,
    iv: String,
}

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

const AES256_KEY_SIZE: usize = 32;
const AES128_BLOCK_SIZE: usize = 16;

static PUBLIC_KEY: &str = r#"
-----BEGIN PUBLIC KEY-----
MIIBITANBgkqhkiG9w0BAQEFAAOCAQ4AMIIBCQKCAQBOlo1/7ilwWwvpBpD221wr
xyMdfDZhQoWOXDJjpbFGlRf1XSkTmooDmW3kaXvYwbOnW54LvGX4nUX7qVk8pP9v
oMjorRBYCIhBb6AqnKrsrQGvFSlQretjO+RyfCzo9vK9WBFvmgOLUgvedYya3X+h
ei5HFkw+wESD+Jyy02FaAB4/DQ5kYYPG4K8QbBT/0h4vsHTV0FIbqp6n1CWhT+vW
jF3Z+CSKUdd1l5KjnU6vyJa7SktBT3/oOIr/aFuuUXzA5WbKD1FwTUuD6Y7r5LAy
RIMbcELGbc3PQHa2xPvk5WbusTaGQp2UXJNR8Yld0Fh6qsizNs1+9Te8MPR9VsNJ
AgMBAAE=
-----END PUBLIC KEY-----
"#;

pub fn generate_key_iv() -> String {
    let mut rng = rand::thread_rng();
    // generate key & iv
    // AES256 key size
    let key_data: [u8; AES256_KEY_SIZE] = rng.gen();
    // AES128 block size
    let iv_data: [u8; AES128_BLOCK_SIZE] = rng.gen();

    std::fs::write("key.bin", key_data).unwrap();
    std::fs::write("iv.bin", iv_data).unwrap();

    let key_base64 = general_purpose::STANDARD.encode(key_data);
    let iv_base64 = general_purpose::STANDARD.encode(iv_data);
    let security = Security {
        key: key_base64,
        iv: iv_base64,
    };
    let serialized_security = serde_json::to_string(&security).unwrap();
    //    println!("security json = {}", serialized_security);
    let public_key = RsaPublicKey::from_public_key_pem(PUBLIC_KEY).unwrap();
    //    println!("public key = {:?}", public_key);

    //    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let padding = PaddingScheme::new_oaep::<sha1::Sha1>();
    let security_enc_data = public_key
        .encrypt(&mut rng, padding, serialized_security.as_bytes())
        .expect("failed to encrypt");
    let security_base64 = general_purpose::STANDARD.encode(security_enc_data);

    security_base64
}

pub fn encrypt(clear_text: &str) -> String {
    let key_data = fs::read("key.bin").unwrap();
    let iv_data = fs::read("iv.bin").unwrap();
    let key_buf = &key_data[..32];
    let iv_buf = &iv_data[..16];
    let buf = clear_text.as_bytes();
    let cipher_data =
        Aes256CbcEnc::new(key_buf.into(), iv_buf.into()).encrypt_padded_vec_mut::<Pkcs7>(buf);
    //    println!("cipher data = {:?}", cipher_data);
    //    println!("cipher data len = {}", cipher_data.len());
    let cipher_base64 = general_purpose::STANDARD.encode(cipher_data);
    // println!("cipher = {}", cipher_base64);
    cipher_base64
}

pub fn decrypt(cipher_base64: &str) -> String {
    let key_data = fs::read("key.bin").unwrap();
    let iv_data = fs::read("iv.bin").unwrap();
    let key_buf = &key_data[..32];
    let iv_buf = &iv_data[..16];

    let cipher_data = general_purpose::STANDARD.decode(cipher_base64).unwrap();
    let buf = cipher_data.as_slice();
    let plain_data = Aes256CbcDec::new(key_buf.into(), iv_buf.into())
        .decrypt_padded_vec_mut::<Pkcs7>(buf)
        .unwrap();
    let plain_text = String::from_utf8_lossy(&plain_data);
    //    println!("plain_text = {}", plain_text);
    plain_text.to_string()
}
