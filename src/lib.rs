#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn init_test(){
    let x: *mut raw_t = ptr::null_mut();
    unsafe {
        dbm_init(x, 3);
    }
}
