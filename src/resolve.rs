use std::net::{ToSocketAddrs, SocketAddr};

pub fn resolve_host(HttpRequest: &mut HttpRequest) -> Result<(), String> {
    if HttpRequest.host.is_empty() {
        return Err("Host is empty".to_string());
    }

    return Ok(());
}