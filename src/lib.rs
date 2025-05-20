use std::ffi::{CString, CStr};
use std::ptr;

pub mod utils;
pub mod http_req;
pub mod client;

pub use utils::{Url, parse_url};
pub use http_req::{HttpRequest, to_format};
pub use client::{HttpResponse, send_request, get, post};


#[unsafe(no_mangle)]
pub extern "C" fn http_get(url: *const i8) -> *mut HttpResponse {
    let url_str = unsafe {
        if url.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(url).to_str().map_err(|_| "Invalid Url".to_string())
    };

    let (url, _) = match url_str.and_then(|s| parse_url(s)) {
        Ok(url) => url,
        Err(_) => return ptr::null_mut(),
    };

    let request = HttpRequest {
        method: "GET".to_string(),
        path: url.path.clone(),
        host: url.host.clone(),
        body: None,
    };

    match send_request(&request, &url) {
        Ok(response) => Box::into_raw(Box::new(response)),
        Err(_) => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn http_post(url: *const i8, body: *const i8) -> *mut HttpResponse {
    let url_str = unsafe {
        if url.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(url).to_str().map_err(|_| "Invalid Url".to_string())
    };

    let body_str = unsafe {
        if body.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(body).to_str().map_err(|_| "Body inválido".to_string())
    };

    let (url, _) = match url_str.and_then(|s| parse_url(s)) {
        Ok(url) => url,
        Err(_) => return ptr::null_mut(),
    };

    let body = match body_str {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let request = HttpRequest {
        method: "POST".to_string(),
        path: url.path.clone(),
        host: url.host.clone(),
        body: Some(body.to_string()),
    };

    match send_request(&request, &url) {
        Ok(response) => Box::into_raw(Box::new(response)),
        Err(_) => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_response(response: *mut HttpResponse) {
    if !response.is_null() {
        unsafe {
            if !(*response).status.is_empty() {
                let _ = CString::from_raw((*response).status.as_ptr() as *mut i8);
            }
            if !(*response).body.is_empty() {
                let _ = CString::from_raw((*response).body.as_ptr() as *mut i8);
            }
            // Liberar a estrutura HttpResponse
            let _ = Box::from_raw(response);
        }
    }
}