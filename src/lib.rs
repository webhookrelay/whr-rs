// #![feature(catch_panic)]
extern crate base64;
use std::panic;

use std::slice;
use std::str;
use std::thread;

use base64::decode;
use serde::{Deserialize, Serialize};

// By default, the "env" namespace is used.
extern "C" {
    fn ext_stop_forwarding();
    fn ext_set_error(ptr: *const u8, len: usize);
    // ext_set_request_body function allows updating request body
    fn ext_set_request_body(ptr: *const u8, len: usize);
    // ext_set_request_header functions allows updating individual HTTP headers
    fn ext_set_request_header(
        key_ptr: *const u8,
        key_len: usize,
        val_ptr: *const u8,
        val_len: usize,
    );
    // ext_set_request_method updates request method
    fn ext_set_request_method(ptr: *const u8, len: usize);
    // ext_set_request_path function allows updating request path (what's after domain),
    // for example request "https://example.com/api/foo" path is /api/foo
    // so we can here update it to /some/other/api. Pair this with a body and header
    // modification to get a completely different request
    fn ext_set_request_path(ptr: *const u8, len: usize);
    // ext_set_request_raw_query function allows updating request query,
    // for example request "https://example.com/api/foo?foo=bar" raw query is foo=bar
    // so we can here update it to something=else.
    fn ext_set_request_raw_query(ptr: *const u8, len: usize);

    fn ext_set_response_body(ptr: *const u8, len: usize);
    fn ext_set_response_header(
        key_ptr: *const u8,
        key_len: usize,
        val_ptr: *const u8,
        val_len: usize,
    );
    fn ext_set_response_status_code(statusCode: i32);
}

#[derive(Serialize, Deserialize)]
pub struct PayloadStruct {
    // Use the type's implementation of std::default::Default if
    // "method" or other fields are not included in the input.
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub raw_query: String,
    #[serde(default)]
    pub body: String,
    // TODO: add headers
}

impl Clone for PayloadStruct {
    fn clone(&self) -> Self {
        Self {
            method: self.method.clone(),
            path: self.path.clone(),
            raw_query: self.raw_query.clone(),
            body: self.body.clone(),
        }
    }
}

// #[derive(Clone, Copy)]
pub struct Request {
    payload: PayloadStruct,
}

impl Clone for Request {
    fn clone(&self) -> Self {
        Self {
            payload: self.payload.clone(),
        }
    }
}

impl Request {
    fn new(p: PayloadStruct) -> Self {
        Request { payload: p }
    }

    // get_body returns request body string
    pub fn get_body(&self) -> String {
        self.payload.body.clone()
    }

    pub fn get_path(&self) -> String {
        self.payload.path.clone()
    }

    // get_method returns request method
    pub fn get_method(&self) -> String {
        self.payload.method.clone()
    }
}

// stop_forwarding - don't forward this request any further
pub fn stop_forwarding() {
    unsafe {
        ext_stop_forwarding();
    }
}

// set_request_body - modify request body
pub fn set_request_body(body: String) {
    unsafe {
        ext_set_request_body(body.as_ptr(), body.len());
    }
}

// set_request_method - modify request method
pub fn set_request_method(method: String) {
    unsafe {
        ext_set_request_method(method.as_ptr(), method.len());
    }
}

// set_request_path - update request path. This new path
// will be added to the Output destination's path. If WHR Output
// path is /v1/store and this function sets /foo then the webhook
// will be sent to /v1/store/foo
pub fn set_request_path(path: String) {
    unsafe {
        ext_set_request_path(path.as_ptr(), path.len());
    }
}

// set_request_raw_query - modify raw request query,
// for example request "https://example.com/api/foo?foo=bar" raw query is foo=bar
pub fn set_request_raw_query(query: String) {
    unsafe {
        ext_set_request_raw_query(query.as_ptr(), query.len());
    }
}

// set_request_header - set request key/value
pub fn set_request_header(key: String, value: String) {
    unsafe {
        ext_set_request_header(key.as_ptr(), key.len(), value.as_ptr(), value.len());
    }
}

// set_response_body - set response body
pub fn set_response_body(body: String) {
    unsafe {
        ext_set_response_body(body.as_ptr(), body.len());
    }
}

// set_response_status_code - set response status code (defaults to 200)
pub fn set_response_status_code(status: i32) {
    unsafe {
        ext_set_response_status_code(status);
    }
}

// set_response_header - set response header
pub fn set_response_header(key: String, value: String) {
    unsafe {
        ext_set_response_header(key.as_ptr(), key.len(), value.as_ptr(), value.len());
    }
}

// TODO: expose delete header fn

/// Run a function
///
pub fn run(ptr: i32, len: i32, to_run: fn(Request)) {
    let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
    // need to parse here the contents into some struct where we can get body
    // example: {"body":"some-body-here","method":"PUT", "raw_query": "/foo/bar"}
    let string_from_host = str::from_utf8(&slice).unwrap();

    let payload = parse_payload(string_from_host);

    let request = Request::new(payload);

    let result = panic::catch_unwind(move || {
        to_run(request);
    });

    if result.is_err() {
        let error_message = "function panicked";
        unsafe {
            ext_set_error(error_message.as_ptr(), error_message.len());
        }
    }
}

pub fn parse_payload(payload_string: &str) -> PayloadStruct {
    let mut parsed_payload: PayloadStruct = serde_json::from_str(&payload_string).unwrap();
    parsed_payload.body = String::from_utf8(decode(&parsed_payload.body).unwrap()).unwrap();
    parsed_payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_payload_method() {
        let result = parse_payload("{\"method\":\"POST\"}");
        assert_eq!(result.method, "POST");
    }

    #[test]
    fn test_parse_payload_body() {
        let result = parse_payload("{\"method\":\"POST\", \"body\": \"Zm9vbw==\"}");
        assert_eq!(result.method, "POST");
        assert_eq!(result.body, "fooo");
    }
}
