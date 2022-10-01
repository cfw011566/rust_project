use clap::Parser;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Version;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path to the file of Push data in JSON format
    #[clap(value_parser)]
    payload: PathBuf,

    /// The path to the file of config
    #[clap(short, long, value_parser, value_name = "hcs_dev.conf")]
    config: PathBuf,

    /// Title
    #[clap(short, long, value_parser)]
    title: Option<String>,

    /// Body
    #[clap(short, long, value_parser)]
    body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    authkey_file: PathBuf,
    key_id: String,
    team_id: String,
    device_token: String,
    topic: String,
    is_production: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    iat: usize,
}

static URL_PREFIX: &str = "https://api.sandbox.push.apple.com";

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = read_config_from_file(&args.config).unwrap();
    println!("Config = {:?}", config);

    let token = jwt_token(&config).unwrap();
    println!("Bearer = {}", token);

    let payload = read_payload_from_file(&args.payload).unwrap();
    println!("Payload = {}", payload);

    let alert = apns_alert(&args);
    println!("alert = {}", alert);

    let apns_json: Value =
        json!({"aps": alert, "meta": payload, "message_id": 1234, "notification_type": 12});
    println!("APNS data = {}", apns_json);

    let url = format!("{URL_PREFIX}/3/device/{}", config.device_token);
    let mut builder = reqwest::Client::new().post(url);
    builder = builder.version(Version::HTTP_2);
    builder = builder.bearer_auth(token);
    builder = builder.header("apns-topic", config.topic);
    builder = builder.header("apns-push-type", "alert");
    builder = builder.header("apns-priority", 10);
    builder = builder.json(&apns_json);

    println!("{:#?}", builder);

    let res = builder.send().await.unwrap();

    println!("{:#?}", res);
    println!("Response: {:?} {}", res.version(), res.status());
}

fn read_config_from_file(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let config = serde_json::from_reader(reader)?;

    Ok(config)
}

fn jwt_token(config: &Config) -> Result<String, Box<dyn Error>> {
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
    )?;
    Ok(token)
}

fn read_payload_from_file(path: &PathBuf) -> Result<Value, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let payload = serde_json::from_reader(reader)?;

    Ok(payload)
}

fn apns_alert(args: &Args) -> Value {
    let mut title = "Title from Rust";
    if let Some(t) = &args.title {
        title = t
    }
    let mut body = "Body from Rust";
    if let Some(b) = &args.body {
        body = b
    }
    json!({ "alert": { "title": title, "body": body } })
}
