fn main() {
    if let Err(e) = api::get_args().and_then(api::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
