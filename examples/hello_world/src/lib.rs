extern crate whr;
use whr::{Request};

// handler is a user handler, incoming Request struct let's you get
// body, method, path, query and headers
fn handler(req: Request) {    
    let new_body = format!("{{\"text\": \"{}\"}}", req.get_body());
    let new_path = format!("{}/page/10", req.get_path());

    // we can use whr crate public functions to modify request
    whr::set_request_method("POST".to_string());
    whr::set_request_header("Content-Type".to_string(), "application/json".to_string());
    whr::set_request_body(new_body);
    whr::set_request_path(new_path);
}

// our entry point into the application, this is called by Webhook Relay with
// initial request payload
#[no_mangle]
pub extern "C" fn handleRequest(ptr: i32, len: i32) {
    whr::run(ptr, len, handler)
}