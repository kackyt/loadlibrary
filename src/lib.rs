#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::os::raw::c_void;

include!("../bindgen/bindings.rs");

pub fn load(path: &str) -> usize {
    let mut size: usize = 0;
    let mut image: *mut c_void = std::ptr::null_mut();

    unsafe {
        pe_load_library(
            path.as_bytes().as_ptr() as *const i8,
            &mut image,
            &mut size as *mut usize,
        );
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
