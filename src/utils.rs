use std::str::FromStr;

#[derive(Debug)]
pub struct Url {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub path: String,
}

pub fn parse_url(url: &str) -> Result<(Url, String), String> {
    let protocol_end = url.find("://").ok_or("Invalid URL: Missing protocol")?;
    let protocol = &url[..protocol_end];
    if protocol != "http" {
        return Err(format!("Invalid URL: Unsupported protocol {}", protocol));
    }

    let rest = &url[protocol_end + 3..];
    let (host, path) = rest
        .find('/')
        .map(|i| (&rest[..i], &rest[i..]))
        .unwrap_or((rest, "/"));

    let (host, port) = host
        .find(":")
        .map(|i| (&host[..i], &host[i + 1..]))
        .unwrap_or((host, "80"));

    let port = u16::from_str(port).map_err(|_| "Invalid URL: Invalid port")?;

    Ok((
        Url {
            protocol: protocol.to_string(),
            host: host.to_string(),
            port,
            path: path.to_string(),
        },
        path.to_string(),
    ))
}
