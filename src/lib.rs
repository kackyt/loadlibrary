#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::{
    ffi::CString,
    os::{raw::c_void, unix::ffi::OsStrExt},
    path::Path,
};
extern crate libc;
include!("../bindgen/bindings.rs");

pub fn win_dlopen<P: AsRef<Path>>(image: &mut pe_image, path: P) -> anyhow::Result<i32> {
    let refp = path.as_ref();
    let c_str = refp.as_os_str().as_bytes();
    let mut size: usize = 0;
    let mut handle = String::from("dummymodule");

    unsafe {
        libc::memcpy(
            image.name.as_mut_ptr() as *mut libc::c_void,
            c_str.as_ptr() as *const libc::c_void,
            c_str.len()
        );
        pe_load_library(
            image.name.as_ptr(),
            &mut image.image,
            &mut size as *mut usize,
        );

        image.size = size as i32;

        link_pe_images(image as *mut pe_image, 1);

        if let Some(entry) = image.entry {
            entry(handle.as_mut_ptr() as *mut c_void, DLL_PROCESS_ATTACH, std::ptr::null_mut());
        }
    }

    Ok(image.size)
}

pub unsafe fn win_dlsym(sym: &str) -> anyhow::Result<*const c_void> {
    let mut ret: *mut c_void = std::ptr::null_mut();
    let c_str = CString::new(sym)?;

    get_export(
        c_str.as_ptr() as *const i8,
        &mut ret as *mut *mut c_void as *mut c_void,
    );

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn it_works() {
        let mut image: pe_image = Default::default();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Test.dll");
        assert!(win_dlopen(&mut image, &path).is_ok());
        unsafe {
            assert_ne!(win_dlsym("MJPInterfaceFunc"), std::ptr::null());
        }
    }
}
