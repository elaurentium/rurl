use crate::resolve;
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::{env, time::Duration};

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub header: Vec<(String, String)>,
    pub body: Option<String>,
    pub host: String,
    pub port: String,
    pub timeout: Option<Duration>,
}

pub fn http_request() -> io::Result<HttpRequest> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "rurl" {
        eprintln!("Usage: {} [options] URL", args[0]);
        eprintln!("Options:");
        eprintln!("  -X, --request <method>    Specify request method (GET, POST, etc.)");
        eprintln!("  -H, --header <header>     Add a custom header");
        eprintln!("  -d, --data <data>         Send data in the request body");
        eprintln!("  -t, --timeout <seconds>   Set connection timeout");
        std::process::exit(1);
    }

    let mut method = "GET".to_string();
    let mut headers: Vec<(String, String)> = vec![];
    let mut data: Option<String> = None;
    let mut timeout: Option<Duration> = None;
    let mut url = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-X" | "--request" => {
                i += 1;
                if i < args.len() {
                    method = args[i].to_uppercase();
                }
            }
            "-H" | "--header" => {
                i += 1;
                if i < args.len() {
                    if let Some((k, v)) = args[i].split_once(':') {
                        headers.push((k.trim().to_string(), v.trim().to_string()));
                    } else {
                        eprintln!("Invalid header format: {}", args[i]);
                    }
                }
            }
            "-d" | "--data" => {
                i += 1;
                if i < args.len() {
                    data = Some(args[i].clone());
                }
            }
            "-t" | "--timeout" => {
                i += 1;
                if i < args.len() {
                    if let Ok(secs) = args[i].parse::<u64>() {
                        timeout = Some(Duration::from_secs(secs));
                    } else {
                        eprintln!("Invalid timeout value: {}", args[i]);
                    }
                }
            }
            _ => {
                if args[i].starts_with("http") {
                    url = args[i].clone();
                }
            }
        }
        i += 1;
    }

    let (host, path, port) = parse_url(&url);

    let mut request = HttpRequest {
        method,
        path,
        header: headers,
        body: data,
        host,
        port,
        timeout,
    };

    if url.is_empty() {
        resolve::resolve_host(&mut request).map_err(|e| {
            eprintln!("Error resolving host: {}", e);
            io::Error::new(io::ErrorKind::InvalidInput, e)
        })?;
        std::process::exit(1);
    }

    connection(&mut request).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(request)
}

pub fn parse_url(url: &str) -> (String, String, String) {
    let url = url.trim_start_matches("http://").trim_start_matches("https://");
    let (host_port, path) = url.split_once('/').unwrap_or((url, ""));
    let path = if path.is_empty() { "/" } else { &format!("/{}", path) };
    let (host, port) = host_port.split_once(':').unwrap_or((host_port, "80"));
    (host.to_string(), path.to_string(), port.to_string())
}

pub fn connection(http: &HttpRequest) -> Result<(), String> {
    let address = format!("{}:{}", http.host, http.port);
    println!("Connecting to: {}", address);

    // Establish TCP connection
    let mut stream = TcpStream::connect(&address)
        .map_err(|e| format!("Failed to connect to {}: {}", address, e))?;

    if let Some(timeout) = http.timeout {
        stream
            .set_read_timeout(Some(timeout))
            .map_err(|e| format!("Failed to set read timeout: {}", e))?;
        stream
            .set_write_timeout(Some(timeout))
            .map_err(|e| format!("Failed to set write timeout: {}", e))?;
    }

    let mut request = String::new();
    request.push_str(&format!("{} {} HTTP/1.1\r\n", http.method, http.path));
    request.push_str(&format!("Host: {}\r\n", http.host));
    request.push_str("Connection: close\r\n");
    request.push_str("\r\n");
    request.push_str("Accept: */*\r\n");

    for (key, value) in &http.header {
        request.push_str(&format!("{}: {}\r\n", key, value));
    }

    if let Some(body) = &http.body {
        request.push_str(&format!("Content-Length: {}\r\n", body.len()));
        request.push_str("\r\n");
        request.push_str(body);
    } else {
        request.push_str("\r\n");
    }

    println!("Request:\n{}", request);

    stream
        .write_all(request.as_bytes())
        .map_err(|e| format!("Failed to write to server: {}", e))?;
    stream
        .flush()
        .map_err(|e| format!("Failed to flush stream: {}", e))?;

    let mut response = String::new();

    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("Failed to read from server: {}", e))?;

    println!("Response from server:\n{}", response);

    // Clean up
    let _ = stream.shutdown(Shutdown::Both);

    Ok(())
}