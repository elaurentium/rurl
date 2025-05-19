use std::net::{ToSocketAddrs, SocketAddr};

use crate::http_request::HttpRequest;

pub fn resolve_host(http: &mut HttpRequest) -> Result<(), String> {
    if http.host.is_empty() {
        return Err("Host is empty".to_string());
    }


    Ok(())
}