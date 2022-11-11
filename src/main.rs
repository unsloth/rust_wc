fn main() {
    if let Err(e) = rust_wc::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
