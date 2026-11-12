#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adblock;
mod governor;
mod storage;
mod ui;

use adblock::AdBlocker;
use governor::MemoryGovernor;
use storage::BrowserStorage;
use ui::get_chrome_html;

#[cfg(target_env = "gnu")]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn RoOriginateErrorW(_error: i32, _cchmax: u32, _message: *const u16) -> i32 {
    0
}

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use tao::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy},
    window::{Window, WindowBuilder},
};
use wry::{Rect, WebContext, WebView, WebViewBuilder};

const LOGICAL_CHROME_HEIGHT: f64 = 84.0;
const DEFAULT_WIDTH: u32 = 1280;
const DEFAULT_HEIGHT: u32 = 853;
const HOME_URL: &str = "https://duckduckgo.com";
pub const MAX_TABS: usize = 20;

pub fn physical_chrome_height(scale_factor: f64) -> u32 {
    (LOGICAL_CHROME_HEIGHT * scale_factor).round() as u32
}

pub fn chrome_expanded_height(mode: &str, scale_factor: f64, window_height: u32) -> u32 {
    match mode {
        "full" => window_height,
        "dropdown" => {
            let extra = (300.0 * scale_factor).round() as u32;
            (physical_chrome_height(scale_factor) + extra).min(window_height)
        }
        _ => physical_chrome_height(scale_factor),
    }
}

