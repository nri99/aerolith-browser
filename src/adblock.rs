const TRACKER_DOMAINS: &[&str] = &[
    "doubleclick.net",
    "googlesyndication.com",
    "adservice.google.com",
    "google-analytics.com",
    "googleanalytics.com",
    "scorecardresearch.com",
    "taboola.com",
    "outbrain.com",
    "criteo.com",
    "criteo.net",
    "adnxs.com",
    "rubiconproject.com",
    "pubmatic.com",
    "openx.net",
    "moatads.com",
    "amazon-adsystem.com",
    "quantserve.com",
    "zedo.com",
    "trafficfactory.biz",
    "serving-sys.com",
    "advertising.com",
    "bidswitch.net",
    "casalemedia.com",
    "yieldmo.com",
    "smartadserver.com",
    "popads.net",
    "popcash.net",
    "propellerads.com",
    "hotjar.com",
    "amplitude.com",
    "segment.io",
    "segment.com",
    "mixpanel.com",
    "clarity.ms",
    "mc.yandex.ru",
];

pub struct AdBlocker {
    pub enabled: bool,
    pub dark_mode: bool,
}

impl AdBlocker {
    pub fn new() -> Self {
        Self {
            enabled: true,
            dark_mode: false,
        }
    }

    /// Fast domain trie check for known advertising, tracking, and telemetry networks
    pub fn is_tracker_or_ad_domain(url_or_host: &str) -> bool {
        let trimmed = url_or_host.trim();
        let host = if let Some(scheme_end) = trimmed.find("://") {
            let after_scheme = &trimmed[scheme_end + 3..];
            let auth_len = after_scheme.find(|c| c == '/' || c == '?' || c == '#').unwrap_or(after_scheme.len());
            let authority = &after_scheme[..auth_len];
            let host_port = if let Some(at) = authority.rfind('@') {
                &authority[at + 1..]
            } else {
                authority
            };
            host_port.split(':').next().unwrap_or(host_port)
        } else {
            let auth_len = trimmed.find(|c| c == '/' || c == '?' || c == '#').unwrap_or(trimmed.len());
            let host_port = &trimmed[..auth_len];
            host_port.split(':').next().unwrap_or(host_port)
        };

        let host_lower = host.to_lowercase();
        TRACKER_DOMAINS.iter().any(|d| {
            host_lower == *d
                || (host_lower.ends_with(d)
                    && host_lower.len() > d.len()
                    && host_lower.as_bytes()[host_lower.len() - d.len() - 1] == b'.')
        })
    }

