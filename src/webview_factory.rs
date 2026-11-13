use crate::adblock::AdBlocker;
use crate::paths::dirs_webview_fallback_data_path;
use crate::types::{ChromeMsg, UserEvent};
use crate::ui::get_chrome_html;
use std::path::PathBuf;
use tao::event_loop::EventLoopProxy;
use tao::window::Window;
use wry::{Rect, WebContext, WebView, WebViewBuilder};

pub fn create_chrome_webview(
    window: &Window,
    proxy: &EventLoopProxy<UserEvent>,
    bounds: Rect,
    primary_data_dir: PathBuf,
) -> Result<(WebView, PathBuf), Box<dyn std::error::Error>> {
    let proxy_chrome = proxy.clone();
    let mut chrome_context = WebContext::new(Some(primary_data_dir.clone()));
    let (chrome_wv, effective_dir) = match WebViewBuilder::new_with_web_context(&mut chrome_context)
        .with_transparent(true)
        .with_focused(true)
        .with_hotkeys_zoom(false)
        .with_bounds(bounds)
        .with_html(get_chrome_html())
        .with_ipc_handler(move |req| {
            if let Ok(msg) = serde_json::from_str::<ChromeMsg>(req.body()) {
                let _ = proxy_chrome.send_event(UserEvent::Chrome(msg));
            }
        })
        .build_as_child(window)
    {
        Ok(wv) => (wv, primary_data_dir),
        Err(_) => {
            // Failover to instance-specific directory if primary WebView2 folder is locked
            let fallback_dir = dirs_webview_fallback_data_path();
            let mut fallback_context = WebContext::new(Some(fallback_dir.clone()));
            let proxy_chrome_fb = proxy.clone();
            let wv = WebViewBuilder::new_with_web_context(&mut fallback_context)
                .with_transparent(true)
                .with_focused(true)
                .with_hotkeys_zoom(false)
                .with_bounds(bounds)
                .with_html(get_chrome_html())
                .with_ipc_handler(move |req| {
                    if let Ok(msg) = serde_json::from_str::<ChromeMsg>(req.body()) {
                        let _ = proxy_chrome_fb.send_event(UserEvent::Chrome(msg));
                    }
                })
                .build_as_child(window)?;
            (wv, fallback_dir)
        }
    };

    let _ = chrome_wv.set_bounds(bounds);
    let _ = chrome_wv.set_visible(true);

    Ok((chrome_wv, effective_dir))
}

pub fn create_content_webview(
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
            function __injectShield() {{
                if (document.getElementById('aerolith-adblock-shield')) return;
                const root = document.head || document.documentElement || document.body;
                if (root) {{
                    const style = document.createElement('style');
                    style.id = 'aerolith-adblock-shield';
                    style.textContent = `{}`;
                    root.appendChild(style);
                }} else {{
                    document.addEventListener('DOMContentLoaded', __injectShield, {{ once: true }});
                }}
            }}
            __injectShield();
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
    let proxy_nav = proxy_ref.clone();

    let mut builder = WebViewBuilder::new_with_web_context(&mut tab_context)
        .with_bounds(bounds)
        .with_initialization_script(&init_script)
        .with_back_forward_navigation_gestures(true)
        .with_hotkeys_zoom(true)
        .with_devtools(cfg!(debug_assertions));

    if url == "about:home" {
        builder = builder.with_html(crate::ui::get_speed_dial_html());
    } else {
        builder = builder.with_url(url);
    }

    let wv = builder
        .with_ipc_handler(move |req| {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(req.body()) {
                let msg_type = val["type"].as_str().or_else(|| val["action"].as_str());
                if let Some(msg_type) = msg_type {
                    match msg_type {
                        "ad_blocked" => {
                            let _ = proxy_content_ipc.send_event(UserEvent::AdBlocked { tab_id });
                        }
                        "home_navigate" => {
                            let target = val["url"]
                                .as_str()
                                .or_else(|| val["query"].as_str())
                                .or_else(|| val["payload"]["url"].as_str())
                                .or_else(|| val["payload"]["query"].as_str());
                            if let Some(target) = target {
                                let _ = proxy_content_ipc.send_event(UserEvent::Chrome(ChromeMsg {
                                    action: "navigate".into(),
                                    payload: Some(serde_json::json!({ "url": target })),
                                }));
                            }
                        }
                        "hibernate_all" => {
                            let _ = proxy_content_ipc.send_event(UserEvent::Chrome(ChromeMsg {
                                action: "hibernate_all".into(),
                                payload: None,
                            }));
                        }
                        "spa_navigation" => {
                            let nav_url = val["url"].as_str().unwrap_or("").to_string();
                            if !nav_url.is_empty() {
                                let _ = proxy_content_ipc.send_event(UserEvent::UrlChanged {
                                    tab_id,
                                    url: nav_url,
                                });
                            }
                            if let Some(title) = val["title"].as_str() {
                                if !title.is_empty() {
                                    let _ = proxy_content_ipc.send_event(UserEvent::TitleChanged {
                                        tab_id,
                                        title: title.to_string(),
                                    });
                                }
                            }
                        }
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
                        "reader_closed" => {
                            let _ = proxy_content_ipc.send_event(UserEvent::ReaderStateChanged {
                                tab_id,
                                active: false,
                            });
                        }
                        "reader_opened" => {
                            let _ = proxy_content_ipc.send_event(UserEvent::ReaderStateChanged {
                                tab_id,
                                active: true,
                            });
                        }
                        _ => {}
                    }
                }
            }
        })
        .with_navigation_handler(move |nav_url| {
            let lower = nav_url.trim().to_lowercase();
            let allowed = lower.starts_with("http://")
                || lower.starts_with("https://")
                || lower.starts_with("file://")
                || lower.starts_with("about:blank")
                || lower.starts_with("about:home");
            if AdBlocker::is_tracker_or_ad_domain(&lower) {
                let _ = proxy_nav.send_event(UserEvent::AdBlocked { tab_id });
                false
            } else {
                allowed
            }
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
        .build_as_child(win)?;

    if url == "about:home" {
        load_speed_dial(&wv);
    }

    let _ = wv.set_bounds(bounds);
    Ok(wv)
}

pub fn load_speed_dial(wv: &WebView) {
    let html = crate::ui::get_speed_dial_html();
    let json = crate::url_utils::clean_js_json(&serde_json::to_string(html).unwrap_or_else(|_| "\"\"".to_string()));
    let script = format!("document.open(); document.write({}); document.close();", json);
    let _ = wv.evaluate_script(&script);
}