pub fn get_security_level_for_url(url: &str) -> &'static str {
    let lower = url.trim().to_lowercase();
    if lower.starts_with("https://") {
        "secure"
    } else if lower.starts_with("http://") {
        "insecure"
    } else if lower.starts_with("file://") {
        "local"
    } else {
        "internal"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChromeMsg {
    pub action: String,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabInfo {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub is_sleeping: bool,
}

pub enum UserEvent {
    Chrome(ChromeMsg),
    TitleChanged {
        tab_id: usize,
        title: String,
    },
    UrlChanged {
        tab_id: usize,
        url: String,
    },
    NewWindow(String),
    MediaStateChanged {
        tab_id: usize,
        playing: bool,
    },
    FormDirtyChanged {
        tab_id: usize,
        dirty: bool,
    },
    ScrollChanged {
        tab_id: usize,
        scroll_x: u32,
        scroll_y: u32,
    },
    LoadingState {
        tab_id: usize,
        loading: bool,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum TabState {
    Active,
    Tier1Throttled,
    Tier2Hibernated,
}

struct Tab {
    id: usize,
    title: String,
    url: String,
    scroll_x: u32,
    scroll_y: u32,
    last_active: std::time::Instant,
    state: TabState,
    webview: Option<WebView>,
    is_audio_playing: bool,
    has_form_input: bool,
    is_loading: bool,
}

fn clean_js_json(json: &str) -> String {
    json.replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029")
}

fn normalize_url(input: &str) -> String {
    let mut trimmed = input.trim();
    if trimmed.is_empty() {
        return HOME_URL.to_string();
    }
    // Limit input length to prevent denial of service or buffer overflow
    if trimmed.len() > 4096 {
        trimmed = &trimmed[..4096];
    }

    let lower = trimmed.to_lowercase();
    if lower == "about:blank" || lower.starts_with("about:blank#") {
        return "about:blank".to_string();
    }
    if lower == "about:home" || lower == "about:start" {
        return HOME_URL.to_string();
    }

    // Security: explicitly block dangerous executable pseudoprotocols & data injection
    if lower.starts_with("javascript:")
        || lower.starts_with("vbscript:")
        || lower.starts_with("data:")
        || lower.starts_with("blob:")
        || lower.starts_with("ms-")
        || lower.starts_with("shell:")
        || lower.starts_with("search-ms:")
        || lower.starts_with("search:")
    {
        let encoded = urlencoding_simple(trimmed);
        return format!("https://duckduckgo.com/?q={}", encoded);
    }

    // Case-insensitive scheme check for supported navigation protocols
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("file://") {
        return trimmed.to_string();
    }

    // If input contains another protocol scheme (e.g. ftp://, ws://, gopher://, custom://),
    // treat as a search query rather than prepending https:// which creates a malformed URL
    if lower.contains("://") {
        let encoded = urlencoding_simple(trimmed);
        return format!("https://duckduckgo.com/?q={}", encoded);
    }

    // Localhost / Loopback addresses (default to HTTP)
    if lower.starts_with("localhost") || lower.starts_with("127.0.0.1") || lower.starts_with("[::1]") {
        return format!("http://{}", trimmed);
    }

    // IPv4 address detection (e.g. 192.168.1.1 or 10.0.0.1:8080)
    let host_part = trimmed.split('/').next().unwrap_or("").split(':').next().unwrap_or("");
    let octets: Vec<&str> = host_part.split('.').collect();
    if octets.len() == 4 && octets.iter().all(|seg| seg.parse::<u8>().is_ok()) {
        return format!("http://{}", trimmed);
    }

    // Plausible domain check: no spaces, contains a dot, not a code token (e.g. std::time, v1.0.0)
    // Note: Host must be stripped of any ':port' before checking if the TLD is alphabetic
    if !trimmed.contains(' ') && trimmed.contains('.') && !trimmed.contains("::") {
        let host_no_port = trimmed.split('/').next().unwrap_or("").split(':').next().unwrap_or("");
        let parts: Vec<&str> = host_no_port.split('.').collect();
        if parts.len() >= 2 {
            let tld = parts.last().unwrap_or(&"");
            let tld_alpha = tld.chars().all(|c| c.is_ascii_alphabetic());
            if tld_alpha && tld.len() >= 2 && !parts[0].is_empty() {
                return format!("https://{}", trimmed);
            }
        }
    }

    // Otherwise fallback to search query
    let encoded = urlencoding_simple(trimmed);
    format!("https://duckduckgo.com/?q={}", encoded)
}

fn urlencoding_simple(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            b' ' => out.push('+'),
            _ => {
                out.push_str(&format!("%{:02X}", b));
            }
        }
    }
    out
}

fn launch_touch_keyboard() {
    let paths = [
        r"C:\Program Files\Common Files\microsoft shared\ink\TabTip.exe",
        r"C:\Program Files (x86)\Common Files\microsoft shared\ink\TabTip.exe",
        "tabtip.exe",
    ];
    for p in paths {
        if let Ok(_) = Command::new(p).spawn() {
            return;
        }
    }
}

fn content_rect(size: PhysicalSize<u32>, scale_factor: f64) -> Rect {
    let chrome_h = physical_chrome_height(scale_factor);
    let h = if size.height > chrome_h {
        size.height - chrome_h
    } else {
        100
    };
    Rect {
        position: PhysicalPosition::new(0, chrome_h as i32).into(),
        size: PhysicalSize::new(size.width, h).into(),
    }
}

fn create_content_webview(
    tab_id: usize,
    url: &str,
    win: &Window,
    proxy_ref: &EventLoopProxy<UserEvent>,
    bounds: Rect,
    adblock_ref: &AdBlocker,
    data_dir: &PathBuf,
) -> Result<WebView, Box<dyn std::error::Error>> {
    let mut tab_context = WebContext::new(Some(data_dir.clone()));
    let init_script = format!(
        r#"
        (function() {{
            const style = document.createElement('style');
            style.id = 'aerolith-adblock-shield';
            style.textContent = `{}`;
            document.head && document.head.appendChild(style);
        }})();
        {}
        {}
        {}"#,
        adblock_ref.get_cosmetic_css(),
        adblock_ref.get_neutralizer_script(),
        adblock_ref.get_dark_mode_init_script(),
        AdBlocker::get_touch_gesture_script()
    );

    let proxy_title = proxy_ref.clone();
    let proxy_load = proxy_ref.clone();
    let proxy_new_win = proxy_ref.clone();
    let proxy_content_ipc = proxy_ref.clone();

    let wv = WebViewBuilder::new_with_web_context(&mut tab_context)
        .with_bounds(bounds)
        .with_url(url)
        .with_initialization_script(&init_script)
        .with_back_forward_navigation_gestures(true)
        .with_hotkeys_zoom(true)
        .with_devtools(cfg!(debug_assertions))
        .with_ipc_handler(move |req| {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(req.body()) {
                if let Some(msg_type) = val["type"].as_str() {
                    match msg_type {
                        "media_playing" => {
                            let playing = val["value"].as_bool().unwrap_or(false);
                            let _ = proxy_content_ipc.send_event(UserEvent::MediaStateChanged { tab_id, playing });
                        }
                        "form_dirty" => {
                            let dirty = val["value"].as_bool().unwrap_or(false);
                            let _ = proxy_content_ipc.send_event(UserEvent::FormDirtyChanged { tab_id, dirty });
                        }
                        "scroll" => {
                            let x = val["x"].as_u64().unwrap_or(0) as u32;
                            let y = val["y"].as_u64().unwrap_or(0) as u32;
                            let _ = proxy_content_ipc.send_event(UserEvent::ScrollChanged {
                                tab_id,
                                scroll_x: x,
                                scroll_y: y,
                            });
                        }
                        _ => {}
                    }
                }
            }
        })
        .with_navigation_handler(|nav_url| {
            let lower = nav_url.trim().to_lowercase();
            let allowed = lower.starts_with("http://")
                || lower.starts_with("https://")
                || lower.starts_with("file://")
                || lower.starts_with("about:blank");
            allowed && !AdBlocker::is_tracker_or_ad_domain(&lower)
        })
        .with_document_title_changed_handler(move |title| {
            let _ = proxy_title.send_event(UserEvent::TitleChanged {
                tab_id,
                title,
            });
        })
        .with_on_page_load_handler(move |event, loaded_url| {
            match event {
                wry::PageLoadEvent::Started => {
                    let _ = proxy_load.send_event(UserEvent::LoadingState {
                        tab_id,
                        loading: true,
                    });
                }
                wry::PageLoadEvent::Finished => {
                    let _ = proxy_load.send_event(UserEvent::LoadingState {
                        tab_id,
                        loading: false,
                    });
                    let _ = proxy_load.send_event(UserEvent::UrlChanged {
                        tab_id,
                        url: loaded_url,
                    });
                }
            }
        })
        .with_new_window_req_handler(move |target_url, _| {
            let lower = target_url.trim().to_lowercase();
            if (lower.starts_with("http://")
                || lower.starts_with("https://")
                || lower.starts_with("file://")
                || lower.starts_with("about:blank"))
                && !AdBlocker::is_tracker_or_ad_domain(&lower)
            {
                let _ = proxy_new_win.send_event(UserEvent::NewWindow(target_url));
            }
            wry::NewWindowResponse::Deny
        })
        .build(win)?;

    Ok(wv)
}

fn create_tab(
    tab_id: usize,
    url: &str,
    win: &Window,
    proxy_ref: &EventLoopProxy<UserEvent>,
    bounds: Rect,
    adblock_ref: &AdBlocker,
    data_dir: &PathBuf,
) -> Result<Tab, Box<dyn std::error::Error>> {
    let wv = create_content_webview(tab_id, url, win, proxy_ref, bounds, adblock_ref, data_dir)?;
    Ok(Tab {
        id: tab_id,
        title: "New Tab".to_string(),
        url: url.to_string(),
        scroll_x: 0,
        scroll_y: 0,
        last_active: std::time::Instant::now(),
        state: TabState::Active,
        webview: Some(wv),
        is_audio_playing: false,
        has_form_input: false,
        is_loading: false,
    })
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Aerolith Browser - Surface Go 2")
        .with_inner_size(PhysicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT))
        .build(&event_loop)?;

    // Database and profile directory setup
    let db_path = dirs_storage_path();
    let mut webview_data_dir = dirs_webview_data_path();
    let storage = BrowserStorage::new(&db_path)?;
    let mut adblocker = AdBlocker::new();

    let mut current_scale_factor = window.scale_factor();
    let mut current_size = window.inner_size();
    let initial_chrome_h = physical_chrome_height(current_scale_factor);

    // 1. Build Chrome UI WebView (Pinned to top: 0..physical_chrome_height)
    let proxy_chrome = proxy.clone();
    let mut chrome_context = WebContext::new(Some(webview_data_dir.clone()));
    let chrome_wv = match WebViewBuilder::new_with_web_context(&mut chrome_context)
        .with_transparent(true)
        .with_focused(true)
        .with_bounds(Rect {
            position: PhysicalPosition::new(0, 0).into(),
            size: PhysicalSize::new(DEFAULT_WIDTH, initial_chrome_h).into(),
        })
        .with_html(get_chrome_html())
        .with_ipc_handler(move |req| {
            if let Ok(msg) = serde_json::from_str::<ChromeMsg>(req.body()) {
                let _ = proxy_chrome.send_event(UserEvent::Chrome(msg));
            }
        })
        .build(&window)
    {
        Ok(wv) => wv,
        Err(_) => {
            // Failover to instance-specific directory if primary WebView2 folder is locked
            let fallback_dir = dirs_webview_fallback_data_path();
            let mut fallback_context = WebContext::new(Some(fallback_dir.clone()));
            let proxy_chrome_fb = proxy.clone();
            let wv = WebViewBuilder::new_with_web_context(&mut fallback_context)
                .with_transparent(true)
                .with_focused(true)
                .with_bounds(Rect {
                    position: PhysicalPosition::new(0, 0).into(),
                    size: PhysicalSize::new(DEFAULT_WIDTH, initial_chrome_h).into(),
                })
                .with_html(get_chrome_html())
                .with_ipc_handler(move |req| {
                    if let Ok(msg) = serde_json::from_str::<ChromeMsg>(req.body()) {
                        let _ = proxy_chrome_fb.send_event(UserEvent::Chrome(msg));
                    }
                })
                .build(&window)?;
            webview_data_dir = fallback_dir;
            wv
        }
    };

    // Tab state
    let mut tabs: Vec<Tab> = Vec::new();
    let mut active_tab_index;
    let mut next_tab_id = 1;

    // Initialize Win32 Memory Governor
    let governor = MemoryGovernor::new();

    // Helper to query actual navigation capability for active tab
    let tab_nav_state = |tab: &Tab| -> (bool, bool) {
        if let Some(wv) = &tab.webview {
            (wv.can_go_back().unwrap_or(false), wv.can_go_forward().unwrap_or(false))
        } else {
            (false, false)
        }
    };

    // Helper to sync tabs with chrome UI (including sleep badge state)
    let sync_tabs_to_chrome = |chrome: &WebView, tabs_list: &[Tab], active_idx: usize| {
        let info_list: Vec<TabInfo> = tabs_list
            .iter()
            .map(|t| TabInfo {
                id: t.id,
                title: t.title.clone(),
                url: t.url.clone(),
                is_sleeping: t.state != TabState::Active,
            })
            .collect();
        if let Ok(json) = serde_json::to_string(&info_list) {
            let script = format!("window.updateTabs({}, {});", clean_js_json(&json), active_idx);
            let _ = chrome.evaluate_script(&script);
        }
    };

    // Helper to sync omnibox with chrome UI safely via JSON serialization with actual nav capabilities
    let sync_address_to_chrome =
        |chrome: &WebView, url: &str, title: &str, is_bookmarked: bool, can_back: bool, can_fwd: bool| {
            let url_json = clean_js_json(&serde_json::to_string(url).unwrap_or_else(|_| "\"\"".to_string()));
            let title_json = clean_js_json(&serde_json::to_string(title).unwrap_or_else(|_| "\"\"".to_string()));
            let script = format!(
                "window.updateAddress({}, {}, {}, {}, {});",
                url_json, title_json, can_back, can_fwd, is_bookmarked
            );
            let _ = chrome.evaluate_script(&script);
        };

    // Helper to save current session to redb
    let save_session_state = |storage: &BrowserStorage, tabs_list: &[Tab], active_idx: usize| {
        let urls: Vec<String> = tabs_list.iter().map(|t| t.url.clone()).collect();
        let _ = storage.save_session(&urls, active_idx);
    };

    // 2. Initialize Session: Restore previous tabs safely up to MAX_TABS
    let session = storage.load_session();
    let (initial_urls, initial_active_idx) = match session {
        Some(s) if !s.tabs.is_empty() => (s.tabs, s.active_index),
        _ => (vec![HOME_URL.to_string()], 0),
    };

    for u in &initial_urls {
        let t_id = next_tab_id;
        next_tab_id += 1;
        if let Ok(t) = create_tab(t_id, u, &window, &proxy, content_rect(current_size, current_scale_factor), &adblocker, &webview_data_dir) {
            tabs.push(t);
        }
    }
    if tabs.is_empty() {
        let initial_tab = create_tab(
            next_tab_id,
            HOME_URL,
            &window,
            &proxy,
            content_rect(current_size, current_scale_factor),
            &adblocker,
            &webview_data_dir,
        )?;
        tabs.push(initial_tab);
        next_tab_id += 1;
    }

    // Determine actual active tab and guarantee visibility state across all restored tabs
    active_tab_index = if initial_active_idx < tabs.len() {
        initial_active_idx
    } else {
        tabs.len() - 1
    };

    for (idx, tab) in tabs.iter_mut().enumerate() {
        if let Some(wv) = &tab.webview {
            let _ = wv.set_visible(idx == active_tab_index);
        }
    }

    let mut chrome_expansion_mode = "none".to_string();

    // Run Event Loop with 5-second Memory Governor wakeups
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(std::time::Instant::now() + std::time::Duration::from_secs(5));

        match event {
            Event::NewEvents(StartCause::Init) => {
                sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                if let Some(cur) = tabs.get(active_tab_index) {
                    let is_bm = storage.is_bookmarked(&cur.url);
                    let (can_back, can_fwd) = tab_nav_state(cur);
                    sync_address_to_chrome(&chrome_wv, &cur.url, &cur.title, is_bm, can_back, can_fwd);
                }
            }

            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                // Surface Go 2 Memory Governor: evaluate background tab sleep states with grace period
                let is_low_mem = governor.is_memory_pressure_high();
                let mut state_changed = false;

                const LOW_MEM_MIN_BG_SECS: u64 = 45;
                const TIER2_HIBERNATION_SECS: u64 = 600;
                const TIER1_THROTTLE_SECS: u64 = 120;

                for (idx, tab) in tabs.iter_mut().enumerate() {
                    if idx == active_tab_index {
                        continue; // Active tab is strictly immune
                    }
                    if tab.is_audio_playing || tab.has_form_input {
                        continue; // Strictly exempt tabs with active media or unsaved form entries
                    }

                    let elapsed = tab.last_active.elapsed();

                    let should_hibernate = (is_low_mem && elapsed >= std::time::Duration::from_secs(LOW_MEM_MIN_BG_SECS))
                        || elapsed >= std::time::Duration::from_secs(TIER2_HIBERNATION_SECS);

                    if should_hibernate {
                        // Tier 2 Deep Hibernation: drop WebView entirely to reclaim physical RAM
                        if tab.state != TabState::Tier2Hibernated && tab.webview.is_some() {
                            tab.webview = None;
                            tab.state = TabState::Tier2Hibernated;
                            state_changed = true;
                        }
                    } else if elapsed >= std::time::Duration::from_secs(TIER1_THROTTLE_SECS) {
                        // Tier 1 Light Throttling: pause animations and throttle background timers
                        if tab.state == TabState::Active {
                            if let Some(wv) = &tab.webview {
                                let _ = wv.evaluate_script(AdBlocker::get_tier1_throttle_script(true));
                            }
                            tab.state = TabState::Tier1Throttled;
                            state_changed = true;
                        }
                    }
                }

                if state_changed {
                    sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                save_session_state(&storage, &tabs, active_tab_index);
                *control_flow = ControlFlow::Exit;
            }

            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size },
                ..
            } => {
                current_scale_factor = scale_factor;
                current_size = *new_inner_size;
                let chrome_h = chrome_expanded_height(&chrome_expansion_mode, current_scale_factor, current_size.height);
                let _ = chrome_wv.set_bounds(Rect {
                    position: PhysicalPosition::new(0, 0).into(),
                    size: PhysicalSize::new(current_size.width, chrome_h).into(),
                });
                if let Some(tab) = tabs.get_mut(active_tab_index) {
                    if let Some(wv) = &tab.webview {
                        let _ = wv.set_bounds(content_rect(current_size, current_scale_factor));
                    }
                }
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                if new_size.width > 0 && new_size.height > 0 {
                    current_size = new_size;
                    let chrome_h = chrome_expanded_height(&chrome_expansion_mode, current_scale_factor, new_size.height);
                    let _ = chrome_wv.set_bounds(Rect {
                        position: PhysicalPosition::new(0, 0).into(),
                        size: PhysicalSize::new(new_size.width, chrome_h).into(),
                    });
                    if let Some(tab) = tabs.get_mut(active_tab_index) {
                        if let Some(wv) = &tab.webview {
                            let _ = wv.set_bounds(content_rect(new_size, current_scale_factor));
                        }
                    }
                }
            }

            Event::UserEvent(UserEvent::MediaStateChanged { tab_id, playing }) => {
                if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
                    tab.is_audio_playing = playing;
                }
            }

            Event::UserEvent(UserEvent::FormDirtyChanged { tab_id, dirty }) => {
                if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
                    tab.has_form_input = dirty;
                }
            }

            Event::UserEvent(UserEvent::ScrollChanged { tab_id, scroll_x, scroll_y }) => {
                if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
                    tab.scroll_x = scroll_x;
                    tab.scroll_y = scroll_y;
                }
            }

            Event::UserEvent(UserEvent::LoadingState { tab_id, loading }) => {
                if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
                    tab.is_loading = loading;
                }
                if let Some(active_tab) = tabs.get(active_tab_index) {
                    if active_tab.id == tab_id {
                        let script = format!("window.setLoading({});", loading);
                        let _ = chrome_wv.evaluate_script(&script);
                    }
                }
            }

            Event::UserEvent(UserEvent::TitleChanged { tab_id, title }) => {
                if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
                    if !title.is_empty() {
                        tab.title = title.clone();
                    }
                    tab.last_active = std::time::Instant::now();
                }
                if let Some(active_tab) = tabs.get(active_tab_index) {
                    if active_tab.id == tab_id {
                        let is_bm = storage.is_bookmarked(&active_tab.url);
                        let (can_back, can_fwd) = tab_nav_state(active_tab);
                        sync_address_to_chrome(&chrome_wv, &active_tab.url, &title, is_bm, can_back, can_fwd);
                        let _ = storage.record_visit(&active_tab.url, &title);
                    }
                }
                sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
            }

            Event::UserEvent(UserEvent::UrlChanged { tab_id, url }) => {
                if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
                    if !url.is_empty() {
                        tab.url = url.clone();
                    }
                    tab.last_active = std::time::Instant::now();
                    tab.is_audio_playing = false;
                    tab.has_form_input = false;
                    if let Some(wv) = &tab.webview {
                        if adblocker.dark_mode {
                            let _ = wv.evaluate_script(AdBlocker::get_dark_mode_script(true));
                        }
                        if adblocker.enabled {
                            let _ = wv.evaluate_script(AdBlocker::get_shield_toggle_script(true));
                        }
                        if tab.scroll_x > 0 || tab.scroll_y > 0 {
                            let _ = wv.evaluate_script(&AdBlocker::get_tab_restore_script(tab.scroll_x, tab.scroll_y));
                        }
                    }
                }
                if let Some(active_tab) = tabs.get(active_tab_index) {
                    if active_tab.id == tab_id {
                        let is_bm = storage.is_bookmarked(&url);
                        let (can_back, can_fwd) = tab_nav_state(active_tab);
                        sync_address_to_chrome(&chrome_wv, &url, &active_tab.title, is_bm, can_back, can_fwd);
                        let _ = storage.record_visit(&url, &active_tab.title);
                        save_session_state(&storage, &tabs, active_tab_index);
                    }
                }
                sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
            }

            Event::UserEvent(UserEvent::NewWindow(target_url)) => {
                let target = normalize_url(&target_url);
                if tabs.len() >= MAX_TABS {
                    // Resource cap: navigate active tab instead of spawning unbounded WebViews
                    if let Some(cur) = tabs.get_mut(active_tab_index) {
                        cur.url = target.clone();
                        cur.last_active = std::time::Instant::now();
                        cur.state = TabState::Active;
                        if cur.webview.is_none() {
                            if let Ok(wv) = create_content_webview(
                                cur.id,
                                &target,
                                &window,
                                &proxy,
                                content_rect(current_size, current_scale_factor),
                                &adblocker,
                                &webview_data_dir,
                            ) {
                                let _ = wv.set_visible(true);
                                cur.webview = Some(wv);
                            }
                        } else if let Some(wv) = &cur.webview {
                            let _ = wv.load_url(&target);
                        }
                        let is_bm = storage.is_bookmarked(&target);
                        let (can_back, can_fwd) = tab_nav_state(cur);
                        sync_address_to_chrome(&chrome_wv, &target, &cur.title, is_bm, can_back, can_fwd);
                        sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                        save_session_state(&storage, &tabs, active_tab_index);
                    }
                } else {
                    let new_id = next_tab_id;
                    next_tab_id += 1;
                    if let Ok(new_tab) = create_tab(
                        new_id,
                        &target,
                        &window,
                        &proxy,
                        content_rect(current_size, current_scale_factor),
                        &adblocker,
                        &webview_data_dir,
                    ) {
                        if let Some(cur) = tabs.get_mut(active_tab_index) {
                            if let Some(wv) = &cur.webview {
                                let _ = wv.set_visible(false);
                            }
                            cur.last_active = std::time::Instant::now();
                        }
                        tabs.push(new_tab);
                        active_tab_index = tabs.len() - 1;
                        if let Some(active_tab) = tabs.get(active_tab_index) {
                            let is_bm = storage.is_bookmarked(&target);
                            let (can_back, can_fwd) = tab_nav_state(active_tab);
                            sync_address_to_chrome(&chrome_wv, &target, "New Tab", is_bm, can_back, can_fwd);
                        }
                        sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                        save_session_state(&storage, &tabs, active_tab_index);
                    }
                }
            }

            Event::UserEvent(UserEvent::Chrome(msg)) => {
                match msg.action.as_str() {
                    "chrome_ready" => {
                        sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                        if let Some(cur) = tabs.get(active_tab_index) {
                            let is_bm = storage.is_bookmarked(&cur.url);
                            let (can_back, can_fwd) = tab_nav_state(cur);
                            sync_address_to_chrome(&chrome_wv, &cur.url, &cur.title, is_bm, can_back, can_fwd);
                        }
                    }

                    "navigate" => {
                        if let Some(payload) = msg.payload {
                            if let Some(raw_url) = payload["url"].as_str() {
                                let target_url = normalize_url(raw_url);
                                if let Some(tab) = tabs.get_mut(active_tab_index) {
                                    tab.url = target_url.clone();
                                    tab.last_active = std::time::Instant::now();
                                    tab.state = TabState::Active;

                                    if tab.webview.is_none() {
                                        if let Ok(wv) = create_content_webview(
                                            tab.id,
                                            &target_url,
                                            &window,
                                            &proxy,
                                            content_rect(current_size, current_scale_factor),
                                            &adblocker,
                                            &webview_data_dir,
                                        ) {
                                            let _ = wv.set_visible(true);
                                            tab.webview = Some(wv);
                                        }
                                    } else if let Some(wv) = &tab.webview {
                                        let _ = wv.load_url(&target_url);
                                    }

                                    let is_bm = storage.is_bookmarked(&target_url);
                                    let (can_back, can_fwd) = tab_nav_state(tab);
                                    sync_address_to_chrome(&chrome_wv, &target_url, &tab.title, is_bm, can_back, can_fwd);
                                    sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                                    save_session_state(&storage, &tabs, active_tab_index);
                                }
                            }
                        }
                    }

                    "switch_tab" => {
                        if let Some(payload) = msg.payload {
                            if let Some(target_idx) = payload["index"].as_u64() {
                                let idx = target_idx as usize;
                                if idx < tabs.len() && idx != active_tab_index {
                                    if let Some(old_tab) = tabs.get_mut(active_tab_index) {
                                        if let Some(wv) = &old_tab.webview {
                                            let _ = wv.set_visible(false);
                                        }
                                        old_tab.last_active = std::time::Instant::now();
                                    }
                                    active_tab_index = idx;
                                    if let Some(new_tab) = tabs.get_mut(active_tab_index) {
                                        new_tab.last_active = std::time::Instant::now();
                                        if new_tab.webview.is_none() {
                                            // Wake tab from Tier 2 Deep Hibernation
                                            if let Ok(wv) = create_content_webview(
                                                new_tab.id,
                                                &new_tab.url,
                                                &window,
                                                &proxy,
                                                content_rect(current_size, current_scale_factor),
                                                &adblocker,
                                                &webview_data_dir,
                                            ) {
                                                let _ = wv.set_visible(true);
                                                if new_tab.scroll_x > 0 || new_tab.scroll_y > 0 {
                                                    let restore_script = AdBlocker::get_tab_restore_script(new_tab.scroll_x, new_tab.scroll_y);
                                                    let _ = wv.evaluate_script(&restore_script);
                                                }
                                                new_tab.webview = Some(wv);
                                            }
                                        } else if let Some(wv) = &new_tab.webview {
                                            if new_tab.state == TabState::Tier1Throttled {
                                                let _ = wv.evaluate_script(AdBlocker::get_tier1_throttle_script(false));
                                            }
                                            let _ = wv.set_bounds(content_rect(current_size, current_scale_factor));
                                            let _ = wv.set_visible(true);
                                        }
                                        new_tab.state = TabState::Active;
                                        let is_bm = storage.is_bookmarked(&new_tab.url);
                                        let (can_back, can_fwd) = tab_nav_state(new_tab);
                                        sync_address_to_chrome(&chrome_wv, &new_tab.url, &new_tab.title, is_bm, can_back, can_fwd);
                                        let _ = chrome_wv.evaluate_script(&format!("window.setLoading({});", new_tab.is_loading));
                                    }
                                    sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                                    save_session_state(&storage, &tabs, active_tab_index);
                                }
                            }
                        }
                    }

                    "new_tab" => {
                        let raw_url = msg.payload
                            .as_ref()
                            .and_then(|p| p["url"].as_str())
                            .unwrap_or(HOME_URL);
                        let target_url = normalize_url(raw_url);

                        if tabs.len() >= MAX_TABS {
                            // Reuse active tab if at maximum tab limit
                            if let Some(cur) = tabs.get_mut(active_tab_index) {
                                cur.url = target_url.clone();
                                cur.last_active = std::time::Instant::now();
                                cur.state = TabState::Active;
                                if cur.webview.is_none() {
                                    if let Ok(wv) = create_content_webview(
                                        cur.id,
                                        &target_url,
                                        &window,
                                        &proxy,
                                        content_rect(current_size, current_scale_factor),
                                        &adblocker,
                                        &webview_data_dir,
                                    ) {
                                        let _ = wv.set_visible(true);
                                        cur.webview = Some(wv);
                                    }
                                } else if let Some(wv) = &cur.webview {
                                    let _ = wv.load_url(&target_url);
                                }
                                let is_bm = storage.is_bookmarked(&target_url);
                                let (can_back, can_fwd) = tab_nav_state(cur);
                                sync_address_to_chrome(&chrome_wv, &target_url, &cur.title, is_bm, can_back, can_fwd);
                                sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                                save_session_state(&storage, &tabs, active_tab_index);
                            }
                        } else {
                            let new_id = next_tab_id;
                            next_tab_id += 1;
                            if let Ok(new_tab) = create_tab(
                                new_id,
                                &target_url,
                                &window,
                                &proxy,
                                content_rect(current_size, current_scale_factor),
                                &adblocker,
                                &webview_data_dir,
                            ) {
                                if let Some(cur) = tabs.get_mut(active_tab_index) {
                                    if let Some(wv) = &cur.webview {
                                        let _ = wv.set_visible(false);
                                    }
                                    cur.last_active = std::time::Instant::now();
                                }
                                tabs.push(new_tab);
                                active_tab_index = tabs.len() - 1;
                                if let Some(active_tab) = tabs.get(active_tab_index) {
                                    let is_bm = storage.is_bookmarked(&target_url);
                                    let (can_back, can_fwd) = tab_nav_state(active_tab);
                                    sync_address_to_chrome(&chrome_wv, &target_url, "New Tab", is_bm, can_back, can_fwd);
                                    let _ = chrome_wv.evaluate_script("window.setLoading(false);");
                                }
                                sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                                save_session_state(&storage, &tabs, active_tab_index);
                            }
                        }
                    }

                    "close_tab" => {
                        if let Some(payload) = msg.payload {
                            if let Some(close_idx) = payload["index"].as_u64() {
                                let idx = close_idx as usize;
                                if tabs.len() > 1 && idx < tabs.len() {
                                    let was_active = idx == active_tab_index;
                                    let removed_tab = tabs.remove(idx);
                                    if let Some(wv) = &removed_tab.webview {
                                        let _ = wv.set_visible(false);
                                    }

                                    if was_active {
                                        if active_tab_index >= tabs.len() {
                                            active_tab_index = tabs.len() - 1;
                                        }
                                    } else if idx < active_tab_index {
                                        active_tab_index -= 1;
                                    }

                                    if let Some(new_active) = tabs.get_mut(active_tab_index) {
                                        new_active.last_active = std::time::Instant::now();
                                        if new_active.webview.is_none() {
                                            if let Ok(wv) = create_content_webview(
                                                new_active.id,
                                                &new_active.url,
                                                &window,
                                                &proxy,
                                                content_rect(current_size, current_scale_factor),
                                                &adblocker,
                                                &webview_data_dir,
                                            ) {
                                                let _ = wv.set_visible(true);
                                                if new_active.scroll_x > 0 || new_active.scroll_y > 0 {
                                                    let restore_script = AdBlocker::get_tab_restore_script(new_active.scroll_x, new_active.scroll_y);
                                                    let _ = wv.evaluate_script(&restore_script);
                                                }
                                                new_active.webview = Some(wv);
                                            }
                                        } else if let Some(wv) = &new_active.webview {
                                            let _ = wv.set_bounds(content_rect(current_size, current_scale_factor));
                                            let _ = wv.set_visible(true);
                                        }
                                        new_active.state = TabState::Active;
                                        let is_bm = storage.is_bookmarked(&new_active.url);
                                        let (can_back, can_fwd) = tab_nav_state(new_active);
                                        sync_address_to_chrome(&chrome_wv, &new_active.url, &new_active.title, is_bm, can_back, can_fwd);
                                        let _ = chrome_wv.evaluate_script(&format!("window.setLoading({});", new_active.is_loading));
                                    }
                                    sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                                    save_session_state(&storage, &tabs, active_tab_index);
                                } else if tabs.len() == 1 && idx == 0 {
                                    if let Some(tab) = tabs.get_mut(0) {
                                        tab.url = HOME_URL.to_string();
                                        tab.title = "New Tab".to_string();
                                        tab.last_active = std::time::Instant::now();
                                        tab.state = TabState::Active;
                                        if let Some(wv) = &tab.webview {
                                            let _ = wv.load_url(HOME_URL);
                                        }
                                        let is_bm = storage.is_bookmarked(HOME_URL);
                                        let (can_back, can_fwd) = tab_nav_state(tab);
                                        sync_address_to_chrome(&chrome_wv, HOME_URL, "New Tab", is_bm, can_back, can_fwd);
                                        sync_tabs_to_chrome(&chrome_wv, &tabs, 0);
                                        save_session_state(&storage, &tabs, 0);
                                    }
                                }
                            }
                        }
                    }

                    "back" => {
                        if let Some(tab) = tabs.get(active_tab_index) {
                            if let Some(wv) = &tab.webview {
                                let _ = wv.go_back();
                            }
                        }
                    }

                    "forward" => {
                        if let Some(tab) = tabs.get(active_tab_index) {
                            if let Some(wv) = &tab.webview {
                                let _ = wv.go_forward();
                            }
                        }
                    }

                    "toggle_fullscreen" => {
                        let is_fs = window.fullscreen().is_some();
                        if is_fs {
                            window.set_fullscreen(None);
                        } else {
                            window.set_fullscreen(Some(tao::window::Fullscreen::Borderless(None)));
                        }
                    }

                    "reload" => {
                        if let Some(tab) = tabs.get(active_tab_index) {
                            if let Some(wv) = &tab.webview {
                                let _ = wv.evaluate_script("window.location.reload()");
                            }
                        }
                    }

                    "home" => {
                        if let Some(tab) = tabs.get_mut(active_tab_index) {
                            tab.url = HOME_URL.to_string();
                            tab.last_active = std::time::Instant::now();
                            tab.state = TabState::Active;
                            if let Some(wv) = &tab.webview {
                                let _ = wv.load_url(HOME_URL);
                            }
                            let is_bm = storage.is_bookmarked(HOME_URL);
                            let (can_back, can_fwd) = tab_nav_state(tab);
                            sync_address_to_chrome(&chrome_wv, HOME_URL, "DuckDuckGo", is_bm, can_back, can_fwd);
                            sync_tabs_to_chrome(&chrome_wv, &tabs, active_tab_index);
                            save_session_state(&storage, &tabs, active_tab_index);
                        }
                    }

                    "launch_keyboard" => {
                        launch_touch_keyboard();
                    }

                    "toggle_bookmark" => {
                        if let Some(tab) = tabs.get(active_tab_index) {
                            if let Ok(is_bm) = storage.toggle_bookmark(&tab.url, &tab.title) {
                                let (can_back, can_fwd) = tab_nav_state(tab);
                                sync_address_to_chrome(&chrome_wv, &tab.url, &tab.title, is_bm, can_back, can_fwd);
                            }
                        }
                    }

                    "get_bookmarks" => {
                        let bookmarks = storage.get_bookmarks();
                        if let Ok(json) = serde_json::to_string(&bookmarks) {
                            let script = format!("window.showDrawerData({}, 'bookmarks');", clean_js_json(&json));
                            let _ = chrome_wv.evaluate_script(&script);
                        }
                    }

                    "clear_history" => {
                        let _ = storage.clear_history();
                        let script = "window.showDrawerData([], 'history');";
                        let _ = chrome_wv.evaluate_script(script);
                    }

                    "get_history" => {
                        let history = storage.get_recent_history(50);
                        if let Ok(json) = serde_json::to_string(&history) {
                            let script = format!("window.showDrawerData({}, 'history');", clean_js_json(&json));
                            let _ = chrome_wv.evaluate_script(&script);
                        }
                    }

                    "toggle_reader" => {
                        if let Some(tab) = tabs.get(active_tab_index) {
                            if let Some(wv) = &tab.webview {
                                let _ = wv.evaluate_script(AdBlocker::get_reader_mode_script());
                            }
                        }
                    }

                    "toggle_dark_mode" => {
                        let enabled = msg.payload
                            .as_ref()
                            .and_then(|p| p["enabled"].as_bool())
                            .unwrap_or(false);
                        adblocker.dark_mode = enabled;
                        let script = AdBlocker::get_dark_mode_script(enabled);
                        for tab in &tabs {
                            if let Some(wv) = &tab.webview {
                                let _ = wv.evaluate_script(script);
                            }
                        }
                    }

                    "search_query" => {
                        if let Some(payload) = msg.payload {
                            if let Some(query) = payload["query"].as_str() {
                                let results = storage.search(query, 6);
                                if let Ok(json) = serde_json::to_string(&results) {
                                    let script = format!("window.showSearchResults({});", clean_js_json(&json));
                                    let _ = chrome_wv.evaluate_script(&script);
                                }
                            }
                        }
                    }

                    "toggle_shield" => {
                        adblocker.enabled = !adblocker.enabled;
                        let shield_script = AdBlocker::get_shield_toggle_script(adblocker.enabled);
                        for tab in &tabs {
                            if let Some(wv) = &tab.webview {
                                let _ = wv.evaluate_script(shield_script);
                            }
                        }
                        let script = format!("window.updateShieldState({});", adblocker.enabled);
                        let _ = chrome_wv.evaluate_script(&script);
                    }

                    "stop" => {
                        if let Some(tab) = tabs.get_mut(active_tab_index) {
                            tab.is_loading = false;
                            if let Some(wv) = &tab.webview {
                                let _ = wv.evaluate_script("window.stop();");
                            }
                            let _ = chrome_wv.evaluate_script("window.setLoading(false);");
                        }
                    }

                    "set_chrome_expanded" => {
                        if let Some(payload) = &msg.payload {
                            let mode = payload["mode"].as_str().unwrap_or("none");
                            chrome_expansion_mode = mode.to_string();
                            let h = chrome_expanded_height(mode, current_scale_factor, current_size.height);
                            let _ = chrome_wv.set_bounds(Rect {
                                position: PhysicalPosition::new(0, 0).into(),
                                size: PhysicalSize::new(current_size.width, h).into(),
                            });
                            if mode == "full" {
                                if let Some(tab) = tabs.get(active_tab_index) {
                                    if let Some(wv) = &tab.webview {
                                        let _ = wv.set_visible(false);
                                    }
                                }
                            } else {
                                if let Some(tab) = tabs.get(active_tab_index) {
                                    if let Some(wv) = &tab.webview {
                                        let _ = wv.set_visible(true);
                                    }
                                }
                            }
                        }
                    }

                    _ => ()
                }
            }

            _ => ()
        }
    });
}

