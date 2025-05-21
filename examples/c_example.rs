//! Simple HTTP GET and POST requests
//!
//! ```bash
//! cargo run --example example
//! ```

use rurl::{http_get, http_post, free_response};
use std::ffi::CString;

fn get() {
    let example = "http://localhost:8000";
    let c_url = CString::new(example).unwrap();
    let response = http_get(c_url.as_ptr());

    if !response.is_null() {
        unsafe {
            println!("Status: {}", (*response).status);
            println!("Body: {}", (*response).body);
            free_response(response);
        }
    } else {
        eprintln!("Request failed.");
    }
}

fn post() {
    let example = "http://localhost:8000";
    let c_url = CString::new(example).unwrap();
    let body = CString::new("{\"test\": \"value\"}").unwrap();
    let response = http_post(c_url.as_ptr(), body.as_ptr());

    if !response.is_null() {
        unsafe {
            println!("Status: {}", (*response).status);
            println!("Body: {}", (*response).body);
            free_response(response);
        }
    } else {
        eprintln!("Request failed.");
    }
}

fn main() {
    get();
    post();
}