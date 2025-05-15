mod http_request;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    return http_request::http_request();
}
