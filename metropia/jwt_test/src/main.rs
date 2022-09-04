use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
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
/*
{
   "alg" : "ES256",
   "kid" : "ABC123DEFG"
}
{
   "iss": "DEF123GHIJ",
   "iat": 1437179036
}
*/

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    authkey_file: String,
    key_id: String,
    team_id: String,
    device_token: String,
    topic: String,
    is_production: bool,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(config)
}

fn main() {
    let config = read_user_from_file("hcs_dev.json").unwrap();
    println!("{:#?}", config);

    let epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let issue_at = epoch.as_secs();
    println!("issue_at {}", issue_at);
    let my_claims = Claims {
        //        iat: issue_at as usize,
        iat: 1656164019,
        iss: config.team_id,
    };

    let mut header = Header::new(Algorithm::ES256);
    header.typ = None;
    header.kid = Some(config.key_id);
    let private_key =
        fs::read_to_string(config.authkey_file).expect("Something went wrong reading the file");
    let token = encode(
        &header,
        &my_claims,
        &EncodingKey::from_ec_pem(private_key.as_bytes()).unwrap(),
    )
    .unwrap();
    println!("token = {}", token)
}
