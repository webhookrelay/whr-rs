extern crate whr;
use whr::{Request};

fn handler(req: Request) {    
    let mut body = req.get_body();
    let new_body = format!("{{\"text\": \"{}\"}}", body);
    
    whr::set_request_method("PUT".to_string());
    whr::set_request_body(new_body);
}

// our entry point into the application
#[no_mangle]
pub extern "C" fn handleRequest(ptr: i32, len: i32) {
    whr::run(ptr, len, handler)
}