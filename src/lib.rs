#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::os::raw::c_void;
include!("../bindgen/bindings.rs");

pub fn win_dlopen(path: &str) -> usize {
    let mut size: usize = 0;
    let mut image: pe_image = Default::default();

    unsafe {
        pe_load_library(
            path.as_bytes().as_ptr() as *const i8,
            &mut image.image,
            &mut size as *mut usize,
        );

        image.size = size as i32;

        link_pe_images(&mut image as *mut pe_image, 1);

        if let Some(entry) = image.entry {
            entry(image.image, DLL_PROCESS_ATTACH, std::ptr::null_mut());
        }
    }

    size
}

pub unsafe fn win_dlsym(sym: &str) -> *const c_void {
    let mut ret: *mut c_void = std::ptr::null_mut();

    get_export(
        sym.as_bytes().as_ptr() as *const i8,
        &mut ret as *mut *mut c_void as *mut c_void,
    );

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_ne!(win_dlopen("Test.dll"), 0);
        unsafe {
            assert_ne!(win_dlsym("MJPInterfaceFunc"), std::ptr::null());
        }
    }
}
