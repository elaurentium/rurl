#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub host: String,
    pub body: Option<String>
}

pub fn to_format(req: &HttpRequest) -> String {
    let mut request = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\n",
        req.method, req.path, req.host
    );
    request.push_str("Connection: close\r\n");
    if let Some(body) = &req.body {
        request.push_str(&format!("Content-Length: {}\r\n", body.len()));
        request.push_str("\r\n");
        request.push_str(body);
    } else {
        request.push_str("\r\n");
    }
    request
}