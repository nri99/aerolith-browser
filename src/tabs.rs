use crate::adblock::AdBlocker;
use crate::storage::BrowserStorage;
use crate::types::{TabInfo, TabState, UserEvent, HOME_URL, MAX_TABS};
use crate::url_utils::{clean_js_json, normalize_url};
use crate::webview_factory::create_content_webview;
use std::path::PathBuf;
use tao::event_loop::EventLoopProxy;
use tao::window::Window;
use wry::{Rect, WebView};

pub struct Tab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub scroll_x: u32,
    pub scroll_y: u32,
    pub last_active: std::time::Instant,
    pub state: TabState,
    pub webview: Option<WebView>,
    pub is_audio_playing: bool,
    pub has_form_input: bool,
    pub is_loading: bool,
    pub is_reader_active: bool,
    pub is_muted: bool,
}

pub fn create_tab(
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
        is_reader_active: false,
        is_muted: false,
    })
}

pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub closed_tabs: Vec<String>,
    pub active_index: usize,
    pub next_id: usize,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            closed_tabs: Vec::new(),
            active_index: 0,
            next_id: 1,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    #[inline]
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_index)
    }

    #[inline]
    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_index)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get(&self, index: usize) -> Option<&Tab> {
        self.tabs.get(index)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Tab> {
        self.tabs.get_mut(index)
    }

    pub fn find_by_id_mut(&mut self, id: usize) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|t| t.id == id)
    }

    #[allow(dead_code)]
    pub fn active_nav_state(&self) -> (bool, bool) {
        if let Some(tab) = self.active_tab() {
            if let Some(wv) = &tab.webview {
                (wv.can_go_back().unwrap_or(false), wv.can_go_forward().unwrap_or(false))
            } else {
                (false, false)
            }
        } else {
            (false, false)
        }
    }

    pub fn tab_nav_state(tab: &Tab) -> (bool, bool) {
        if let Some(wv) = &tab.webview {
            (wv.can_go_back().unwrap_or(false), wv.can_go_forward().unwrap_or(false))
        } else {
            (false, false)
        }
    }

    pub fn sync_tabs_to_chrome(&self, chrome: &WebView) {
        let info_list: Vec<TabInfo> = self
            .tabs
            .iter()
            .map(|t| TabInfo {
                id: t.id,
                title: t.title.clone(),
                url: t.url.clone(),
                is_sleeping: t.state != TabState::Active,
                is_audio_playing: t.is_audio_playing,
                is_muted: t.is_muted,
            })
            .collect();
        if let Ok(json) = serde_json::to_string(&info_list) {
            let script = format!("window.updateTabs({}, {});", clean_js_json(&json), self.active_index);
            let _ = chrome.evaluate_script(&script);
        }
    }

    pub fn sync_address_to_chrome(&self, chrome: &WebView, storage: &BrowserStorage) {
        if let Some(cur) = self.active_tab() {
            let url_json = clean_js_json(&serde_json::to_string(&cur.url).unwrap_or_else(|_| "\"\"".to_string()));
            let title_json = clean_js_json(&serde_json::to_string(&cur.title).unwrap_or_else(|_| "\"\"".to_string()));
            let is_bm = storage.is_bookmarked(&cur.url);
            let (can_back, can_fwd) = Self::tab_nav_state(cur);
            let script = format!(
                "window.updateAddress({}, {}, {}, {}, {});",
                url_json, title_json, can_back, can_fwd, is_bm
            );
            let _ = chrome.evaluate_script(&script);
        }
    }

    pub fn save_session(&self, storage: &BrowserStorage) {
        let urls: Vec<String> = self.tabs.iter().map(|t| t.url.clone()).collect();
        let _ = storage.save_session(&urls, self.active_index);
    }

    pub fn new_tab(
        &mut self,
        target_url: &str,
        win: &Window,
        proxy: &EventLoopProxy<UserEvent>,
        bounds: Rect,
        adblock: &AdBlocker,
        data_dir: &PathBuf,
        chrome_wv: &WebView,
        storage: &BrowserStorage,
    ) {
        if self.tabs.len() >= MAX_TABS {
            // Cap resources: reuse current tab
            self.navigate(target_url, win, proxy, bounds, adblock, data_dir, chrome_wv, storage);
            return;
        }

        let new_id = self.next_id;
        self.next_id += 1;

        if let Ok(new_tab) = create_tab(new_id, target_url, win, proxy, bounds, adblock, data_dir) {
            if let Some(wv) = &new_tab.webview {
                let _ = wv.set_bounds(bounds);
                let _ = wv.set_visible(true);
            }
            if let Some(cur) = self.tabs.get_mut(self.active_index) {
                if let Some(wv) = &cur.webview {
                    let _ = wv.set_visible(false);
                }
                cur.last_active = std::time::Instant::now();
            }
            self.tabs.push(new_tab);
            self.active_index = self.tabs.len() - 1;

            self.sync_address_to_chrome(chrome_wv, storage);
            let _ = chrome_wv.evaluate_script("window.setLoading(false);");
            let _ = chrome_wv.evaluate_script("window.setReaderState(false);");
            self.sync_tabs_to_chrome(chrome_wv);
            self.save_session(storage);
        }
    }

    pub fn switch_tab(
        &mut self,
        target_idx: usize,
        win: &Window,
        proxy: &EventLoopProxy<UserEvent>,
        bounds: Rect,
        adblock: &AdBlocker,
        data_dir: &PathBuf,
        chrome_wv: &WebView,
        storage: &BrowserStorage,
    ) {
        if target_idx < self.tabs.len() && target_idx != self.active_index {
            if let Some(old_tab) = self.tabs.get_mut(self.active_index) {
                if let Some(wv) = &old_tab.webview {
                    let _ = wv.set_visible(false);
                }
                old_tab.last_active = std::time::Instant::now();
            }
            self.active_index = target_idx;
            if let Some(new_tab) = self.tabs.get_mut(self.active_index) {
                new_tab.last_active = std::time::Instant::now();
                if new_tab.webview.is_none() {
                    // Wake tab from Tier 2 Deep Hibernation
                    if let Ok(wv) = create_content_webview(new_tab.id, &new_tab.url, win, proxy, bounds, adblock, data_dir) {
                        let _ = wv.set_bounds(bounds);
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
                    let _ = wv.set_bounds(bounds);
                    let _ = wv.set_visible(true);
                }
                new_tab.state = TabState::Active;
                let _ = chrome_wv.evaluate_script(&format!("window.setLoading({});", new_tab.is_loading));
                let _ = chrome_wv.evaluate_script(&format!("window.setReaderState({});", new_tab.is_reader_active));
            }
            self.sync_address_to_chrome(chrome_wv, storage);
            self.sync_tabs_to_chrome(chrome_wv);
            self.save_session(storage);
        }
    }

    pub fn close_tab(
        &mut self,
        close_idx: usize,
        win: &Window,
        proxy: &EventLoopProxy<UserEvent>,
        bounds: Rect,
        adblock: &AdBlocker,
        data_dir: &PathBuf,
        chrome_wv: &WebView,
        storage: &BrowserStorage,
    ) {
        if self.tabs.len() > 1 && close_idx < self.tabs.len() {
            let was_active = close_idx == self.active_index;
            let removed_tab = self.tabs.remove(close_idx);
            if !removed_tab.url.is_empty() && removed_tab.url != HOME_URL && removed_tab.url != "about:blank" {
                self.closed_tabs.push(removed_tab.url);
                if self.closed_tabs.len() > 30 {
                    self.closed_tabs.remove(0);
                }
            }
            if let Some(wv) = &removed_tab.webview {
                let _ = wv.set_visible(false);
            }

            if was_active {
                if self.active_index >= self.tabs.len() {
                    self.active_index = self.tabs.len() - 1;
                }
            } else if close_idx < self.active_index {
                self.active_index -= 1;
            }

            if let Some(new_active) = self.tabs.get_mut(self.active_index) {
                new_active.last_active = std::time::Instant::now();
                if new_active.webview.is_none() {
                    if let Ok(wv) = create_content_webview(new_active.id, &new_active.url, win, proxy, bounds, adblock, data_dir) {
                        let _ = wv.set_bounds(bounds);
                        let _ = wv.set_visible(true);
                        if new_active.scroll_x > 0 || new_active.scroll_y > 0 {
                            let restore_script = AdBlocker::get_tab_restore_script(new_active.scroll_x, new_active.scroll_y);
                            let _ = wv.evaluate_script(&restore_script);
                        }
                        new_active.webview = Some(wv);
                    }
                } else if let Some(wv) = &new_active.webview {
                    let _ = wv.set_bounds(bounds);
                    let _ = wv.set_visible(true);
                }
                new_active.state = TabState::Active;
                let _ = chrome_wv.evaluate_script(&format!("window.setLoading({});", new_active.is_loading));
                let _ = chrome_wv.evaluate_script(&format!("window.setReaderState({});", new_active.is_reader_active));
            }
            self.sync_address_to_chrome(chrome_wv, storage);
            self.sync_tabs_to_chrome(chrome_wv);
            self.save_session(storage);
        } else if self.tabs.len() == 1 && close_idx == 0 {
            if let Some(tab) = self.tabs.get_mut(0) {
                if !tab.url.is_empty() && tab.url != HOME_URL && tab.url != "about:blank" {
                    self.closed_tabs.push(tab.url.clone());
                    if self.closed_tabs.len() > 30 {
                        self.closed_tabs.remove(0);
                    }
                }
                tab.url = HOME_URL.to_string();
                tab.title = "New Tab".to_string();
                tab.last_active = std::time::Instant::now();
                tab.state = TabState::Active;
                tab.is_loading = false;
                tab.is_reader_active = false;
                if let Some(wv) = &tab.webview {
                    let _ = wv.load_url(HOME_URL);
                }
                self.sync_address_to_chrome(chrome_wv, storage);
                self.sync_tabs_to_chrome(chrome_wv);
                self.save_session(storage);
            }
        }
    }

    pub fn reopen_closed_tab(
        &mut self,
        win: &Window,
        proxy: &EventLoopProxy<UserEvent>,
        bounds: Rect,
        adblock: &AdBlocker,
        data_dir: &PathBuf,
        chrome_wv: &WebView,
        storage: &BrowserStorage,
    ) {
        if let Some(target_url) = self.closed_tabs.pop() {
            if self.tabs.len() < MAX_TABS {
                let new_id = self.next_id;
                self.next_id += 1;
                if let Ok(new_tab) = create_tab(new_id, &target_url, win, proxy, bounds, adblock, data_dir) {
                    if let Some(wv) = &new_tab.webview {
                        let _ = wv.set_bounds(bounds);
                        let _ = wv.set_visible(true);
                    }
                    if let Some(cur) = self.tabs.get_mut(self.active_index) {
                        if let Some(wv) = &cur.webview {
                            let _ = wv.set_visible(false);
                        }
                        cur.last_active = std::time::Instant::now();
                    }
                    self.tabs.push(new_tab);
                    self.active_index = self.tabs.len() - 1;
                    self.sync_address_to_chrome(chrome_wv, storage);
                    let _ = chrome_wv.evaluate_script("window.setLoading(false);");
                    let _ = chrome_wv.evaluate_script("window.setReaderState(false);");
                    self.sync_tabs_to_chrome(chrome_wv);
                    self.save_session(storage);
                }
            } else if let Some(cur) = self.tabs.get_mut(self.active_index) {
                cur.url = target_url.clone();
                cur.last_active = std::time::Instant::now();
                cur.state = TabState::Active;
                if let Some(wv) = &cur.webview {
                    let _ = wv.set_bounds(bounds);
                    let _ = wv.set_visible(true);
                    let _ = wv.load_url(&target_url);
                }
                self.sync_address_to_chrome(chrome_wv, storage);
                self.sync_tabs_to_chrome(chrome_wv);
                self.save_session(storage);
            }
        }
    }

    pub fn navigate(
        &mut self,
        target_url: &str,
        win: &Window,
        proxy: &EventLoopProxy<UserEvent>,
        bounds: Rect,
        adblock: &AdBlocker,
        data_dir: &PathBuf,
        chrome_wv: &WebView,
        storage: &BrowserStorage,
    ) {
        let mut final_url = normalize_url(target_url);
        if adblock.block_short_form && AdBlocker::is_short_form_url(&final_url) {
            if let Some(sanitized) = AdBlocker::sanitize_short_form_url(&final_url) {
                final_url = sanitized;
            }
        }

        if let Some(tab) = self.tabs.get_mut(self.active_index) {
            tab.url = final_url.clone();
            tab.last_active = std::time::Instant::now();
            tab.state = TabState::Active;

            if tab.webview.is_none() {
                if let Ok(wv) = create_content_webview(tab.id, &final_url, win, proxy, bounds, adblock, data_dir) {
                    let _ = wv.set_bounds(bounds);
                    let _ = wv.set_visible(true);
                    tab.webview = Some(wv);
                }
            } else if let Some(wv) = &tab.webview {
                let _ = wv.load_url(&final_url);
            }

            self.sync_address_to_chrome(chrome_wv, storage);
            self.sync_tabs_to_chrome(chrome_wv);
            self.save_session(storage);
        }
    }

    pub fn toggle_mute(&mut self, tab_idx: usize, chrome_wv: &WebView) {
        if let Some(tab) = self.tabs.get_mut(tab_idx) {
            tab.is_muted = !tab.is_muted;
            let mute_val = tab.is_muted;
            if let Some(wv) = &tab.webview {
                let mute_script = format!(
                    r#"
                    (function() {{
                        try {{
                            document.querySelectorAll('audio, video').forEach(el => {{
                                el.muted = {};
                            }});
                        }} catch(e) {{}}
                    }})();
                    "#,
                    mute_val
                );
                let _ = wv.evaluate_script(&mute_script);
            }
            self.sync_tabs_to_chrome(chrome_wv);
        }
    }

    pub fn zoom_in(&self) {
        if let Some(tab) = self.active_tab() {
            if let Some(wv) = &tab.webview {
                let _ = wv.evaluate_script(r#"
                    (function() {
                        const cur = parseFloat(document.body.style.zoom || '1.0');
                        if (cur < 3.0) {
                            document.body.style.zoom = (cur + 0.1).toFixed(2);
                        }
                    })();
                "#);
            }
        }
    }

    pub fn zoom_out(&self) {
        if let Some(tab) = self.active_tab() {
            if let Some(wv) = &tab.webview {
                let _ = wv.evaluate_script(r#"
                    (function() {
                        const cur = parseFloat(document.body.style.zoom || '1.0');
                        if (cur > 0.4) {
                            document.body.style.zoom = (cur - 0.1).toFixed(2);
                        }
                    })();
                "#);
            }
        }
    }

    pub fn zoom_reset(&self) {
        if let Some(tab) = self.active_tab() {
            if let Some(wv) = &tab.webview {
                let _ = wv.evaluate_script(r#"
                    (function() {
                        document.body.style.zoom = '1.0';
                    })();
                "#);
            }
        }
    }

    pub fn find_in_page(&self, query: &str, backward: bool) {
        if let Some(tab) = self.active_tab() {
            if let Some(wv) = &tab.webview {
                let clean_q = clean_js_json(&serde_json::to_string(query).unwrap_or_default());
                let find_script = format!(
                    r#"
                    (function() {{
                        try {{
                            const q = {};
                            if (q) {{
                                window.find(q, false, {}, true, false, false, false);
                            }}
                        }} catch(e) {{}}
                    }})();
                    "#,
                    clean_q, backward
                );
                let _ = wv.evaluate_script(&find_script);
            }
        }
    }

    pub fn apply_governor_tick(&mut self, is_low_mem: bool, chrome_wv: &WebView) {
        let mut state_changed = false;
        const LOW_MEM_MIN_BG_SECS: u64 = 45;
        const TIER2_HIBERNATION_SECS: u64 = 600;
        const TIER1_THROTTLE_SECS: u64 = 120;

        for (idx, tab) in self.tabs.iter_mut().enumerate() {
            if idx == self.active_index {
                continue;
            }
            if tab.is_audio_playing || tab.has_form_input {
                continue;
            }

            let elapsed = tab.last_active.elapsed();
            let should_hibernate = (is_low_mem && elapsed >= std::time::Duration::from_secs(LOW_MEM_MIN_BG_SECS))
                || elapsed >= std::time::Duration::from_secs(TIER2_HIBERNATION_SECS);

            if should_hibernate {
                if tab.state != TabState::Tier2Hibernated && tab.webview.is_some() {
                    tab.webview = None;
                    tab.state = TabState::Tier2Hibernated;
                    state_changed = true;
                }
            } else if elapsed >= std::time::Duration::from_secs(TIER1_THROTTLE_SECS) {
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
            self.sync_tabs_to_chrome(chrome_wv);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            is_reader_active: false,
            is_muted: false,
        };
        assert!(tab.is_loading);
        tab.is_loading = false;
        assert!(!tab.is_loading);

        assert!(!tab.is_reader_active);
        tab.is_reader_active = true;
        assert!(tab.is_reader_active);

        assert!(!tab.is_muted);
        tab.is_muted = true;
        assert!(tab.is_muted);
    }

    #[test]
    fn test_tab_smart_sleep_exemptions_lifecycle() {
        let mut tab = Tab {
            id: 1,
            title: "Exemption Test".to_string(),
            url: "https://example.com".to_string(),
            scroll_x: 0,
            scroll_y: 0,
            last_active: std::time::Instant::now(),
            state: TabState::Active,
            webview: None,
            is_audio_playing: true,
            has_form_input: false,
            is_loading: false,
            is_reader_active: false,
            is_muted: false,
        };
        assert!(tab.is_audio_playing);
        assert!(!tab.has_form_input);

        tab.has_form_input = true;
        assert!(tab.has_form_input);
    }

    #[test]
    fn test_closed_tabs_stack_behavior() {
        let mut closed_tabs: Vec<String> = Vec::new();
        for i in 1..=35 {
            let url = format!("https://example.com/page{}", i);
            closed_tabs.push(url);
            if closed_tabs.len() > 30 {
                closed_tabs.remove(0);
            }
        }
        assert_eq!(closed_tabs.len(), 30);
        assert_eq!(closed_tabs.pop(), Some("https://example.com/page35".to_string()));
        assert_eq!(closed_tabs.first(), Some(&"https://example.com/page6".to_string()));
    }

    #[test]
    fn test_title_length_clamping() {
        let normal = "Normal Title - Aerolith";
        let clamped_normal: String = if normal.chars().count() > 256 {
            normal.chars().take(256).collect()
        } else {
            normal.to_string()
        };
        assert_eq!(clamped_normal, normal);

        let giant = "X".repeat(500);
        let clamped_giant: String = if giant.chars().count() > 256 {
            giant.chars().take(256).collect()
        } else {
            giant
        };
        assert_eq!(clamped_giant.len(), 256);
        assert_eq!(clamped_giant.chars().count(), 256);
    }

    #[test]
    fn test_short_form_url_redirection_pipeline() {
        let blocker = AdBlocker::new();
        assert!(blocker.block_short_form);

        let raw_yt = "https://www.youtube.com/shorts/dQw4w9WgXcQ?feature=share";
        let sanitized = AdBlocker::sanitize_short_form_url(raw_yt);
        assert_eq!(sanitized, Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ&feature=share".to_string()));

        let raw_ig = "https://www.instagram.com/reel/C3123456789/";
        let sanitized_ig = AdBlocker::sanitize_short_form_url(raw_ig);
        assert_eq!(sanitized_ig, Some("https://www.instagram.com/p/C3123456789/".to_string()));

        let normal_yt = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        assert_eq!(AdBlocker::sanitize_short_form_url(normal_yt), None);
    }
}
