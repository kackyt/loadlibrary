use std::{os::raw::c_void, path::Path};

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

pub fn win_dlopen<P: AsRef<Path>>(path: P) -> anyhow::Result<*mut c_void> {
    #[cfg(target_os = "linux")]
    {
        linux::win_dlopen(path)
    }

    #[cfg(target_os = "windows")]
    {
        windows::win_dlopen(path)
    }
}

pub unsafe fn win_dlsym(handle: *mut c_void, sym: &str) -> anyhow::Result<*const c_void> {
    #[cfg(target_os = "linux")]
    {
        linux::win_dlsym(handle, sym)
    }

    #[cfg(target_os = "windows")]
    {
        windows::win_dlsym(handle, sym)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn it_works() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Test.dll");
        assert!(win_dlopen(&path).is_ok());
        unsafe {
            let handle = win_dlopen(&path).unwrap();
            assert_ne!(
                win_dlsym(handle, "MJPInterfaceFunc").unwrap(),
                std::ptr::null()
            );
        }
    }
}
