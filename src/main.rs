use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

fn main() {}

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Greeting {
    message: String,
}

#[no_mangle]
pub fn hello(input_ptr: *mut c_char) -> *mut c_char {
    let input = unsafe {
        CStr::from_ptr(input_ptr)
    }.to_str().unwrap();
    let person: Person = serde_json::from_str(input).unwrap();
    let message = format!(
        "Hello, {} {}! Welcome to Rust World!!",
        person.first_name,
        person.last_name
    );
    let greeting = Greeting { message };
    let res = serde_json::to_string(&greeting).unwrap();
    let c_str = CString::new(res).unwrap();
    c_str.into_raw()
}
