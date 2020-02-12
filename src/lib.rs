extern crate base64;

use std::slice;
use std::str;

use base64::decode;
use serde::{Deserialize, Serialize};

// By default, the "env" namespace is used.
extern "C" {
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
}

#[derive(Serialize, Deserialize)]
pub struct PayloadStruct {
    // Use the type's implementation of std::default::Default if
    // "method" or other fields are not included in the input.
    #[serde(default)]
    method: String,
    #[serde(default)]
    raw_query: String,
    #[serde(default)]
    body: String,
    // TODO: add headers
}

pub struct Request {
    payload: PayloadStruct,
}

impl Request {
    // get_body returns request body string
    fn get_body(self) -> String {
        self.payload.body
    }
    // get_method returns request method
    fn get_method(self) -> String {
        self.payload.method
    }

    // set_request_body - modify request body
    fn set_request_body(self, body: String) {
        unsafe {
            ext_set_request_body(body.as_ptr(), body.len());
        }
    }

    // set_request_method - modify request method
    fn set_request_method(self, method: String) {
        unsafe {
            ext_set_request_method(method.as_ptr(), method.len());
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
