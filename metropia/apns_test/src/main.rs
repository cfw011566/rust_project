use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    authkey_file: String,
    key_id: String,
    team_id: String,
    device_token: String,
    topic: String,
    is_production: bool,
}

static URL_PREFIX: &str = "https://api.sandbox.push.apple.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config_from_file("hcs_dev.json").unwrap();
    println!("{:#?}", config);

    let token = jwt_token(&config);
    println!("bearer {}", token);

    let apns_data = r#"{ "aps" : { "alert" : "Hello"}, "message_id": "1234" }"#;
    let payload_json: Value = serde_json::from_str(apns_data).unwrap();

    let url = format!("{URL_PREFIX}/3/device/{}", config.device_token);
    let mut builder = reqwest::Client::new().post(url);
    builder = builder.version(Version::HTTP_2);
    builder = builder.bearer_auth(token);
    builder = builder.header("apns-topic", config.topic);
    builder = builder.header("apns-push-type", "alert");
    builder = builder.header("apns-priority", 10);
    builder = builder.json(&payload_json);

    println!("{:#?}", builder);

    let res = builder.send().await?;

    println!("{:#?}", res);
    println!("Response: {:?} {}", res.version(), res.status());

    Ok(())
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config = serde_json::from_reader(reader)?;

    // Return the `Config`.
    Ok(config)
}

fn jwt_token(config: &Config) -> String {
    let epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let my_claims = Claims {
        iat: epoch,
        iss: config.team_id.clone(),
    };

    let mut header = Header::new(Algorithm::ES256);
    header.typ = None;
    header.kid = Some(config.key_id.clone());
    let private_key =
        fs::read_to_string(&config.authkey_file).expect("Something went wrong reading the file");
    let token = encode(
        &header,
        &my_claims,
        &EncodingKey::from_ec_pem(private_key.as_bytes()).unwrap(),
    )
    .unwrap();
    token
}
