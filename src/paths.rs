use std::path::PathBuf;
use std::process::Command;

pub fn app_data_base_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let mut p = PathBuf::from(local_app_data);
            p.push("AerolithBrowser");
            return p;
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            let mut p = PathBuf::from(home);
            p.push("Library");
            p.push("Application Support");
            p.push("AerolithBrowser");
            return p;
        }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
            let mut p = PathBuf::from(xdg);
            p.push("aerolith-browser");
            return p;
        } else if let Ok(home) = std::env::var("HOME") {
            let mut p = PathBuf::from(home);
            p.push(".local");
            p.push("share");
            p.push("aerolith-browser");
            return p;
        }
    }

    let mut p = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    p.push(".aerolith");
    p
}

pub fn dirs_storage_path() -> PathBuf {
    let mut p = app_data_base_dir();
    let _ = std::fs::create_dir_all(&p);
    p.push("aerolith_storage.redb");
    p
}

pub fn dirs_webview_data_path() -> PathBuf {
    let mut p = app_data_base_dir();
    #[cfg(target_os = "windows")]
    p.push("WebView2Data");
    #[cfg(not(target_os = "windows"))]
    p.push("WebViewData");
    let _ = std::fs::create_dir_all(&p);
    p
}

pub fn dirs_webview_fallback_data_path() -> PathBuf {
    let mut p = app_data_base_dir();
    #[cfg(target_os = "windows")]
    p.push(format!("WebView2Data_{}", std::process::id()));
    #[cfg(not(target_os = "windows"))]
    p.push(format!("WebViewData_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&p);
    p
}

pub fn touch_keyboard_candidate_paths() -> Vec<String> {
    #[cfg(target_os = "windows")]
    {
        vec![
            r"C:\Program Files\Common Files\microsoft shared\ink\TabTip.exe".to_string(),
            r"C:\Program Files (x86)\Common Files\microsoft shared\ink\TabTip.exe".to_string(),
            "tabtip.exe".to_string(),
            r"C:\Windows\System32\osk.exe".to_string(),
            "osk.exe".to_string(),
        ]
    }
    #[cfg(target_os = "macos")]
    {
        vec![
            "/System/Library/Input Methods/AccessibilityKeyboard.app/Contents/MacOS/AccessibilityKeyboard".to_string(),
            "AccessibilityKeyboard".to_string(),
        ]
    }
    #[cfg(target_os = "linux")]
    {
        vec![
            "onboard".to_string(),
            "maliit-keyboard".to_string(),
            "matchbox-keyboard".to_string(),
            "florence".to_string(),
        ]
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        vec!["osk".to_string()]
    }
}

pub fn launch_touch_keyboard() {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg("-a").arg("AccessibilityKeyboard").spawn();
    }

    #[cfg(not(target_os = "macos"))]
    {
        for p in touch_keyboard_candidate_paths() {
            if Command::new(&p).spawn().is_ok() {
                return;
            }
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
    #[cfg(target_os = "macos")]
    {
        let escaped = msg.replace('"', "\\\"");
        let script = format!("display dialog \"{}\" with title \"Aerolith Browser Error\" buttons {{\"OK\"}} default button \"OK\" with icon stop", escaped);
        let _ = Command::new("osascript").arg("-e").arg(script).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("zenity")
            .arg("--error")
            .arg("--title=Aerolith Browser Error")
            .arg(format!("--text={}", msg))
            .spawn()
            .or_else(|_| {
                Command::new("kdialog")
                    .arg("--error")
                    .arg(msg)
                    .spawn()
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirs_webview_fallback_data_path() {
        let p = dirs_webview_fallback_data_path();
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        assert!(name.contains("WebView") && name.contains("Data_"), "Fallback path must contain WebView and Data_");
    }

    #[test]
    fn test_keyboard_paths_contain_osk() {
        let paths = touch_keyboard_candidate_paths();
        assert!(!paths.is_empty(), "Touch keyboard candidates must not be empty");
        #[cfg(target_os = "windows")]
        {
            assert!(paths.iter().any(|p| p.contains("osk.exe")));
            assert!(paths.iter().any(|p| p.contains("TabTip.exe")));
        }
        #[cfg(target_os = "macos")]
        {
            assert!(paths.iter().any(|p| p.contains("AccessibilityKeyboard")));
        }
        #[cfg(target_os = "linux")]
        {
            assert!(paths.iter().any(|p| p.contains("onboard") || p.contains("maliit") || p.contains("matchbox")));
        }
    }
}
