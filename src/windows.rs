use std::{ffi::CString, os::raw::c_void, path::Path};

use anyhow::ensure;
use winapi::{
    shared::minwindef::HINSTANCE__,
    um::{
        errhandlingapi::GetLastError,
        libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA},
    },
};

pub fn win_dlopen<P: AsRef<Path>>(path: P) -> anyhow::Result<*mut c_void> {
    unsafe {
        let path = path.as_ref().to_str().unwrap();
        let handle = LoadLibraryA(path.as_ptr() as *const i8);

        ensure!(
            !handle.is_null(),
            "Cannot load {} : ERRNO {}",
            path,
            GetLastError()
        );

        return Ok(handle as *mut c_void);
    }
}

pub unsafe fn win_dlsym(handle: *mut c_void, sym: &str) -> anyhow::Result<*const c_void> {
    let proc_name = CString::new(sym)?;
    let func = GetProcAddress(handle as *mut HINSTANCE__, proc_name.as_ptr() as *const i8);

    Ok(func as *const c_void)
}

pub unsafe fn win_dlclose(handle: *mut c_void) -> anyhow::Result<()> {
    ensure!(
        FreeLibrary(handle as *mut HINSTANCE__) != 0,
        "Cannot free library"
    );

    Ok(())
}
