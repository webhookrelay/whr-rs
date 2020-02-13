extern crate whr;
use whr::{Request};

fn handler(req: Request) {
    req.set_request_method("DELETE".to_string());
}

// our entry point into the application
#[no_mangle]
pub extern "C" fn handleRequest(ptr: i32, len: i32) {
    whr::run(ptr, len, handler)
}