    /// Script to initialize dark mode on new tab creation if enabled
    pub fn get_dark_mode_init_script(&self) -> &'static str {
        if self.dark_mode {
            Self::get_dark_mode_script(true)
        } else {
            ""
        }
    }

    /// Returns cosmetic CSS rule injected into all pages to hide common ad frames and banners
    pub fn get_cosmetic_css(&self) -> &'static str {
        if !self.enabled {
            return "";
        }

        r#"
        /* Aerolith High-Performance Surface Go 2 Ad Shield */
        div[id*="google_ads"],
        div[class*="adsbygoogle"],
        ins.adsbygoogle,
        iframe[id*="google_ads_frame"],
        iframe[src*="doubleclick.net"],
        iframe[src*="googlesyndication.com"],
        div[class*="ad-container"],
        div[class*="ad-banner"],
        div[id*="ad-banner"],
        div[class*="advertisement"],
        div[aria-label="advertisement"],
        div[class*="sponsored-content"],
        div[class*="sponsored_post"],
        div[id*="taboola-"],
        div[id*="outbrain-"],
        div[class*="outbrain"],
        amp-ad,
        .ytp-ad-overlay-container,
        .ytp-ad-message-container {
            display: none !important;
            visibility: hidden !important;
            height: 0 !important;
            width: 0 !important;
            opacity: 0 !important;
            pointer-events: none !important;
        }
        "#
    }

    /// Script to dynamically toggle adblock shield on existing open tabs
    pub fn get_shield_toggle_script(enable: bool) -> &'static str {
        if enable {
            r#"
            (function() {
                let s = document.getElementById('aerolith-adblock-shield');
                if (!s) {
                    s = document.createElement('style');
                    s.id = 'aerolith-adblock-shield';
                    s.textContent = `
                        div[id*="google_ads"],
                        div[class*="adsbygoogle"],
                        ins.adsbygoogle,
                        iframe[id*="google_ads_frame"],
                        iframe[src*="doubleclick.net"],
                        iframe[src*="googlesyndication.com"],
                        div[class*="ad-container"],
                        div[class*="ad-banner"],
                        div[id*="ad-banner"],
                        div[class*="advertisement"],
                        div[aria-label="advertisement"],
                        div[class*="sponsored-content"],
                        div[class*="sponsored_post"],
                        div[id*="taboola-"],
                        div[id*="outbrain-"],
                        div[class*="outbrain"],
                        amp-ad,
                        .ytp-ad-overlay-container,
                        .ytp-ad-message-container {
                            display: none !important;
                            visibility: hidden !important;
                            height: 0 !important;
                            width: 0 !important;
                            opacity: 0 !important;
                            pointer-events: none !important;
                        }
                    `;
                    document.head && document.head.appendChild(s);
                }
            })();
            "#
        } else {
            r#"
            (function() {
                const s = document.getElementById('aerolith-adblock-shield');
                if (s) s.remove();
            })();
            "#
        }
    }

    /// Dual-Layer in-engine network interceptor: traps fetch, XHR, beacons, and monitors media/form state
    pub fn get_neutralizer_script(&self) -> &'static str {
        if !self.enabled {
            return "";
        }

        r#"
        (function() {
            // 1. Dual-Layer Network Request Interception: window.fetch
            if (window.fetch) {
                const originalFetch = window.fetch.bind(window);
                window.fetch = function(resource, init) {
                    let url = '';
                    if (typeof resource === 'string') {
                        url = resource;
                    } else if (resource && resource.url) {
                        url = resource.url;
                    }
                    const lower = (url || '').toLowerCase();
                    if (
                        lower.includes('doubleclick.net') ||
                        lower.includes('googlesyndication.com') ||
                        lower.includes('google-analytics.com') ||
                        lower.includes('scorecardresearch.com') ||
                        lower.includes('taboola.com') ||
                        lower.includes('outbrain.com') ||
                        lower.includes('criteo.com') ||
                        lower.includes('adnxs.com') ||
                        lower.includes('hotjar.com') ||
                        lower.includes('clarity.ms') ||
                        lower.includes('amplitude.com') ||
                        lower.includes('segment.io') ||
                        lower.includes('mixpanel.com')
                    ) {
                        // Synthetic 204 response drops connection without initiating TCP/TLS socket
                        return Promise.resolve(new Response(null, { status: 204, statusText: 'Blocked by Aerolith' }));
                    }
                    return originalFetch(resource, init);
                };
            }

            // 2. Dual-Layer Network Request Interception: XMLHttpRequest
            if (window.XMLHttpRequest) {
                const originalOpen = XMLHttpRequest.prototype.open;
                XMLHttpRequest.prototype.open = function(method, url) {
                    const lower = (url || '').toString().toLowerCase();
                    if (
                        lower.includes('doubleclick.net') ||
                        lower.includes('googlesyndication.com') ||
                        lower.includes('google-analytics.com') ||
                        lower.includes('scorecardresearch.com') ||
                        lower.includes('taboola.com') ||
                        lower.includes('outbrain.com') ||
                        lower.includes('criteo.com') ||
                        lower.includes('adnxs.com') ||
                        lower.includes('hotjar.com') ||
                        lower.includes('clarity.ms') ||
                        lower.includes('amplitude.com') ||
                        lower.includes('segment.io') ||
                        lower.includes('mixpanel.com')
                    ) {
                        // Neutralize destination to prevent outbound network call
                        return originalOpen.call(this, method, 'data:text/plain,blocked');
                    }
                    return originalOpen.apply(this, arguments);
                };
            }

            // 3. Neutralize tracking beacons
            if (navigator.sendBeacon) {
                const originalBeacon = navigator.sendBeacon.bind(navigator);
                navigator.sendBeacon = function(url, data) {
                    if (typeof url === 'string') {
                        const lower = url.toLowerCase();
                        if (
                            lower.includes('google-analytics') ||
                            lower.includes('analytics') ||
                            lower.includes('telemetry') ||
                            lower.includes('doubleclick') ||
                            lower.includes('scorecardresearch') ||
                            lower.includes('hotjar') ||
                            lower.includes('clarity.ms')
                        ) {
                            return true;
                        }
                    }
                    return originalBeacon(url, data);
                };
            }

            // 4. Media, Form State & Scroll Tracking for Sleep Exemption & Position Restoration
            function __aerolithNotifyHost(type, payload) {
                if (window.ipc) {
                    try {
                        window.ipc.postMessage(JSON.stringify(Object.assign({ type: type }, payload)));
                    } catch (e) {}
                }
            }

            document.addEventListener('play', () => {
                document.documentElement.dataset.aerolithMediaPlaying = 'true';
                __aerolithNotifyHost('media_playing', { value: true });
            }, true);
            document.addEventListener('pause', () => {
                const audios = Array.from(document.querySelectorAll('audio, video'));
                const anyPlaying = audios.some(a => !a.paused && !a.ended);
                document.documentElement.dataset.aerolithMediaPlaying = anyPlaying ? 'true' : 'false';
                __aerolithNotifyHost('media_playing', { value: anyPlaying });
            }, true);
            document.addEventListener('input', () => {
                document.documentElement.dataset.aerolithFormDirty = 'true';
                __aerolithNotifyHost('form_dirty', { value: true });
            }, true);

            let __aerolithScrollDebounce = null;
            window.addEventListener('scroll', () => {
                clearTimeout(__aerolithScrollDebounce);
                __aerolithScrollDebounce = setTimeout(() => {
                    __aerolithNotifyHost('scroll', {
                        x: Math.round(window.scrollX || window.pageXOffset || 0),
                        y: Math.round(window.scrollY || window.pageYOffset || 0)
                    });
                }, 150);
            }, { passive: true });
        })();
        "#
    }

    /// Script to throttle Tier 1 sleeping background tabs
    pub fn get_tier1_throttle_script(enable: bool) -> &'static str {
        if enable {
            r#"
            (function() {
                if (window.__aerolithThrottled) return;
                window.__aerolithThrottled = true;
                if (!window.__origRaf) {
                    window.__origRaf = window.requestAnimationFrame;
                    window.requestAnimationFrame = function(cb) {
                        return setTimeout(cb, 1000); // Throttle RAF to 1 FPS for background tab
                    };
                }
            })();
            "#
        } else {
            r#"
            (function() {
                if (!window.__aerolithThrottled) return;
                window.__aerolithThrottled = false;
                if (window.__origRaf) {
                    window.requestAnimationFrame = window.__origRaf;
                    window.__origRaf = null;
                }
            })();
            "#
        }
    }

    /// Script to restore exact scroll offset when waking from deep hibernation
    pub fn get_tab_restore_script(scroll_x: u32, scroll_y: u32) -> String {
        format!(
            r#"
            (function() {{
                try {{
                    window.scrollTo({}, {});
                }} catch (e) {{}}
            }})();
            "#,
            scroll_x, scroll_y
        )
    }

    /// Touch Gesture Script: Edge Swipe Back / Forward for Surface Go 2
    pub fn get_touch_gesture_script() -> &'static str {
        r#"
        (function() {
            let startX = 0;
            let startY = 0;
            let isEdgeSwipe = false;
            let direction = null;

            window.addEventListener('touchstart', (e) => {
                if (!e.touches || e.touches.length !== 1) return;
                startX = e.touches[0].clientX;
                startY = e.touches[0].clientY;
                const edgeThreshold = 35;

                if (startX <= edgeThreshold) {
                    isEdgeSwipe = true;
                    direction = 'back';
                } else if (startX >= window.innerWidth - edgeThreshold) {
                    isEdgeSwipe = true;
                    direction = 'forward';
                } else {
                    isEdgeSwipe = false;
                    direction = null;
                }
            }, { passive: true });

            window.addEventListener('touchend', (e) => {
                if (!isEdgeSwipe || !direction) return;
                const endX = e.changedTouches[0].clientX;
                const endY = e.changedTouches[0].clientY;
                const deltaX = endX - startX;
                const deltaY = Math.abs(endY - startY);

                // Ensure horizontal swipe with minimal vertical drift
                if (deltaY < 100) {
                    if (direction === 'back' && deltaX > 80) {
                        window.history.back();
                    } else if (direction === 'forward' && deltaX < -80) {
                        window.history.forward();
                    }
                }
                isEdgeSwipe = false;
                direction = null;
            }, { passive: true });
        })();
        "#
    }

    /// Dark Mode Toggle Script
    pub fn get_dark_mode_script(enable: bool) -> &'static str {
        if enable {
            r#"
            (function() {
                let el = document.getElementById('aerolith-dark-mode');
                if (!el) {
                    el = document.createElement('style');
                    el.id = 'aerolith-dark-mode';
                    el.textContent = `
                        html {
                            filter: invert(90%) hue-rotate(180deg) !important;
                            background: #121212 !important;
                        }
                        img, video, canvas, iframe, [style*="background-image"] {
                            filter: invert(100%) hue-rotate(180deg) !important;
                        }
                    `;
                    document.head && document.head.appendChild(el);
                }
            })();
            "#
        } else {
            r#"
            (function() {
                const el = document.getElementById('aerolith-dark-mode');
                if (el) el.remove();
            })();
            "#
        }
    }

    /// Reader Mode / Focus Extraction Script with XSS sanitization
    pub fn get_reader_mode_script() -> &'static str {
        r#"
        (function() {
            let existing = document.getElementById('aerolith-reader-view');
            if (existing) {
                existing.remove();
                document.body.style.display = '';
                return;
            }

            // Find main content container
            const candidates = document.querySelectorAll('article, main, .post-content, .entry-content, [role="main"]');
            let mainEl = null;
            let maxLen = 0;
            candidates.forEach(c => {
                const len = c.innerText ? c.innerText.length : 0;
                if (len > maxLen) {
                    maxLen = len;
                    mainEl = c;
                }
            });

            if (!mainEl || maxLen < 200) {
                mainEl = document.body;
            }

            const title = document.title || 'Reader View';
            
            // Clone and sanitize DOM to prevent XSS
            const clone = mainEl.cloneNode(true);
            const dangerous = clone.querySelectorAll('script, style, iframe, object, embed, form, input, button, select, textarea');
            dangerous.forEach(el => el.remove());

            // Strip inline on* event handlers and dangerous pseudo-protocols from all remaining attributes
            const allElements = clone.querySelectorAll('*');
            allElements.forEach(el => {
                for (let i = el.attributes.length - 1; i >= 0; i--) {
                    const attrName = el.attributes[i].name.toLowerCase();
                    if (attrName.startsWith('on')) {
                        el.removeAttribute(el.attributes[i].name);
                    }
                }
                if (el.hasAttribute('href')) {
                    const h = el.getAttribute('href').trim().toLowerCase();
                    if (h.startsWith('javascript:') || h.startsWith('vbscript:') || h.startsWith('data:')) {
                        el.removeAttribute('href');
                    }
                }
                if (el.hasAttribute('src')) {
                    const s = el.getAttribute('src').trim().toLowerCase();
                    if (s.startsWith('javascript:') || s.startsWith('vbscript:')) {
                        el.removeAttribute('src');
                    }
                }
            });

            const readerDiv = document.createElement('div');
            readerDiv.id = 'aerolith-reader-view';
            readerDiv.style.cssText = `
                position: fixed;
                top: 0; left: 0; width: 100vw; height: 100vh;
                background: #18191c;
                color: #e3e5e8;
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Georgia, serif;
                font-size: 19px;
                line-height: 1.7;
                overflow-y: auto;
                z-index: 2147483647;
                padding: 40px 20px 80px 20px;
                box-sizing: border-box;
            `;

            const container = document.createElement('div');
            container.style.cssText = 'max-width: 720px; margin: 0 auto;';

            const header = document.createElement('div');
            header.style.cssText = 'display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; border-bottom: 1px solid #333; padding-bottom: 12px;';

            const badge = document.createElement('span');
            badge.style.cssText = 'font-size: 13px; text-transform: uppercase; letter-spacing: 1px; color: #5e6ad2; font-weight: 600;';
            badge.textContent = 'Aerolith Reader Mode';

            const closeBtn = document.createElement('button');
            closeBtn.style.cssText = 'background: #2a2d39; border: none; color: #fff; padding: 6px 14px; border-radius: 6px; cursor: pointer; font-size: 14px;';
            closeBtn.textContent = 'Close ✕';
            closeBtn.onclick = () => {
                const el = document.getElementById('aerolith-reader-view');
                if (el) el.remove();
            };

            header.appendChild(badge);
            header.appendChild(closeBtn);

            const h1 = document.createElement('h1');
            h1.style.cssText = 'font-size: 28px; line-height: 1.3; margin-bottom: 24px; color: #ffffff;';
            h1.textContent = title;

            const bodyDiv = document.createElement('div');
            bodyDiv.className = 'reader-body';
            bodyDiv.style.cssText = 'word-break: break-word;';
            bodyDiv.appendChild(clone);

            container.appendChild(header);
            container.appendChild(h1);
            container.appendChild(bodyDiv);
            readerDiv.appendChild(container);

            document.body.appendChild(readerDiv);

            // Escape key dismisses reader mode
            window.addEventListener('keydown', function escReader(e) {
                if (e.key === 'Escape') {
                    const el = document.getElementById('aerolith-reader-view');
                    if (el) el.remove();
                    window.removeEventListener('keydown', escReader);
                }
            });
        })();
        "#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adblock_default_state() {
        let blocker = AdBlocker::new();
        assert!(blocker.enabled);
        assert!(!blocker.dark_mode);
        assert!(!blocker.get_cosmetic_css().is_empty());
        assert!(blocker.get_dark_mode_init_script().is_empty());
    }

    #[test]
    fn test_adblock_disabled_returns_empty_css() {
        let mut blocker = AdBlocker::new();
        blocker.enabled = false;
        assert_eq!(blocker.get_cosmetic_css(), "");
        assert_eq!(blocker.get_neutralizer_script(), "");
    }

    #[test]
    fn test_shield_toggle_script() {
        let on_script = AdBlocker::get_shield_toggle_script(true);
        let off_script = AdBlocker::get_shield_toggle_script(false);

        assert!(on_script.contains("aerolith-adblock-shield"));
        assert!(on_script.contains("display: none !important"));
        assert!(off_script.contains("s.remove()"));
    }

    #[test]
    fn test_dark_mode_toggle_script() {
        let dark_on = AdBlocker::get_dark_mode_script(true);
        let dark_off = AdBlocker::get_dark_mode_script(false);

        assert!(dark_on.contains("invert(90%)"));
        assert!(dark_on.contains("#121212"));
        assert!(dark_off.contains("el.remove()"));

        let mut blocker = AdBlocker::new();
        blocker.dark_mode = true;
        assert!(blocker.get_dark_mode_init_script().contains("aerolith-dark-mode"));
    }

    #[test]
    fn test_reader_mode_sanitization_script() {
        let reader_script = AdBlocker::get_reader_mode_script();
        assert!(reader_script.contains("aerolith-reader-view"));
        assert!(reader_script.contains("script, style, iframe, object, embed, form, input, button, select, textarea"));
        assert!(reader_script.contains("dangerous.forEach(el => el.remove())"));
        assert!(reader_script.contains("attrName.startsWith('on')"));
        assert!(reader_script.contains("el.removeAttribute(el.attributes[i].name)"));
        assert!(reader_script.contains("h.startsWith('javascript:')"));
    }

    #[test]
    fn test_is_tracker_or_ad_domain() {
        assert!(AdBlocker::is_tracker_or_ad_domain("https://doubleclick.net/ad"));
        assert!(AdBlocker::is_tracker_or_ad_domain("http://adservice.google.com/test"));
        assert!(AdBlocker::is_tracker_or_ad_domain("https://sub.taboola.com/pixel"));
        assert!(AdBlocker::is_tracker_or_ad_domain("https://criteo.com"));
        assert!(AdBlocker::is_tracker_or_ad_domain("hotjar.com"));

        assert!(!AdBlocker::is_tracker_or_ad_domain("https://duckduckgo.com"));
        assert!(!AdBlocker::is_tracker_or_ad_domain("https://github.com/rust-lang"));
        assert!(!AdBlocker::is_tracker_or_ad_domain("https://crates.io"));
    }

    #[test]
    fn test_tier1_throttle_script() {
        let on = AdBlocker::get_tier1_throttle_script(true);
        let off = AdBlocker::get_tier1_throttle_script(false);
        assert!(on.contains("__aerolithThrottled"));
        assert!(on.contains("requestAnimationFrame"));
        assert!(off.contains("__aerolithThrottled = false"));
    }

    #[test]
    fn test_tab_restore_script() {
        let script = AdBlocker::get_tab_restore_script(100, 450);
        assert!(script.contains("window.scrollTo(100, 450)"));
    }

    #[test]
    fn test_neutralizer_intercepts_fetch_and_xhr() {
        let blocker = AdBlocker::new();
        let script = blocker.get_neutralizer_script();
        assert!(script.contains("window.fetch"));
        assert!(script.contains("XMLHttpRequest.prototype.open"));
        assert!(script.contains("navigator.sendBeacon"));
        assert!(script.contains("aerolithMediaPlaying"));
        assert!(script.contains("aerolithFormDirty"));
        assert!(script.contains("__aerolithNotifyHost"));
        assert!(script.contains("media_playing"));
        assert!(script.contains("scroll"));
    }
}
