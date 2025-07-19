use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;
use std::os::raw::c_char;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path};
use std::string::ToString;
use std::time::{SystemTime, UNIX_EPOCH};
///
/// #
/// * 'name' - appimage name
/// * 'version' - appimane version
/// * 'exec' - the path of the executable file
/// * 'icon' - if the appimage icon is not found, the default icon will be used
/// * 'appimage_type' - such as Type=Application in .desktop
/// * 'categories' - appimage categories
/// * 'mine_type' - used to indicate that the application can open or handle specific types of files
/// * 'keywords' - used to search
/// * 'desktop_entry' - used to store the full content of the parsed desktop file

#[derive(Debug, Clone)]
pub struct AppimageMetadata {
    pub name: String,
    pub version: String,
    pub exec: Option<String>,
    pub icon: Option<String>,
    pub desktop_entry: Vec<String>,
}
//
// # param
// * appimage_list_files - return a file list ended with NULL
// * appimage_string_list_free - free the file list which get by appimage_list_files
// * appimage_extract_file_following_symlinks = extract files from appimage to target file_path,if sucess, return 0;

unsafe extern "C" {
    fn appimage_list_files(path: *const c_char) -> *mut *mut c_char;
    fn appimage_string_list_free(list: *mut *mut c_char);
    fn appimage_extract_file_following_symlinks(
        appimage_file_path: *const c_char,
        file_path: *const c_char,
        target_file_path: *const c_char,
    ) -> i32;
}

/// encapsulation C functions
fn list_files(appimage_path: &Path) -> Result<Vec<String>> {
    let c_path = CString::new(appimage_path.as_os_str().as_bytes())?;
    let mut list = Vec::new();
    unsafe {
        let list_ptr = appimage_list_files(c_path.as_ptr());
        if list_ptr.is_null() {
            return Err(anyhow!("appimage_list_files return null"));
        }
        let mut idx = 0;
        loop {
            let item_ptr = *list_ptr.add(idx);
            if item_ptr.is_null() {
                break;
            }
            let r_str = CStr::from_ptr(item_ptr).to_string_lossy().into_owned();
            list.push(r_str);
            idx += 1;
        }
        appimage_string_list_free(list_ptr);
    }
    return Ok(list);
}

fn extract_files(
    appimage_file_path: &Path,
    file_path: &Path,
    target_file_path: &Path,
) -> Result<()> {
    let c_appimage_file_path = CString::new(appimage_file_path.as_os_str().as_bytes())?;
    let c_file_path = CString::new(file_path.as_os_str().as_bytes())?;
    let c_target_file_path = CString::new(target_file_path.as_os_str().as_bytes())?;
    let result = unsafe {
        appimage_extract_file_following_symlinks(
            c_appimage_file_path.as_ptr(),
            c_file_path.as_ptr(),
            c_target_file_path.as_ptr(),
        )
    };
    if result != 0 {
        return Err(anyhow!("extract files from appimage failed"));
    }
    return Ok(());
}
// extract files and set attribute
/// parse desktop file
fn parse_desktop_file(desktop_path: &Path) -> Result<(HashMap<String, String>, Vec<String>)> {
    let mut kv_map = HashMap::new();
    let file = File::open(desktop_path).map_err(|e| {
        anyhow!(
            "fail to read desktop_file from {}->{}",
            desktop_path.display(),
            e
        )
    })?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut in_entry_session = false;

    for line in reader.lines() {
        let line = line?;
        lines.push(line.clone());
        if let Some(stripped) = line.strip_prefix("[") {
            in_entry_session = stripped.starts_with("Desktop Entry");
            continue;
        }
        if line.trim_start().starts_with("#") || !in_entry_session {
            continue;
        }
        if let Some((k, v)) = line.split_once("=") {
            kv_map
                .entry(k.trim().to_string())
                .or_insert_with(|| v.trim().to_string());
        }
    }
    return Ok((kv_map, lines));
}

pub fn extract_metadata(appimage_path: &Path) -> Result<AppimageMetadata> {
    let files = list_files(appimage_path)?;
    let desktop_path = files
        .iter()
        .find(|f| f.ends_with(".desktop"))
        .ok_or_else(|| anyhow!("can not find desktop file"))?
        .to_string();
    let tmp_dir = std::env::temp_dir();
    let time_stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let tmp_desktop_path = tmp_dir.join(format!("{}.{}.desktop", "appimage", time_stamp));
    let _ = extract_files(appimage_path, Path::new(&desktop_path), &tmp_desktop_path);
    let (kv, lines) = parse_desktop_file(&tmp_desktop_path)?;
    let metadata = AppimageMetadata {
        name: kv.get("Name").cloned().unwrap_or("Unknow Name".to_string()),
        version: kv
            .get("Version")
            .or_else(|| kv.get("X-AppImage-Version"))
            .cloned()
            .unwrap_or("Unknow Version".to_string()),
        exec: kv.get("Exec").cloned(),
        icon: kv.get("Icon").cloned(),
        desktop_entry: lines,
    };
    let _ = std::fs::remove_file(&tmp_desktop_path);
    return Ok(metadata);
}
