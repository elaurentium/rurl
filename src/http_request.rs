use std::path;
use std::{env, time::Duration};
use std::io::{self, Read, Write};
use std::net::{TcpStream, Shutdown};

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub header: Vec<(String, String)>,
    pub body: Option<String>,
    pub query: String,
    pub host: String,
    pub port: String,
    pub timeout: Option<Duration>,
}


pub fn http_request() -> io::Result<HttpRequest> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "rurl" {
        println!("Uso: {} [opções] URL", args[0]);
        println!("Opções:");
        println!("  -X, --request <método>    Especifica o método de requisição (GET, POST, etc.)");
        println!("  -H, --header <header>     Adiciona um header personalizado");
        println!("  -d, --data <dados>        Envia dados no corpo da requisição");
        println!("  -v, --verbose             Mostra informações detalhadas");
        println!("  -o, --output <arquivo>    Salva a resposta em um arquivo");
        println!("  -t, --timeout <segundos>  Define o timeout da conexão");
        std::process::exit(1);
    }

    let url = args.last().unwrap().clone();
    let method = "GET".to_string();

    let host = url.clone().to_string();
    let port = "80".to_string();
    let path = "/".to_string();

    let request = HttpRequest {
        method,
        path,
        header: vec![],
        body: None,
        query: String::new(),
        host,
        port,
        timeout: None,
    };

    return Ok(request);
}

pub fn connection(http: &mut HttpRequest) -> Result<(), String> {
    let host = &http.host;
    let path = &http.path;
    let mut stream = TcpStream::connect(host).expect("Não pode conectar ao servidor.");

    let mut request = String::new();
    request.push_str(&format!("{} {} HTTP/1.1\r\n", http.method, path));
    request.push_str("\r\n");
    request.push_str(&format!("Host: {}\r\n", host));
    request.push_str("\r\n");
    request.push_str("Connection: close\r\n");
    request.push_str("\r\n");
    print!("\n-----------\n{}\n-----------\n", request);

    let req_bytes = request.as_bytes();

    stream
        .write_all(req_bytes)
        .expect("Não pode escrever no servidor.");

    stream
        .shutdown(Shutdown::Both)
        .expect("shutdown call failed");
    print!("Foi conectado ao servidor http over TCP");

    stream
        .read_to_string(&mut request)
        .expect("Não pode ler do servidor.");
    print!("Conectado ao servidor: {}", request);

    Ok(())
}
