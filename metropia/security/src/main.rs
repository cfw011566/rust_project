use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::*;
use rand::prelude::*;
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

fn main() {
    let (security, cipher) = login_payload();
    println!("security = {}", security);
    println!("cipher = {}", cipher);
}

fn login_payload() -> (String, String) {
    let mut rng = rand::thread_rng();

    // generate key & iv
    // AES256 key size
    let key_buf: [u8; 32] = rng.gen();
    // AES128 block size
    let iv_buf: [u8; 16] = rng.gen();

    // build { "key": key_base64, "iv": iv_base64 }
    let key_base64 = encode(key_buf);
    let iv_base64 = encode(iv_buf);
    //    println!("{:?}", key_buf);
    //    println!("{:?}", iv_buf);
    //    println!("key = {}, iv = {}", key_base64, iv_base64);
    let security = Security {
        key: key_base64,
        iv: iv_base64,
    };
    let serialized_security = serde_json::to_string(&security).unwrap();
    //    println!("security json = {}", serialized_security);

    let crypto_pubkey = r#"-----BEGIN PUBLIC KEY-----
MIIBITANBgkqhkiG9w0BAQEFAAOCAQ4AMIIBCQKCAQBOlo1/7ilwWwvpBpD221wr
xyMdfDZhQoWOXDJjpbFGlRf1XSkTmooDmW3kaXvYwbOnW54LvGX4nUX7qVk8pP9v
oMjorRBYCIhBb6AqnKrsrQGvFSlQretjO+RyfCzo9vK9WBFvmgOLUgvedYya3X+h
ei5HFkw+wESD+Jyy02FaAB4/DQ5kYYPG4K8QbBT/0h4vsHTV0FIbqp6n1CWhT+vW
jF3Z+CSKUdd1l5KjnU6vyJa7SktBT3/oOIr/aFuuUXzA5WbKD1FwTUuD6Y7r5LAy
RIMbcELGbc3PQHa2xPvk5WbusTaGQp2UXJNR8Yld0Fh6qsizNs1+9Te8MPR9VsNJ
AgMBAAE=
-----END PUBLIC KEY-----"#;
    let public_key = RsaPublicKey::from_public_key_pem(crypto_pubkey).unwrap();
    //    println!("public key = {:?}", public_key);

    // let security_base64 = encrypt(securityJSON, getIdRsaPub());
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let security_enc_data = public_key
        .encrypt(&mut rng, padding, &serialized_security.as_bytes())
        .expect("failed to encrypt");
    let security_base64 = encode(security_enc_data);
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
    let mut buf = [0u8; 256];
    let pt_len = serialized_account.len();
    buf[..pt_len].copy_from_slice(&serialized_account.as_bytes());
    let cipher_data = Aes256CbcEnc::new(&key_buf.into(), &iv_buf.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();
    //    println!("cipher data = {:?}", cipher_data);
    //    println!("cipher data len = {}", cipher_data.len());
    let cipher_base64 = encode(cipher_data);
    //    println!("cipher = {}", cipher_base64);

    (security_base64, cipher_base64)
}
