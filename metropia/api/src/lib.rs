use anyhow::{Context, Ok, Result};
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::io::{BufRead, BufReader};

mod cipher;

/// Metropia API by Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "api")]
#[command(author = "CF Wang <cf.wang@metropia.com>")]
#[command(version = "1.0")]
#[command(about = "Metropia API command line tool", long_about = None)]
pub struct Cli {
    /// Configuration file
    #[arg(default_value = "config.json")]
    config: String,

    /// Verbose
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// register with first name, last name, email and password
    Register(Register),

    /// login with email and password
    Login { email: String, password: String },

    /// get profile
    Profile,

    /// get favorites
    Favorites,

    /// get reservation
    Reservations,

    /// query with email
    Carpool,
}

#[derive(Args, Debug)]
struct Register {
    /// email
    email: String,
    /// password
    password: String,
    /// first name
    #[arg(short, long)]
    first_name: Option<String>,
    /// last name
    #[arg(short, long, default_value = "")]
    last_name: String,
}

pub fn get_args() -> Result<Cli> {
    let cli = Cli::parse();
    Ok(cli)
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(filename)?))),
    }
}

pub async fn run(cli: Cli) {
    println!("{:?}", cli);
    let _file = open(&cli.config)
        .with_context(|| format!("counld not open file `{}`", cli.config))
        .unwrap();
    match &cli.command {
        Commands::Login { email, password } => {
            login(email, password).await.unwrap();
        }
        Commands::Profile => {
            get_profile().await.unwrap();
        }
        Commands::Favorites => {
            get_favorites().await.unwrap();
        }
        Commands::Reservations => {
            get_reservations().await.unwrap();
        }
        Commands::Carpool => {
            carpool();
        }
        _ => println!("{:?} not supported yet", &cli.command),
    }
}

fn carpool() {
    let client = reqwest::Client::builder().build().unwrap();
    let req = client
        .get("https://www.google.com")
        .query(&[("foo", "bar")])
        .query(&[("test", " space*star:common=equal")])
        .query(&[("email", "test+3@example.com")])
        .build()
        .unwrap();
    println!("url = {}", req.url());

    let mut req = client.get("https://www.google.com").build().unwrap();
    let query: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("foo", "bar")
        .append_pair("test", " space*star:common=equal")
        .append_pair("email", "test+3@example.com")
        .finish();
    println!("query = {}", query);
    req.url_mut().set_query(Some(&query));
    println!("url = {}", req.url());
}

static URL_PREFIX: &str = "https://dev-portal.connectsmartx.com";

async fn login(email: &str, password: &str) -> Result<()> {
    println!("login with email {} and password {}", email, password);
    let account_json = json!({
        "email": email,
        "password": password
    });
    let serialized_account = account_json.to_string();

    let security_base64 = cipher::generate_key_iv();
    let cipher_base64 = cipher::encrypt(&serialized_account);

    let client = reqwest::Client::new();
    let url = format!("{URL_PREFIX}/api/v2/login");
    let pay_load = json!({
        "security_key": security_base64,
        "cipher": cipher_base64
    });
    println!("{:?}", pay_load);
    let res = client.post(url).json(&pay_load).send().await?;
    //    println!("{:#?}", res);
    println!("Response: {:?} {}", res.version(), res.status());
    //    println!("Headers: {:#?}\n", res.headers());
    let headers = res.headers();
    let metropia_token = if let Some(token) = headers.get("access-token") {
        token.to_str().unwrap_or("").to_string()
    } else {
        "".to_string()
    };
    println!("Token = {}", metropia_token);
    std::fs::write("token.jwt", metropia_token).unwrap();
    let body = res.text().await?;
    let v: Value = serde_json::from_str(&body)?;

    // Access parts of the data by indexing with square brackets.
    println!("result = {}", v["result"]);
    println!("data = {}", v["data"]);

    let cipher_base64 = &v["data"]["cipher"];
    if let Some(cipher_base64) = cipher_base64.as_str() {
        let plain_text = cipher::decrypt(cipher_base64);
        let login_json: Value = serde_json::from_str(&plain_text)?;
        println!("id = {}", login_json["id"]);
        let id = &login_json["id"];
        if let Some(mut x) = id.as_u64() {
            x += 1;
            println!("id + 1 = {}", x);
        }
    }

    Ok(())
}

async fn get_profile() -> Result<()> {
    let token = std::fs::read_to_string("token.jwt").unwrap();
    let client = reqwest::Client::new();
    let url = format!("{URL_PREFIX}/api/v1/profile");
    let resp = client.get(url).bearer_auth(&token).send().await?;
    let body = resp.text().await?;
    let v: Value = serde_json::from_str(&body)?;
    println!("result = {}", v["result"]);
    println!("data = {}", v["data"]);
    let cipher_base64 = &v["data"]["cipher"];
    if let Some(cipher_base64) = cipher_base64.as_str() {
        let plain_text = cipher::decrypt(cipher_base64);
        println!("plain_text = {}", plain_text);

        let profile_json: Value = serde_json::from_str(&plain_text)?;
        println!("rating = {}", profile_json["rating"]);
    }
    Ok(())
}

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

async fn get_favorites() -> Result<()> {
    let token = std::fs::read_to_string("token.jwt").unwrap();
    let client = reqwest::Client::new();
    let url = format!("{URL_PREFIX}/api/v1/favorites");
    let resp = client.get(url).bearer_auth(&token).send().await.unwrap();
    let body = resp.text().await.unwrap();
    //    println!("favorites = {}", body);
    let v: Value = serde_json::from_str(&body).unwrap();
    let v_str = serde_json::to_string(&v["data"]["favorites"]).unwrap();
    //    println!("v_str = {}", v_str);
    let favorites: Vec<Favorite> = serde_json::from_str(&v_str).unwrap();
    println!("favorites = {:#?}", favorites);
    Ok(())
}

async fn get_reservations() -> Result<()> {
    let token = std::fs::read_to_string("token.jwt").unwrap();
    let client = reqwest::Client::new();
    let url = format!("{URL_PREFIX}/api/v2/reservation?travel_mode=1,2,3,4,5,6,100&is_today=true");
    let resp = client.get(url).bearer_auth(&token).send().await?;
    let body = resp.text().await?;
    let v: Value = serde_json::from_str(&body)?;
    println!("result = {}", v["result"]);
    println!("data = {}", v["data"]);
    let cipher_base64 = &v["data"]["cipher"];
    if let Some(cipher_base64) = cipher_base64.as_str() {
        let plain_text = cipher::decrypt(cipher_base64);
        println!("plain_text = {}", plain_text);

        let reservation_json: Value = serde_json::from_str(&plain_text)?;
        println!(
            "{}",
            serde_json::to_string_pretty(&reservation_json).unwrap()
        );
    }

    Ok(())
}
