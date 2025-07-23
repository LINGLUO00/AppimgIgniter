use anyhow::Result;
use std::ffi::CString;
use std::os::raw::c_char;

// encapsulate the appimage library functions
unsafe extern "C" {
    fn appimage_install_desktop_file(desktop_file: *const c_char);
    fn appimage_install_icon(icon: *const c_char);
    fn appimage_register_in_system(desktop_file: *const c_char) -> i32;
    fn appimage_unregister_in_system(desktop_file: *const c_char) -> i32;
    fn appimage_hash_appimage(appimage_file: *const c_char) -> *const c_char;
}

// install the desktop file to the system
pub fn install_desktop_file(desktop_file: &str) -> Result<()> {
    let desktop_file = CString::new(desktop_file).unwrap();
    unsafe {
        appimage_install_desktop_file(desktop_file.as_ptr());
    }
    Ok(())
}

// install the icon to the system
pub fn install_icon(icon: &str) -> Result<()> {
    let icon = CString::new(icon).unwrap();
    unsafe {
        appimage_install_icon(icon.as_ptr());
    }
    Ok(())
}

// register the desktop file in the system
pub fn register_in_system(desktop_file: &str) -> Result<()> {
    let desktop_file = CString::new(desktop_file).unwrap();
    unsafe {
        let result = appimage_register_in_system(desktop_file.as_ptr());
        if result == 0 {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to register the desktop file in the system"
            ))
        }
    }
}

// unregister the desktop file in the system
pub fn unregister_in_system(desktop_file: &str) -> Result<()> {
    let desktop_file = CString::new(desktop_file).unwrap();
    unsafe {
        appimage_unregister_in_system(desktop_file.as_ptr());
    }
    Ok(())
}

// hash the appimage file
pub fn hash_appimage(appimage_file: &str) -> String {
    let appimage_file = CString::new(appimage_file).unwrap();
    unsafe {
        let hash = appimage_hash_appimage(appimage_file.as_ptr());
        let hash = CString::from_raw(hash as *mut c_char);
        hash.to_string_lossy().to_string()
    }
}
