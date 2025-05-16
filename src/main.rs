mod http_request;


fn main() -> Result<(), String> {
    let mut req = http_request::http_request().map_err(|e| e.to_string())?;
    http_request::connection(&mut req)?;
    Ok(())
}
