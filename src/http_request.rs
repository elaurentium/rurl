pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub header: Vec<(String, String)>,
    pub body: String,
    pub query: String,
    pub host: String,
    pub port: String,
}

impl HttpRequest {
    pub fn new() -> Self {
        HttpRequest {
            method: String::new(),
            path: String::new(),
            header: Vec::new(),
            body: String::new(),
            query: String::new(),
            host: String::new(),
            port: String::new(),
        }
    }
}

