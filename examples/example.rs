//! Simple HTTP GET and POST requests
//!
//! ```bash
//! cargo run --example example
//! ```

use rurl::{get, post};

fn main() {
    let get_response = get("http://httpbin.org/get");
    println!("GET Response: {:?}", get_response);

    let post_response = post("http://httpbin.org/post", "{\"test\": \"value\"}");
    println!("POST Response: {:?}", post_response);
}
