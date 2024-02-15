#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use anyhow::ensure;
use std::{
    ffi::CString,
    os::{raw::c_void, unix::ffi::OsStrExt},
    path::Path,
};
extern crate libc;
include!("../bindgen/bindings.rs");

pub fn win_dlopen<P: AsRef<Path>>(path: P) -> anyhow::Result<*mut c_void> {
    let refp = path.as_ref();
    let c_str = refp.as_os_str().as_bytes();
    let mut image: pe_image = Default::default();
    let mut size: usize = 0;
    let mut handle = String::from("dummymodule");

    unsafe {
        libc::memcpy(
            image.name.as_mut_ptr() as *mut libc::c_void,
            c_str.as_ptr() as *const libc::c_void,
            c_str.len(),
        );
        ensure!(
            pe_load_library(
                image.name.as_ptr(),
                &mut image.image,
                &mut size as *mut usize,
            ),
            "Cannot load {}",
            refp.to_str().unwrap_or("[unknown path]")
        );

        image.size = size as i32;

        ensure!(
            link_pe_images(&mut image as *mut pe_image, 1) == 0,
            "Cannot link {}",
            refp.to_str().unwrap_or("[unknown path]")
        );

        if let Some(entry) = image.entry {
            entry(
                handle.as_mut_ptr() as *mut c_void,
                DLL_PROCESS_ATTACH,
                std::ptr::null_mut(),
            );
        }
    }

    Ok(&image as *const pe_image as *mut c_void)
}

pub unsafe fn win_dlsym(_: *mut c_void, sym: &str) -> anyhow::Result<*const c_void> {
    let mut ret: *mut c_void = std::ptr::null_mut();
    let c_str = CString::new(sym)?;

    ensure!(
        get_export(
            c_str.as_ptr() as *const i8,
            &mut ret as *mut *mut c_void as *mut c_void,
        ) == 0,
        "Not found {}",
        sym
    );

    Ok(ret)
}

pub unsafe fn win_dlclose(handle: *mut c_void) -> anyhow::Result<()> {
    let image = &mut *(handle as *mut pe_image);
    if let Some(entry) = image.entry {
        entry(
            std::ptr::null_mut(),
            DLL_PROCESS_DETACH,
            std::ptr::null_mut(),
        );
    }
    Ok(())
}
