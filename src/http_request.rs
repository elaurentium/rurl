use std::env::args;
use std::{env, time::Duration};
use std::io::{self, Write};
use std::net::TcpStream;

pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub path: String,
    pub header: Vec<(String, String)>,
    pub body: Option<String>,
    pub query: String,
    pub host: String,
    pub port: String,
    pub timeout: Option<Duration>,
}


pub fn http_request() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: {} [opções] URL", args[0]);
        println!("Opções:");
        println!("  -X, --request <método>    Especifica o método de requisição (GET, POST, etc.)");
        println!("  -H, --header <header>     Adiciona um header personalizado");
        println!("  -d, --data <dados>        Envia dados no corpo da requisição");
        println!("  -v, --verbose             Mostra informações detalhadas");
        println!("  -o, --output <arquivo>    Salva a resposta em um arquivo");
        println!("  -t, --timeout <segundos>  Define o timeout da conexão");
        return Ok(());
    }


    return Ok(());
}

pub fn connection(http: &mut HttpRequest) -> Result<(), String> {
    let host = format!("{}:{}", http.host, http.port);
    let stream = TcpStream::connect(host).map_err(|e| format!("Erro ao conectar: {}", e))?;
    stream.set_read_timeout(http.timeout).map_err(|e| format!("Erro ao definir timeout: {}", e))?;
    stream.set_write_timeout(http.timeout).map_err(|e| format!("Erro ao definir timeout: {}", e))?;
    Ok(())
}