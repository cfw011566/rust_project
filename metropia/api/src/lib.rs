use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::fs::File;
use std::io::{BufRead, BufReader};

// type MyResult<T> = Result<T, Box<dyn Error>>;

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
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(cli: Cli) -> Result<()> {
    println!("{:?}", cli);
    let _file =
        open(&cli.config).with_context(|| format!("counld not open file `{}`", cli.config))?;
    Ok(())
}
