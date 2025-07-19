use anyhow::{Result, anyhow};
use std::{
    ffi::CString,
    os::raw::{c_char, c_int},
    os::unix::ffi::OsStrExt,
    path::Path,
};

unsafe extern "C" {
    fn appimage_get_type(path: *const c_char, verbose: bool) -> c_int;
}

pub enum AppimageType {
    Invalid,
    Legacy,
    Type1,
    Type2,
}

pub fn get_appimage_type(path: &Path, verbose: bool) -> Result<AppimageType> {
    let c_path = CString::new(path.as_os_str().as_bytes())?;
    let appimage_type = unsafe { appimage_get_type(c_path.as_ptr(), verbose) };
    if appimage_type == -1 {
        Ok(AppimageType::Invalid)
    } else if appimage_type == 0 {
        Ok(AppimageType::Legacy)
    } else if appimage_type == 1 {
        Ok(AppimageType::Type1)
    } else if appimage_type == 2 {
        Ok(AppimageType::Type2)
    } else {
        Err(anyhow!("Invalid appimage type: {}", appimage_type))
    }
}
