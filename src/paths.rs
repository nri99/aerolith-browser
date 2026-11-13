use std::path::PathBuf;
use std::process::Command;

pub fn dirs_storage_path() -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let mut p = PathBuf::from(local_app_data);
        p.push("AerolithBrowser");
        let _ = std::fs::create_dir_all(&p);
        p.push("aerolith_storage.redb");
        p
    } else {
        PathBuf::from("aerolith_storage.redb")
    }
}

pub fn dirs_webview_data_path() -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let mut p = PathBuf::from(local_app_data);
        p.push("AerolithBrowser");
        p.push("WebView2Data");
        let _ = std::fs::create_dir_all(&p);
        p
    } else {
        let mut p = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        p.push("WebView2Data");
        let _ = std::fs::create_dir_all(&p);
        p
    }
}

pub fn dirs_webview_fallback_data_path() -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let mut p = PathBuf::from(local_app_data);
        p.push("AerolithBrowser");
        p.push(format!("WebView2Data_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&p);
        p
    } else {
        let mut p = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        p.push(format!("WebView2Data_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&p);
        p
    }
}

pub const TOUCH_KEYBOARD_PATHS: &[&str] = &[
    r"C:\Program Files\Common Files\microsoft shared\ink\TabTip.exe",
    r"C:\Program Files (x86)\Common Files\microsoft shared\ink\TabTip.exe",
    "tabtip.exe",
    r"C:\Windows\System32\osk.exe",
    "osk.exe",
];

pub fn touch_keyboard_candidate_paths() -> &'static [&'static str] {
    TOUCH_KEYBOARD_PATHS
}

pub fn launch_touch_keyboard() {
    for p in touch_keyboard_candidate_paths() {
        if let Ok(_) = Command::new(p).spawn() {
            return;
        }
    }
}

#[cfg(target_os = "windows")]
pub fn show_fatal_error(msg: &str) {
    use std::os::windows::ffi::OsStrExt;
    let wide_msg: Vec<u16> = std::ffi::OsStr::new(msg).encode_wide().chain(Some(0)).collect();
    let wide_title: Vec<u16> = std::ffi::OsStr::new("Aerolith Browser Error").encode_wide().chain(Some(0)).collect();
    #[link(name = "user32")]
    unsafe extern "system" {
        fn MessageBoxW(hwnd: isize, text: *const u16, caption: *const u16, utype: u32) -> i32;
    }
    unsafe {
        MessageBoxW(0, wide_msg.as_ptr(), wide_title.as_ptr(), 0x10 /* MB_ICONERROR */);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn show_fatal_error(msg: &str) {
    eprintln!("Aerolith Browser Error: {}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirs_webview_fallback_data_path() {
        let p = dirs_webview_fallback_data_path();
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        assert!(name.starts_with("WebView2Data_"));
    }

    #[test]
    fn test_keyboard_paths_contain_osk() {
        let paths = touch_keyboard_candidate_paths();
        assert!(paths.iter().any(|p| p.contains("osk.exe")));
        assert!(paths.iter().any(|p| p.contains("TabTip.exe")));
    }
}
