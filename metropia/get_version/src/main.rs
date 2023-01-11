use clap::Parser;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
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
    payload: Option<PathBuf>,

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

static URL_PREFIX: &str = "https://api.appstoreconnect.apple.com";

#[derive(Debug, Serialize, Deserialize)]
struct Attributes {
    #[serde(rename = "appStoreState")]
    app_store_state: String,
    copyright: String,
    #[serde(rename = "createdDate")]
    created_date: String,
    downloadable: bool,
    #[serde(rename = "earliestReleaseDate")]
    earliest_release_date: Option<String>,
    platform: String,
    #[serde(rename = "releaseType")]
    release_type: String,
    #[serde(rename = "usesIdfa")]
    uses_idfa: Option<String>,
    #[serde(rename = "versionString")]
    version_string: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = read_config_from_file(&args.config).unwrap();
    println!("Config = {:?}", config);

    let token = jwt_token(&config).unwrap();
    println!("Bearer = {}", token);

    //    let url = format!("{URL_PREFIX}/v1/apps/{}/appStoreVersions", "1529532077");
    let url = format!(
        "{URL_PREFIX}/v1/apps/{}/appStoreVersions?filter[appStoreState]=READY_FOR_SALE",
        "1529532077"
    );

    let client = reqwest::Client::new();
    let resp = client.get(url).bearer_auth(&token).send().await.unwrap();
    let body = resp.text().await.unwrap();

    //    println!("{:#?}", body);

    let val: Value = serde_json::from_str(&body).unwrap();
    println!("{}", val);
    let v_data = &val["data"];
    //    println!("t = {}", v_data[0]);
    let v_array = v_data.as_array().unwrap();
    for v in v_array {
        if let Ok(att) = serde_json::from_value::<Attributes>(v["attributes"].clone()) {
            /*
            if att.app_store_state == "READY_FOR_SALE" {
                println!("Ready for Sale version = {}", att.version_string);
            }
            */
            println!("Ready for Sale version = {}", att.version_string);
        }
    }
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
    let claims = json!({
        "iss": "69a6de7f-b2f6-47e3-e053-5b8c7c11a4d1",
        "iat": epoch,
        "exp": epoch+300,
        "aud": "appstoreconnect-v1"
    });
    //        "scope": [ "GET /v1/apps/appStoreVersions/1529532077?filter[appStoreState]=READY_FOR_SALE" ]

    let mut header = Header::new(Algorithm::ES256);
    header.typ = Some("JWT".to_string());
    header.kid = Some(config.key_id.clone());
    let private_key =
        fs::read_to_string(&config.authkey_file).expect("Something went wrong reading the file");
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_ec_pem(private_key.as_bytes()).unwrap(),
    )?;
    Ok(token)
}