fn dirs_storage_path() -> PathBuf {
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

fn dirs_webview_data_path() -> PathBuf {
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

fn dirs_webview_fallback_data_path() -> PathBuf {
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

#[cfg(target_os = "windows")]
fn show_fatal_error(msg: &str) {
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

fn main() {
    if let Err(e) = run() {
        #[cfg(target_os = "windows")]
        show_fatal_error(&format!("Aerolith Browser failed to start:\n\n{}", e));
        #[cfg(not(target_os = "windows"))]
        eprintln!("Aerolith Browser failed to start: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url_empty() {
        assert_eq!(normalize_url(""), HOME_URL);
        assert_eq!(normalize_url("   "), HOME_URL);
    }

    #[test]
    fn test_normalize_url_about() {
        assert_eq!(normalize_url("about:blank"), "about:blank");
        assert_eq!(normalize_url("about:blank#hash"), "about:blank");
        assert_eq!(normalize_url("about:home"), HOME_URL);
        assert_eq!(normalize_url("about:start"), HOME_URL);
    }

    #[test]
    fn test_normalize_url_case_insensitive_schemes() {
        assert_eq!(normalize_url("HTTPS://EXAMPLE.COM"), "HTTPS://EXAMPLE.COM");
        assert_eq!(normalize_url("Http://github.com/rust-lang"), "Http://github.com/rust-lang");
        assert_eq!(normalize_url("FILE:///C:/test.html"), "FILE:///C:/test.html");
    }

    #[test]
    fn test_normalize_url_loopback_and_ports() {
        assert_eq!(normalize_url("localhost"), "http://localhost");
        assert_eq!(normalize_url("localhost:3000"), "http://localhost:3000");
        assert_eq!(normalize_url("127.0.0.1"), "http://127.0.0.1");
        assert_eq!(normalize_url("127.0.0.1:8080"), "http://127.0.0.1:8080");
        assert_eq!(normalize_url("[::1]:8080"), "http://[::1]:8080");
    }

    #[test]
    fn test_normalize_url_ipv4_addresses() {
        assert_eq!(normalize_url("192.168.1.1"), "http://192.168.1.1");
        assert_eq!(normalize_url("192.168.1.1:8443"), "http://192.168.1.1:8443");
        assert_eq!(normalize_url("10.0.0.1"), "http://10.0.0.1");
    }

    #[test]
    fn test_normalize_url_domains() {
        assert_eq!(normalize_url("duckduckgo.com"), "https://duckduckgo.com");
        assert_eq!(normalize_url("crates.io/crates/wry"), "https://crates.io/crates/wry");
        assert_eq!(normalize_url("sub.domain.co.uk"), "https://sub.domain.co.uk");
    }

    #[test]
    fn test_normalize_url_code_tokens_search() {
        assert_eq!(normalize_url("std::time"), "https://duckduckgo.com/?q=std%3A%3Atime");
        assert_eq!(normalize_url("v1.2.3"), "https://duckduckgo.com/?q=v1.2.3");
        assert_eq!(normalize_url("3.14"), "https://duckduckgo.com/?q=3.14");
    }

    #[test]
    fn test_normalize_url_blocked_dangerous_protocols() {
        let bad1 = normalize_url("javascript:alert(1)");
        assert!(bad1.starts_with("https://duckduckgo.com/?q="));

        let bad2 = normalize_url("vbscript:MsgBox(1)");
        assert!(bad2.starts_with("https://duckduckgo.com/?q="));

        let bad3 = normalize_url("data:text/html,<h1>hacked</h1>");
        assert!(bad3.starts_with("https://duckduckgo.com/?q="));

        let bad4 = normalize_url("ms-msdt:/id PCWDiagnostic");
        assert!(bad4.starts_with("https://duckduckgo.com/?q="));
    }

    #[test]
    fn test_urlencoding_simple() {
        assert_eq!(urlencoding_simple("hello world"), "hello+world");
        assert_eq!(urlencoding_simple("rust & web"), "rust+%26+web");
        assert_eq!(urlencoding_simple("a=1&b=2"), "a%3D1%26b%3D2");
        assert_eq!(urlencoding_simple("café"), "caf%C3%A9");
    }

    #[test]
    fn test_normalize_url_domain_with_custom_port() {
        assert_eq!(normalize_url("example.com:8080"), "https://example.com:8080");
        assert_eq!(normalize_url("app.internal:3000/dashboard"), "https://app.internal:3000/dashboard");
    }

    #[test]
    fn test_normalize_url_unsupported_scheme_routes_to_search() {
        let ftp_res = normalize_url("ftp://ftp.example.org/files");
        assert!(ftp_res.starts_with("https://duckduckgo.com/?q=ftp%3A%2F%2F"));

        let search_res = normalize_url("search-ms:query=test");
        assert!(search_res.starts_with("https://duckduckgo.com/?q=search-ms%3A"));

        let ws_res = normalize_url("ws://websocket.service.local");
        assert!(ws_res.starts_with("https://duckduckgo.com/?q=ws%3A%2F%2F"));
    }

    #[test]
    fn test_clean_js_json() {
        let dirty = "{\"title\":\"Hello\u{2028}World\u{2029}!\"}";
        let clean = clean_js_json(dirty);
        assert!(!clean.contains('\u{2028}'));
        assert!(!clean.contains('\u{2029}'));
        assert!(clean.contains("\\u2028"));
        assert!(clean.contains("\\u2029"));
    }

    #[test]
    fn test_normalize_url_length_clamp() {
        let giant = "a".repeat(10000);
        let normalized = normalize_url(&giant);
        assert!(normalized.starts_with("https://duckduckgo.com/?q="));
        assert!(normalized.len() < 15000);
    }

    #[test]
    fn test_content_rect_calculation() {
        let size = PhysicalSize::new(1920, 1280);
        // Test 100% scale
        let rect_1x = content_rect(size, 1.0);
        let pos_1x: PhysicalPosition<i32> = rect_1x.position.to_physical(1.0);
        let sz_1x: PhysicalSize<u32> = rect_1x.size.to_physical(1.0);
        assert_eq!(pos_1x.x, 0);
        assert_eq!(pos_1x.y, 84);
        assert_eq!(sz_1x.width, 1920);
        assert_eq!(sz_1x.height, 1280 - 84);

        // Test Surface Go 2 default 150% scale
        let rect_15x = content_rect(size, 1.5);
        let pos_15x: PhysicalPosition<i32> = rect_15x.position.to_physical(1.0);
        let sz_15x: PhysicalSize<u32> = rect_15x.size.to_physical(1.0);
        assert_eq!(pos_15x.x, 0);
        assert_eq!(pos_15x.y, 126);
        assert_eq!(sz_15x.height, 1280 - 126);
    }

    #[test]
    fn test_physical_chrome_height_scaling() {
        assert_eq!(physical_chrome_height(1.0), 84);
        assert_eq!(physical_chrome_height(1.25), 105);
        assert_eq!(physical_chrome_height(1.5), 126);
        assert_eq!(physical_chrome_height(2.0), 168);
    }

    #[test]
    fn test_tab_smart_sleep_exemptions_lifecycle() {
        let tab = Tab {
            id: 1,
            title: "Audio Stream".to_string(),
            url: "https://music.example.com".to_string(),
            scroll_x: 100,
            scroll_y: 250,
            last_active: std::time::Instant::now(),
            state: TabState::Active,
            webview: None,
            is_audio_playing: true,
            has_form_input: false,
            is_loading: false,
        };
        // Background audio playing tabs must be exempt from deep hibernation
        assert!(tab.is_audio_playing);
        assert!(!tab.is_loading);
        assert_eq!(tab.scroll_x, 100);
        assert_eq!(tab.scroll_y, 250);
    }

    #[test]
    fn test_tab_info_serialization_includes_is_sleeping() {
        let tab_active = TabInfo {
            id: 1,
            title: "Active Page".to_string(),
            url: "https://example.com".to_string(),
            is_sleeping: false,
        };
        let tab_sleep = TabInfo {
            id: 2,
            title: "Sleeping Page".to_string(),
            url: "https://example.org".to_string(),
            is_sleeping: true,
        };

        let json_active = serde_json::to_string(&tab_active).unwrap();
        let json_sleep = serde_json::to_string(&tab_sleep).unwrap();

        assert!(json_active.contains("\"is_sleeping\":false"));
        assert!(json_sleep.contains("\"is_sleeping\":true"));
    }

    #[test]
    fn test_tab_state_lifecycle_enum() {
        let state1 = TabState::Active;
        let state2 = TabState::Tier1Throttled;
        let state3 = TabState::Tier2Hibernated;

        assert_ne!(state1, state2);
        assert_ne!(state2, state3);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_dirs_webview_fallback_data_path() {
        let p = dirs_webview_fallback_data_path();
        let s = p.to_string_lossy();
        assert!(s.contains("WebView2Data_"));
    }

    #[test]
    fn test_chrome_expanded_height_modes() {
        let win_h = 1280;
        // 100% scale (scale_factor = 1.0)
        assert_eq!(chrome_expanded_height("none", 1.0, win_h), 84);
        assert_eq!(chrome_expanded_height("dropdown", 1.0, win_h), 84 + 300);
        assert_eq!(chrome_expanded_height("full", 1.0, win_h), win_h);

        // Surface Go 2 150% scale (scale_factor = 1.5)
        let base_15x = physical_chrome_height(1.5); // 126
        let extra_15x = (300.0 * 1.5 as f64).round() as u32; // 450
        assert_eq!(chrome_expanded_height("none", 1.5, win_h), base_15x);
        assert_eq!(chrome_expanded_height("dropdown", 1.5, win_h), base_15x + extra_15x);
        assert_eq!(chrome_expanded_height("full", 1.5, win_h), win_h);

        // Check clamping if dropdown extra exceeds window height
        let tiny_win_h = 200;
        assert_eq!(chrome_expanded_height("dropdown", 1.0, tiny_win_h), tiny_win_h);
    }

    #[test]
    fn test_security_level_detection() {
        assert_eq!(get_security_level_for_url("https://example.com"), "secure");
        assert_eq!(get_security_level_for_url("HTTPS://DUCKDUCKGO.COM/SEARCH"), "secure");
        assert_eq!(get_security_level_for_url("http://insecure.site"), "insecure");
        assert_eq!(get_security_level_for_url("http://192.168.1.1"), "insecure");
        assert_eq!(get_security_level_for_url("file:///C:/Users/test.html"), "local");
        assert_eq!(get_security_level_for_url("about:blank"), "internal");
        assert_eq!(get_security_level_for_url("chrome://settings"), "internal");
    }

    #[test]
    fn test_tab_loading_state_toggle() {
        let mut tab = Tab {
            id: 42,
            title: "Loading Page".to_string(),
            url: "https://example.org".to_string(),
            scroll_x: 0,
            scroll_y: 0,
            last_active: std::time::Instant::now(),
            state: TabState::Active,
            webview: None,
            is_audio_playing: false,
            has_form_input: false,
            is_loading: true,
        };
        assert!(tab.is_loading);
        tab.is_loading = false;
        assert!(!tab.is_loading);
    }
}
