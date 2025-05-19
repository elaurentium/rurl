mod http_request;
mod resolve;


fn main() {
    if let Err(e) = http_request::http_request() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
