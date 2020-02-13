extern crate whr;
// use whr;
use whr::{Request};

fn handler(req: Request) {
    req::set_request_method("DELETE")
}

#[no_mangle]
pub extern "C" fn handleRequest(ptr: i32, len: i32) {
    whr::run(ptr, len, handler)
}