mod http_request;

fn main() {
    let req = http_request::HttpRequest::new();
    println!("HTTP Request: {:?}", req.method);
}
