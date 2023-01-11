#[tokio::main]
async fn main() {
    /*
    if let Err(e) = api::get_args().and_then(api::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    */
    match api::get_args() {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(cli) => api::run(cli).await,
    }
}
