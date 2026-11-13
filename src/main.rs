#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adblock;
mod geometry;
mod governor;
mod paths;
mod storage;
mod tabs;
mod types;
mod ui;
mod url_utils;
mod webview_factory;

use adblock::AdBlocker;
use geometry::{chrome_expanded_height, content_rect, physical_chrome_height, reassert_chrome_bounds};
use governor::MemoryGovernor;
use paths::{dirs_storage_path, dirs_webview_data_path, launch_touch_keyboard, show_fatal_error};
use storage::BrowserStorage;
use tabs::{create_tab, TabManager};
use types::{ChromeMsg, UserEvent, DEFAULT_HEIGHT, DEFAULT_WIDTH, HOME_URL};
use url_utils::{clean_js_json, sanitize_title};
use webview_factory::create_chrome_webview;

#[cfg(all(target_os = "windows", target_env = "gnu"))]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn RoOriginateErrorW(_error: i32, _cchmax: u32, _message: *const u16) -> i32 {
    0
}

use std::path::PathBuf;
use tao::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy},
    window::{Window, WindowBuilder},
};
use wry::{Rect, WebView};

fn handle_chrome_msg(
    msg: ChromeMsg,
    tab_mgr: &mut TabManager,
    chrome_wv: &WebView,
    window: &Window,
    proxy: &EventLoopProxy<UserEvent>,
    adblocker: &mut AdBlocker,
    storage: &BrowserStorage,
    webview_data_dir: &PathBuf,
    chrome_expansion_mode: &mut String,
    current_scale_factor: f64,
    current_size: PhysicalSize<u32>,
) {
    let bounds = content_rect(current_size, current_scale_factor);

    match msg.action.as_str() {
        "chrome_ready" => {
            tab_mgr.sync_tabs_to_chrome(chrome_wv);
            tab_mgr.sync_address_to_chrome(chrome_wv, storage);
            let _ = chrome_wv.evaluate_script(&format!("window.updateShortFormState({});", adblocker.block_short_form));
            let engine = storage.get_setting_string("search_engine", "duckduckgo");
            let clear = storage.get_setting_bool("clear_on_exit", false);
            let free_mb = MemoryGovernor::get_available_mb();
            let load_pct = MemoryGovernor::get_memory_load_pct();
            let script = format!(
                "window.updateSettingsData('{}', {}, {}, {});",
                engine, clear, free_mb, load_pct
            );
            let _ = chrome_wv.evaluate_script(&script);
        }

        "navigate" => {
            if let Some(payload) = msg.payload {
                if let Some(raw_url) = payload["url"].as_str() {
                    tab_mgr.navigate(raw_url, window, proxy, bounds, adblocker, webview_data_dir, chrome_wv, storage);
                    reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
                }
            }
        }

        "switch_tab" => {
            if let Some(payload) = msg.payload {
                if let Some(target_idx) = payload["index"].as_u64() {
                    tab_mgr.switch_tab(target_idx as usize, window, proxy, bounds, adblocker, webview_data_dir, chrome_wv, storage);
                    reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
                }
            }
        }

        "new_tab" => {
            let raw_url = msg.payload
                .as_ref()
                .and_then(|p| p["url"].as_str())
                .unwrap_or(HOME_URL);
            tab_mgr.new_tab(raw_url, window, proxy, bounds, adblocker, webview_data_dir, chrome_wv, storage);
            reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
        }

        "close_tab" => {
            if let Some(payload) = msg.payload {
                if let Some(close_idx) = payload["index"].as_u64() {
                    tab_mgr.close_tab(close_idx as usize, window, proxy, bounds, adblocker, webview_data_dir, chrome_wv, storage);
                    reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
                }
            }
        }

        "reopen_closed_tab" => {
            tab_mgr.reopen_closed_tab(window, proxy, bounds, adblocker, webview_data_dir, chrome_wv, storage);
            reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
        }

        "duplicate_tab" => {
            if let Some(payload) = &msg.payload {
                if let Some(idx) = payload["index"].as_u64() {
                    tab_mgr.duplicate_tab(idx as usize, window, proxy, bounds, adblocker, webview_data_dir, chrome_wv, storage);
                    reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
                }
            }
        }

        "reload_tab" => {
            if let Some(payload) = &msg.payload {
                if let Some(idx) = payload["index"].as_u64() {
                    if let Some(tab) = tab_mgr.get(idx as usize) {
                        if let Some(wv) = &tab.webview {
                            let _ = wv.evaluate_script("window.location.reload()");
                        }
                    }
                }
            }
        }

        "close_other_tabs" => {
            if let Some(payload) = &msg.payload {
                if let Some(idx) = payload["index"].as_u64() {
                    tab_mgr.close_other_tabs(
                        idx as usize,
                        window,
                        proxy,
                        bounds,
                        adblocker,
                        webview_data_dir,
                        chrome_wv,
                        storage,
                    );
                    reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
                }
            }
        }

        "close_tabs_to_right" => {
            if let Some(payload) = &msg.payload {
                if let Some(idx) = payload["index"].as_u64() {
                    tab_mgr.close_tabs_to_right(
                        idx as usize,
                        window,
                        proxy,
                        bounds,
                        adblocker,
                        webview_data_dir,
                        chrome_wv,
                        storage,
                    );
                    reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
                }
            }
        }

        "toggle_mute" => {
            if let Some(payload) = &msg.payload {
                if let Some(target_idx) = payload["index"].as_u64() {
                    tab_mgr.toggle_mute(target_idx as usize, chrome_wv);
                }
            }
        }

        "back" => {
            if let Some(tab) = tab_mgr.active_tab() {
                if let Some(wv) = &tab.webview {
                    let _ = wv.go_back();
                }
            }
        }

        "forward" => {
            if let Some(tab) = tab_mgr.active_tab() {
                if let Some(wv) = &tab.webview {
                    let _ = wv.go_forward();
                }
            }
        }

        "reload" => {
            if let Some(tab) = tab_mgr.active_tab() {
                if let Some(wv) = &tab.webview {
                    let _ = wv.evaluate_script("window.location.reload()");
                }
            }
        }

        "stop" => {
            if let Some(tab) = tab_mgr.active_tab_mut() {
                tab.is_loading = false;
                if let Some(wv) = &tab.webview {
                    let _ = wv.evaluate_script("window.stop();");
                }
                let _ = chrome_wv.evaluate_script("window.setLoading(false);");
            }
        }

        "home" => {
            tab_mgr.navigate(HOME_URL, window, proxy, bounds, adblocker, webview_data_dir, chrome_wv, storage);
            reassert_chrome_bounds(chrome_wv, chrome_expansion_mode, current_scale_factor, current_size);
        }

        "toggle_fullscreen" => {
            let is_fs = window.fullscreen().is_some();
            if is_fs {
                window.set_fullscreen(None);
            } else {
                window.set_fullscreen(Some(tao::window::Fullscreen::Borderless(None)));
            }
        }

        "launch_keyboard" => {
            launch_touch_keyboard();
        }

        "toggle_bookmark" => {
            if let Some(tab) = tab_mgr.active_tab() {
                if let Ok(is_bm) = storage.toggle_bookmark(&tab.url, &tab.title) {
                    let (can_back, can_fwd) = TabManager::tab_nav_state(tab);
                    let url_json = clean_js_json(&serde_json::to_string(&tab.url).unwrap_or_else(|_| "\"\"".to_string()));
                    let title_json = clean_js_json(&serde_json::to_string(&tab.title).unwrap_or_else(|_| "\"\"".to_string()));
                    let script = format!(
                        "window.updateAddress({}, {}, {}, {}, {}, {});",
                        url_json, title_json, can_back, can_fwd, is_bm, tab.blocked_count
                    );
                    let _ = chrome_wv.evaluate_script(&script);
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

        "delete_bookmark" => {
            if let Some(payload) = &msg.payload {
                if let Some(url) = payload["url"].as_str() {
                    let _ = storage.delete_bookmark(url);
                    let bookmarks = storage.get_bookmarks();
                    if let Ok(json) = serde_json::to_string(&bookmarks) {
                        let script = format!("window.showDrawerData({}, 'bookmarks');", clean_js_json(&json));
                        let _ = chrome_wv.evaluate_script(&script);
                    }
                    if let Some(tab) = tab_mgr.active_tab() {
                        if tab.url == url {
                            tab_mgr.sync_address_to_chrome(chrome_wv, storage);
                        }
                    }
                }
            }
        }

        "get_history" => {
            let history = storage.get_recent_history(50);
            if let Ok(json) = serde_json::to_string(&history) {
                let script = format!("window.showDrawerData({}, 'history');", clean_js_json(&json));
                let _ = chrome_wv.evaluate_script(&script);
            }
        }

        "delete_history_entry" => {
            if let Some(payload) = &msg.payload {
                if let Some(url) = payload["url"].as_str() {
                    let _ = storage.delete_history_item(url);
                    let history = storage.get_recent_history(50);
                    if let Ok(json) = serde_json::to_string(&history) {
                        let script = format!("window.showDrawerData({}, 'history');", clean_js_json(&json));
                        let _ = chrome_wv.evaluate_script(&script);
                    }
                }
            }
        }

        "clear_history" => {
            let _ = storage.clear_history();
            let script = "window.showDrawerData([], 'history');";
            let _ = chrome_wv.evaluate_script(script);
        }

        "search_query" => {
            if let Some(payload) = &msg.payload {
                if let Some(query) = payload["query"].as_str() {
                    let results = storage.search(query, 6);
                    if let Ok(json) = serde_json::to_string(&results) {
                        let q_json = clean_js_json(&serde_json::to_string(query).unwrap_or_else(|_| "\"\"".to_string()));
                        let script = format!("window.showSearchResults({}, {});", clean_js_json(&json), q_json);
                        let _ = chrome_wv.evaluate_script(&script);
                    }
                }
            }
        }

        "toggle_shield" => {
            adblocker.enabled = !adblocker.enabled;
            let shield_script = AdBlocker::get_shield_toggle_script(adblocker.enabled);
            for tab in &tab_mgr.tabs {
                if let Some(wv) = &tab.webview {
                    let _ = wv.evaluate_script(shield_script);
                }
            }
            let script = format!("window.updateShieldState({});", adblocker.enabled);
            let _ = chrome_wv.evaluate_script(&script);
        }

        "toggle_short_form" => {
            adblocker.block_short_form = !adblocker.block_short_form;
            let _ = storage.set_setting_bool("block_short_form", adblocker.block_short_form);
            let script = AdBlocker::get_short_form_toggle_script(adblocker.block_short_form);
            for tab in &tab_mgr.tabs {
                if let Some(wv) = &tab.webview {
                    let _ = wv.evaluate_script(script);
                }
            }
            let chrome_script = format!("window.updateShortFormState({});", adblocker.block_short_form);
            let _ = chrome_wv.evaluate_script(&chrome_script);
        }

        "toggle_reader" => {
            if let Some(tab) = tab_mgr.active_tab() {
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
            for tab in &tab_mgr.tabs {
                if let Some(wv) = &tab.webview {
                    let _ = wv.evaluate_script(script);
                }
            }
        }

        "find_in_page" => {
            if let Some(payload) = &msg.payload {
                let query = payload["query"].as_str().unwrap_or("");
                let backward = payload["backward"].as_bool().unwrap_or(false);
                tab_mgr.find_in_page(query, backward);
            }
        }

        "zoom_in" => {
            tab_mgr.zoom_in();
        }

        "zoom_out" => {
            tab_mgr.zoom_out();
        }

        "zoom_reset" => {
            tab_mgr.zoom_reset();
        }

        "set_chrome_expanded" => {
            if let Some(payload) = &msg.payload {
                let mode = payload["mode"].as_str().unwrap_or("none");
                *chrome_expansion_mode = mode.to_string();
                let h = chrome_expanded_height(mode, current_scale_factor, current_size.height);
                let _ = chrome_wv.set_bounds(Rect {
                    position: PhysicalPosition::new(0, 0).into(),
                    size: PhysicalSize::new(current_size.width, h).into(),
                });
                if mode == "full" {
                    if let Some(tab) = tab_mgr.active_tab() {
                        if let Some(wv) = &tab.webview {
                            let _ = wv.set_visible(false);
                        }
                    }
                } else {
                    if let Some(tab) = tab_mgr.active_tab() {
                        if let Some(wv) = &tab.webview {
                            let _ = wv.set_visible(true);
                        }
                    }
                }
            }
        }

        "hibernate_all" => {
            tab_mgr.hibernate_all_background(chrome_wv);
            let free_mb = MemoryGovernor::get_available_mb();
            let load_pct = MemoryGovernor::get_memory_load_pct();
            let engine = storage.get_setting_string("search_engine", "duckduckgo");
            let clear = storage.get_setting_bool("clear_on_exit", false);
            let script = format!(
                "window.updateSettingsData('{}', {}, {}, {});",
                engine, clear, free_mb, load_pct
            );
            let _ = chrome_wv.evaluate_script(&script);
        }

        "print" => {
            if let Some(tab) = tab_mgr.active_tab() {
                if let Some(wv) = &tab.webview {
                    let _ = wv.evaluate_script("window.print();");
                }
            }
        }

        "set_search_engine" => {
            if let Some(payload) = &msg.payload {
                if let Some(engine) = payload["engine"].as_str() {
                    let _ = storage.set_setting_string("search_engine", engine);
                }
            }
        }

        "set_clear_on_exit" => {
            if let Some(payload) = &msg.payload {
                if let Some(enabled) = payload["enabled"].as_bool() {
                    let _ = storage.set_setting_bool("clear_on_exit", enabled);
                }
            }
        }

        "get_settings" => {
            let engine = storage.get_setting_string("search_engine", "duckduckgo");
            let clear = storage.get_setting_bool("clear_on_exit", false);
            let free_mb = MemoryGovernor::get_available_mb();
            let load_pct = MemoryGovernor::get_memory_load_pct();
            let script = format!(
                "window.updateSettingsData('{}', {}, {}, {});",
                engine, clear, free_mb, load_pct
            );
            let _ = chrome_wv.evaluate_script(&script);
        }

        _ => ()
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Aerolith Browser")
        .with_inner_size(PhysicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT))
        .build(&event_loop)?;

    // Database, settings and adblocker setup
    let db_path = dirs_storage_path();
    let storage = BrowserStorage::new(&db_path)?;
    let mut adblocker = AdBlocker::new();
    adblocker.block_short_form = storage.get_setting_bool("block_short_form", true);

    let mut current_scale_factor = window.scale_factor();
    let mut current_size = window.inner_size();
    let initial_chrome_h = physical_chrome_height(current_scale_factor);
    let initial_chrome_rect = Rect {
        position: PhysicalPosition::new(0, 0).into(),
        size: PhysicalSize::new(current_size.width, initial_chrome_h).into(),
    };

    // 1. Build Chrome UI WebView (Pinned to top: 0..physical_chrome_height)
    let (chrome_wv, webview_data_dir) = create_chrome_webview(
        &window,
        &proxy,
        initial_chrome_rect,
        dirs_webview_data_path(),
    )?;

    // 2. Initialize TabManager and restore session
    let mut tab_mgr = TabManager::new();
    let governor = MemoryGovernor::new();
    let mut chrome_expansion_mode = "none".to_string();

    let session = storage.load_session();
    let (initial_urls, initial_active_idx) = match session {
        Some(s) if !s.tabs.is_empty() => (s.tabs, s.active_index),
        _ => (vec![HOME_URL.to_string()], 0),
    };

    let c_rect = content_rect(current_size, current_scale_factor);
    for u in &initial_urls {
        let t_id = tab_mgr.next_id;
        tab_mgr.next_id += 1;
        let effective_url = if adblocker.block_short_form && AdBlocker::is_short_form_url(u) {
            AdBlocker::sanitize_short_form_url(u).unwrap_or_else(|| u.clone())
        } else {
            u.clone()
        };
        if let Ok(t) = create_tab(t_id, &effective_url, &window, &proxy, c_rect, &adblocker, &webview_data_dir) {
            tab_mgr.tabs.push(t);
        }
    }
    if tab_mgr.is_empty() {
        let initial_tab = create_tab(
            tab_mgr.next_id,
            HOME_URL,
            &window,
            &proxy,
            c_rect,
            &adblocker,
            &webview_data_dir,
        )?;
        tab_mgr.tabs.push(initial_tab);
        tab_mgr.next_id += 1;
    }

    tab_mgr.active_index = if initial_active_idx < tab_mgr.len() {
        initial_active_idx
    } else {
        tab_mgr.len() - 1
    };

    for (idx, tab) in tab_mgr.tabs.iter_mut().enumerate() {
        if let Some(wv) = &tab.webview {
            let _ = wv.set_bounds(content_rect(current_size, current_scale_factor));
            let _ = wv.set_visible(idx == tab_mgr.active_index);
        }
    }
    reassert_chrome_bounds(&chrome_wv, &chrome_expansion_mode, current_scale_factor, current_size);

    // Run Event Loop with 5-second Memory Governor wakeups
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(std::time::Instant::now() + std::time::Duration::from_secs(5));

        match event {
            Event::NewEvents(StartCause::Init) => {
                tab_mgr.sync_tabs_to_chrome(&chrome_wv);
                tab_mgr.sync_address_to_chrome(&chrome_wv, &storage);
            }

            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                tab_mgr.apply_governor_tick(governor.is_memory_pressure_high(), &chrome_wv);
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let clear_on_exit = storage.get_setting_bool("clear_on_exit", false);
                if clear_on_exit {
                    let _ = storage.clear_history();
                    let _ = storage.clear_session();
                } else {
                    tab_mgr.save_session(&storage);
                }
                *control_flow = ControlFlow::Exit;
            }

            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size },
                ..
            } => {
                current_scale_factor = scale_factor;
                current_size = *new_inner_size;
                reassert_chrome_bounds(&chrome_wv, &chrome_expansion_mode, current_scale_factor, current_size);
                if let Some(tab) = tab_mgr.active_tab_mut() {
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
                    reassert_chrome_bounds(&chrome_wv, &chrome_expansion_mode, current_scale_factor, current_size);
                    if let Some(tab) = tab_mgr.active_tab_mut() {
                        if let Some(wv) = &tab.webview {
                            let _ = wv.set_bounds(content_rect(new_size, current_scale_factor));
                        }
                    }
                }
            }

            Event::UserEvent(UserEvent::MediaStateChanged { tab_id, playing }) => {
                if let Some(tab) = tab_mgr.find_by_id_mut(tab_id) {
                    tab.is_audio_playing = playing;
                }
                tab_mgr.sync_tabs_to_chrome(&chrome_wv);
            }

            Event::UserEvent(UserEvent::FormDirtyChanged { tab_id, dirty }) => {
                if let Some(tab) = tab_mgr.find_by_id_mut(tab_id) {
                    tab.has_form_input = dirty;
                }
            }

            Event::UserEvent(UserEvent::ScrollChanged { tab_id, scroll_x, scroll_y }) => {
                if let Some(tab) = tab_mgr.find_by_id_mut(tab_id) {
                    tab.scroll_x = scroll_x;
                    tab.scroll_y = scroll_y;
                }
            }

            Event::UserEvent(UserEvent::LoadingState { tab_id, loading }) => {
                if let Some(tab) = tab_mgr.find_by_id_mut(tab_id) {
                    tab.is_loading = loading;
                }
                if let Some(active_tab) = tab_mgr.active_tab() {
                    if active_tab.id == tab_id {
                        let script = format!("window.setLoading({});", loading);
                        let _ = chrome_wv.evaluate_script(&script);
                    }
                }
            }

            Event::UserEvent(UserEvent::ReaderStateChanged { tab_id, active }) => {
                if let Some(tab) = tab_mgr.find_by_id_mut(tab_id) {
                    tab.is_reader_active = active;
                }
                if let Some(active_tab) = tab_mgr.active_tab() {
                    if active_tab.id == tab_id {
                        let script = format!("window.setReaderState({});", active);
                        let _ = chrome_wv.evaluate_script(&script);
                    }
                }
            }

            Event::UserEvent(UserEvent::TitleChanged { tab_id, title }) => {
                let sanitized_title = sanitize_title(&title);
                if let Some(tab) = tab_mgr.find_by_id_mut(tab_id) {
                    if !sanitized_title.is_empty() {
                        tab.title = sanitized_title.clone();
                    }
                    tab.last_active = std::time::Instant::now();
                }
                if let Some(active_tab) = tab_mgr.active_tab() {
                    if active_tab.id == tab_id {
                        tab_mgr.sync_address_to_chrome(&chrome_wv, &storage);
                        let _ = storage.record_visit(&active_tab.url, &sanitized_title);
                    }
                }
                tab_mgr.sync_tabs_to_chrome(&chrome_wv);
            }

            Event::UserEvent(UserEvent::UrlChanged { tab_id, url }) => {
                let was_sanitized = adblocker.block_short_form && AdBlocker::is_short_form_url(&url);
                let effective_url = if was_sanitized {
                    AdBlocker::sanitize_short_form_url(&url).unwrap_or(url.clone())
                } else {
                    url.clone()
                };

                let is_about_home = if let Some(tab) = tab_mgr.tabs.iter().find(|t| t.id == tab_id) {
                    tab.url == HOME_URL && (effective_url == "about:blank" || effective_url == HOME_URL)
                } else {
                    false
                };

                let needs_redirect = !is_about_home && was_sanitized && effective_url != url && !effective_url.is_empty();

                if let Some(tab) = tab_mgr.find_by_id_mut(tab_id) {
                    if !effective_url.is_empty() && !is_about_home {
                        tab.url = effective_url.clone();
                    }
                    tab.last_active = std::time::Instant::now();
                    tab.is_audio_playing = false;
                    tab.has_form_input = false;
                    tab.is_reader_active = false;
                    if let Some(wv) = &tab.webview {
                        if needs_redirect {
                            let _ = wv.load_url(&effective_url);
                        }
                        if adblocker.dark_mode {
                            let _ = wv.evaluate_script(AdBlocker::get_dark_mode_script(true));
                        }
                        if adblocker.enabled {
                            let _ = wv.evaluate_script(AdBlocker::get_shield_toggle_script(true));
                        }
                        if adblocker.block_short_form {
                            let _ = wv.evaluate_script(AdBlocker::get_short_form_toggle_script(true));
                        }
                        if tab.scroll_x > 0 || tab.scroll_y > 0 {
                            let _ = wv.evaluate_script(&AdBlocker::get_tab_restore_script(tab.scroll_x, tab.scroll_y));
                        }
                    }
                }
                if let Some(active_tab) = tab_mgr.active_tab() {
                    if active_tab.id == tab_id {
                        tab_mgr.sync_address_to_chrome(&chrome_wv, &storage);
                        let _ = chrome_wv.evaluate_script("window.setReaderState(false);");
                        if !is_about_home {
                            let _ = storage.record_visit(&effective_url, &active_tab.title);
                        }
                        tab_mgr.save_session(&storage);
                    }
                }
                tab_mgr.sync_tabs_to_chrome(&chrome_wv);
            }

            Event::UserEvent(UserEvent::NewWindow(target_url)) => {
                let bounds = content_rect(current_size, current_scale_factor);
                tab_mgr.new_tab(
                    &target_url,
                    &window,
                    &proxy,
                    bounds,
                    &adblocker,
                    &webview_data_dir,
                    &chrome_wv,
                    &storage,
                );
                reassert_chrome_bounds(&chrome_wv, &chrome_expansion_mode, current_scale_factor, current_size);
            }

            Event::UserEvent(UserEvent::Chrome(msg)) => {
                handle_chrome_msg(
                    msg,
                    &mut tab_mgr,
                    &chrome_wv,
                    &window,
                    &proxy,
                    &mut adblocker,
                    &storage,
                    &webview_data_dir,
                    &mut chrome_expansion_mode,
                    current_scale_factor,
                    current_size,
                );
            }

            Event::UserEvent(UserEvent::AdBlocked { tab_id }) => {
                if let Some(tab) = tab_mgr.find_by_id_mut(tab_id) {
                    tab.blocked_count += 1;
                }
                if let Some(active_tab) = tab_mgr.active_tab() {
                    if active_tab.id == tab_id {
                        tab_mgr.sync_address_to_chrome(&chrome_wv, &storage);
                    }
                }
                tab_mgr.sync_tabs_to_chrome(&chrome_wv);
            }

            _ => ()
        }
    });
}

fn main() {
    if let Err(e) = run() {
        show_fatal_error(&format!("Aerolith Browser failed to start:\n\n{}", e));
    }
}
