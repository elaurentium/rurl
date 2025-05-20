use std::net::TcpStream;
use std::io::{Write, Read};
use crate::http_req::{HttpRequest, to_format};
use crate::utils::{Url, parse_url};

#[repr(C)]
#[derive(Debug)]
pub struct HttpResponse {
    pub status: String,
    pub body: String,
}


pub fn send_request(http: &HttpRequest, url: &Url) -> Result<HttpResponse, String> {
    let address = format!("{}:{}", url.host, url.port);
    let mut stream = TcpStream::connect(&address)
        .map_err(|e| format!("Failed to connect to {}: {}", address, e))?;

    let request_str = to_format(&http);

    stream
        .write_all(request_str.as_bytes())
        .map_err(|e| format!("Failed to send request: {}", e))?;
    stream.flush()
        .map_err(|e| format!("Failed to flush stream: {}", e))?;

    let mut response = String::new();

    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let (status, body) = response
        .split_once("\r\n\r\n")
        .ok_or("Invalid response format")?;

    Ok(HttpResponse { status: status.to_string(), body: body.to_string() })
}

pub fn get(url: &str) -> Result<HttpResponse, String> {
    let (url, _) = parse_url(url)?;
    let request = HttpRequest {
        method: "GET".to_string(),
        path: url.path.to_string(),
        host: url.host.to_string(),
        body: None,
    };
    send_request(&request, &url)
}

pub fn post(url: &str, body: &str) -> Result<HttpResponse, String> {
    let (url, _) = parse_url(url)?;
    let request = HttpRequest {
        method: "POST".to_string(),
        path: url.path.to_string(),
        host: url.host.to_string(),
        body: Some(body.to_string()),
    };
    send_request(&request, &url)
}

