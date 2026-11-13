const TRACKER_DOMAINS: &[&str] = &[
    // Core Google & Ad Networks
    "doubleclick.net",
    "googlesyndication.com",
    "adservice.google.com",
    "google-analytics.com",
    "googleanalytics.com",
    "analytics.google.com",
    "stats.g.doubleclick.net",
    "pagead2.googlesyndication.com",
    // Major Ad Exchanges & Retargeters
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
    "trafficjunky.com",
    "zergnet.com",
    "adroll.com",
    "appnexus.com",
    "revcontent.com",
    "mgid.com",
    "adpushup.com",
    "sovrn.com",
    "infolinks.com",
    "indexexchange.com",
    "sharethrough.com",
    "triplelift.com",
    "gumgum.com",
    "teads.tv",
    "connatix.com",
    "undertone.com",
    // Social & Pixel Trackers
    "connect.facebook.net",
    "pixel.facebook.com",
    "an.facebook.com",
    "analytics.tiktok.com",
    "ads-api.tiktok.com",
    "static.ads-twitter.com",
    "analytics.twitter.com",
    "bat.bing.com",
    "c.bing.com",
    // Behavioral & Session Recording Telemetry
    "hotjar.com",
    "amplitude.com",
    "segment.io",
    "segment.com",
    "mixpanel.com",
    "clarity.ms",
    "fullstory.com",
    "mouseflow.com",
    "crazyegg.com",
    "heapanalytics.com",
    "loggly.com",
    "datadoghq.com",
    "sentry.io",
    "bugsnag.com",
    "newrelic.com",
    "chartbeat.com",
    "mc.yandex.ru",
    // Attribution & Fingerprint Brokers
    "appsflyer.com",
    "adjust.com",
    "branch.io",
    "kochava.com",
    "demdex.net",
    "bluekai.com",
    "exelator.com",
    "krxd.net",
    "liveramp.com",
    "tapad.com",
    "lotame.com",
    "agkn.com",
    "fpjs.io",
    "fingerprintjs.com",
];

pub struct AdBlocker {
    pub enabled: bool,
    pub dark_mode: bool,
    pub block_short_form: bool,
}

impl AdBlocker {
    pub fn new() -> Self {
        Self {
            enabled: true,
            dark_mode: false,
            block_short_form: true,
        }
    }

    /// Checks if a URL targets short-form algorithmic loops (YouTube Shorts, Instagram Reels, TikTok, Facebook Reels, Snapchat Spotlight)
    pub fn is_short_form_url(url: &str) -> bool {
        let lower = url.trim().to_lowercase();
        if lower.contains("youtube.com/shorts/") || lower.contains("youtu.be/shorts/") {
            return true;
        }
        if lower.contains("instagram.com/reel/") || lower.contains("instagram.com/reels") {
            return true;
        }
        if lower.contains("tiktok.com") {
            return true;
        }
        if lower.contains("facebook.com/reel/") || lower.contains("facebook.com/reels") {
            return true;
        }
        if lower.contains("snapchat.com/spotlight") {
            return true;
        }
        false
    }

    /// Rewrites short-form algorithmic dopamine trap URLs into standard intentional layouts
    /// - YouTube Shorts (`/shorts/{id}`) -> Desktop Watch (`/watch?v={id}`)
    /// - Instagram Reels (`/reel/{id}`) -> Standard Post (`/p/{id}/`)
    /// - Instagram Reels Feed (`/reels/`) -> Instagram Home (`https://www.instagram.com/`)
    /// - Facebook Reels (`/reel/{id}`) -> Facebook Watch (`/watch/?v={id}`)
    /// - Facebook Reels Feed (`/reels/`) -> Facebook Home (`https://www.facebook.com/`)
    /// - TikTok (`tiktok.com`) -> Focused mindful block notice
    pub fn sanitize_short_form_url(url: &str) -> Option<String> {
        let trimmed = url.trim();
        let lower = trimmed.to_lowercase();

        // 1. YouTube Shorts: https://(www.|m.)?youtube.com/shorts/{id}(?query) -> https://www.youtube.com/watch?v={id}(&query)
        if lower.contains("youtube.com/shorts/") || lower.contains("youtu.be/shorts/") {
            let marker = "/shorts/";
            if let Some(idx) = lower.find(marker) {
                let after_shorts = &trimmed[idx + marker.len()..];
                // Video ID ends at '/', '?', '#', or end of string
                let id_end = after_shorts.find(|c| c == '/' || c == '?' || c == '#').unwrap_or(after_shorts.len());
                let video_id = &after_shorts[..id_end];
                if !video_id.is_empty() {
                    let remainder = &after_shorts[id_end..];
                    let extra_query = if remainder.starts_with('?') {
                        // Replace '?' with '&' since we already have ?v=
                        format!("&{}", &remainder[1..])
                    } else if let Some(q_pos) = remainder.find('?') {
                        format!("&{}", &remainder[q_pos + 1..])
                    } else {
                        String::new()
                    };
                    return Some(format!("https://www.youtube.com/watch?v={}{}", video_id, extra_query));
                }
            }
        }

        // 2. Instagram Reels: /reel/{id} -> /p/{id}/, /reels/ -> /
        if lower.contains("instagram.com/reel/") {
            let marker = "instagram.com/reel/";
            if let Some(idx) = lower.find(marker) {
                let after_reel = &trimmed[idx + marker.len()..];
                let id_end = after_reel.find(|c| c == '/' || c == '?' || c == '#').unwrap_or(after_reel.len());
                let reel_id = &after_reel[..id_end];
                if !reel_id.is_empty() {
                    return Some(format!("https://www.instagram.com/p/{}/", reel_id));
                }
            }
        } else if lower.contains("instagram.com/reels") {
            return Some("https://www.instagram.com/".to_string());
        }

        // 3. Facebook Reels: /reel/{id} -> /watch/?v={id}, /reels/ -> /
        if lower.contains("facebook.com/reel/") {
            let marker = "facebook.com/reel/";
            if let Some(idx) = lower.find(marker) {
                let after_reel = &trimmed[idx + marker.len()..];
                let id_end = after_reel.find(|c| c == '/' || c == '?' || c == '#').unwrap_or(after_reel.len());
                let reel_id = &after_reel[..id_end];
                if !reel_id.is_empty() {
                    return Some(format!("https://www.facebook.com/watch/?v={}", reel_id));
                }
            }
        } else if lower.contains("facebook.com/reels") {
            return Some("https://www.facebook.com/".to_string());
        }

        // 4. TikTok: Dedicated short-form video platform
        if lower.contains("tiktok.com") {
            return Some("about:blank#short_form_blocked".to_string());
        }

        // 5. Snapchat Spotlight
        if lower.contains("snapchat.com/spotlight") {
            return Some("https://www.snapchat.com/".to_string());
        }

        None
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

    /// Cosmetic CSS hiding common ad frames, banners, and sponsored widgets
    pub fn get_ad_cosmetic_css() -> &'static str {
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

    /// Cosmetic CSS hiding infinite short-form content loops (YouTube Shorts, Reels, Spotlight)
    pub fn get_short_form_css() -> &'static str {
        r#"
        /* Aerolith Surface Go 2 Short-Form Content Shield */
        ytd-reel-shelf-renderer,
        ytd-rich-shelf-renderer[is-shorts],
        ytd-rich-section-renderer:has(ytd-rich-shelf-renderer[is-shorts]),
        ytd-guide-entry-renderer:has(a[title="Shorts"]),
        ytd-guide-entry-renderer:has(a[href*="/shorts"]),
        ytd-mini-guide-entry-renderer[aria-label="Shorts"],
        ytd-mini-guide-entry-renderer:has(a[href*="/shorts"]),
        a[title="Shorts"],
        a[href^="/shorts"],
        a[href*="/shorts/"],
        ytd-notification-renderer:has(a[href*="/shorts/"]),
        #shorts-container,
        ytd-shorts,
        ytm-reel-shelf-renderer,
        ytm-pivot-bar-item-renderer:has([aria-label="Shorts"]),
        ytm-compact-show-renderer:has(a[href*="/shorts"]),
        a[href*="/reels/"],
        a[href^="/reel/"],
        a[href*="/reel/"],
        svg[aria-label="Reels"],
        div:has(> svg[aria-label="Reels"]),
        div[aria-label*="Reels"],
        div[data-pagelet*="Reels"],
        a[href*="snapchat.com/spotlight"],
        a[href^="/shorts/"] {
            display: none !important;
            visibility: hidden !important;
            height: 0 !important;
            width: 0 !important;
            max-height: 0 !important;
            max-width: 0 !important;
            opacity: 0 !important;
            pointer-events: none !important;
            overflow: hidden !important;
        }
        "#
    }

    /// Returns cosmetic CSS rule injected into all pages to hide common ad frames and short-form loops
    pub fn get_cosmetic_css(&self) -> String {
        let mut css = String::new();
        if self.enabled {
            css.push_str(Self::get_ad_cosmetic_css());
        }
        if self.block_short_form {
            css.push_str(Self::get_short_form_css());
        }
        css
    }

    /// Script to dynamically toggle short-form content shield on existing open tabs
    pub fn get_short_form_toggle_script(enable: bool) -> &'static str {
        if enable {
            r#"
            (function() {
                let s = document.getElementById('aerolith-short-form-shield');
                if (!s) {
                    s = document.createElement('style');
                    s.id = 'aerolith-short-form-shield';
                    s.textContent = `
                        ytd-reel-shelf-renderer,
                        ytd-rich-shelf-renderer[is-shorts],
                        ytd-rich-section-renderer:has(ytd-rich-shelf-renderer[is-shorts]),
                        ytd-guide-entry-renderer:has(a[title="Shorts"]),
                        ytd-guide-entry-renderer:has(a[href*="/shorts"]),
                        ytd-mini-guide-entry-renderer[aria-label="Shorts"],
                        ytd-mini-guide-entry-renderer:has(a[href*="/shorts"]),
                        a[title="Shorts"],
                        a[href^="/shorts"],
                        a[href*="/shorts/"],
                        ytd-notification-renderer:has(a[href*="/shorts/"]),
                        #shorts-container,
                        ytd-shorts,
                        ytm-reel-shelf-renderer,
                        ytm-pivot-bar-item-renderer:has([aria-label="Shorts"]),
                        ytm-compact-show-renderer:has(a[href*="/shorts"]),
                        a[href*="/reels/"],
                        a[href^="/reel/"],
                        a[href*="/reel/"],
                        svg[aria-label="Reels"],
                        div:has(> svg[aria-label="Reels"]),
                        div[aria-label*="Reels"],
                        div[data-pagelet*="Reels"],
                        a[href*="snapchat.com/spotlight"],
                        a[href^="/shorts/"] {
                            display: none !important;
                            visibility: hidden !important;
                            height: 0 !important;
                            width: 0 !important;
                            max-height: 0 !important;
                            max-width: 0 !important;
                            opacity: 0 !important;
                            pointer-events: none !important;
                            overflow: hidden !important;
                        }
                    `;
                    const root = document.head || document.documentElement || document.body;
                    root && root.appendChild(s);
                }
            })();
            "#
        } else {
            r#"
            (function() {
                const s = document.getElementById('aerolith-short-form-shield');
                if (s) s.remove();
            })();
            "#
        }
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
                    const root = document.head || document.documentElement || document.body;
                    root && root.appendChild(s);
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
                        __aerolithNotifyHost('ad_blocked', {});
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
                        __aerolithNotifyHost('ad_blocked', {});
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
                            __aerolithNotifyHost('ad_blocked', {});
                            return true;
                        }
                    }
                    return originalBeacon(url, data);
                };
            }

            // 4. Anti-Fingerprinting Shield: Canvas Micro-Noise Farbling
            try {
                const origToDataURL = HTMLCanvasElement.prototype.toDataURL;
                HTMLCanvasElement.prototype.toDataURL = function() {
                    const ctx = this.getContext('2d');
                    if (ctx && this.width > 16 && this.height > 16) {
                        try {
                            const imgData = ctx.getImageData(0, 0, Math.min(this.width, 32), Math.min(this.height, 32));
                            for (let i = 0; i < imgData.data.length; i += 16) {
                                imgData.data[i] = (imgData.data[i] ^ 1);
                            }
                            ctx.putImageData(imgData, 0, 0);
                        } catch (e) {}
                    }
                    return origToDataURL.apply(this, arguments);
                };

                const origToBlob = HTMLCanvasElement.prototype.toBlob;
                HTMLCanvasElement.prototype.toBlob = function(callback, type, quality) {
                    const ctx = this.getContext('2d');
                    if (ctx && this.width > 16 && this.height > 16) {
                        try {
                            const imgData = ctx.getImageData(0, 0, Math.min(this.width, 32), Math.min(this.height, 32));
                            for (let i = 0; i < imgData.data.length; i += 16) {
                                imgData.data[i] = (imgData.data[i] ^ 1);
                            }
                            ctx.putImageData(imgData, 0, 0);
                        } catch (e) {}
                    }
                    return origToBlob.apply(this, arguments);
                };

                const origGetImageData = CanvasRenderingContext2D.prototype.getImageData;
                CanvasRenderingContext2D.prototype.getImageData = function(sx, sy, sw, sh) {
                    const imgData = origGetImageData.apply(this, arguments);
                    if (sw > 8 && sh > 8 && imgData && imgData.data) {
                        for (let i = 0; i < imgData.data.length; i += 32) {
                            imgData.data[i] = (imgData.data[i] ^ 1);
                        }
                    }
                    return imgData;
                };
            } catch (e) {}

            // 5. Anti-Fingerprinting Shield: AudioContext Oscillator Noise Injection
            try {
                if (window.AudioBuffer) {
                    const origGetChannelData = AudioBuffer.prototype.getChannelData;
                    AudioBuffer.prototype.getChannelData = function() {
                        const buffer = origGetChannelData.apply(this, arguments);
                        for (let i = 0; i < buffer.length; i += 100) {
                            buffer[i] += 0.0000001 * (Math.random() - 0.5);
                        }
                        return buffer;
                    };
                }
                if (window.AnalyserNode) {
                    const origGetFloatFrequencyData = AnalyserNode.prototype.getFloatFrequencyData;
                    AnalyserNode.prototype.getFloatFrequencyData = function(array) {
                        origGetFloatFrequencyData.apply(this, arguments);
                        for (let i = 0; i < array.length; i += 10) {
                            array[i] += 0.0001 * (Math.random() - 0.5);
                        }
                    };
                }
            } catch (e) {}

            // 6. Anti-Fingerprinting Shield: Hardware & Battery Normalization + DNT & GPC
            try {
                if (navigator.getBattery) {
                    navigator.getBattery = function() {
                        return Promise.resolve({
                            charging: true,
                            chargingTime: 0,
                            dischargingTime: Infinity,
                            level: 1.0,
                            addEventListener: function() {},
                            removeEventListener: function() {},
                            dispatchEvent: function() { return false; }
                        });
                    };
                }
                try { Object.defineProperty(navigator, 'hardwareConcurrency', { get: () => 4, configurable: true }); } catch (e) {}
                try { Object.defineProperty(navigator, 'deviceMemory', { get: () => 8, configurable: true }); } catch (e) {}
                try { Object.defineProperty(navigator, 'doNotTrack', { get: () => "1", configurable: true }); } catch (e) {}
                try { Object.defineProperty(navigator, 'globalPrivacyControl', { get: () => true, configurable: true }); } catch (e) {}
                try { Object.defineProperty(window, 'globalPrivacyControl', { get: () => true, configurable: true }); } catch (e) {}
            } catch (e) {}

            // 7. WebRTC Local Host IP Leak Protection
            try {
                if (window.RTCPeerConnection) {
                    const origAddIceCandidate = RTCPeerConnection.prototype.addIceCandidate;
                    RTCPeerConnection.prototype.addIceCandidate = function(candidate) {
                        if (candidate && candidate.candidate) {
                            const c = candidate.candidate;
                            if (c.includes(' typ host ') && (
                                c.includes(' 192.168.') ||
                                c.includes(' 10.') ||
                                c.includes(' 172.16.') ||
                                c.includes(' 172.17.') ||
                                c.includes(' 172.18.') ||
                                c.includes(' 172.19.') ||
                                c.includes(' 172.2') ||
                                c.includes(' 172.3')
                            )) {
                                return Promise.resolve();
                            }
                        }
                        return origAddIceCandidate.apply(this, arguments);
                    };
                }
            } catch (e) {}

            // 8. Hyperlink Auditing (<a ping>) Protection
            try {
                const stripPings = () => {
                    document.querySelectorAll('a[ping]').forEach(a => a.removeAttribute('ping'));
                };
                if (document.readyState === 'loading') {
                    document.addEventListener('DOMContentLoaded', stripPings, { once: true });
                } else {
                    stripPings();
                }
            } catch (e) {}

            // 9. Media, Form State & Scroll Tracking for Sleep Exemption & Position Restoration
            function __aerolithNotifyHost(type, payload) {
                if (window.ipc) {
                    try {
                        window.ipc.postMessage(JSON.stringify(Object.assign({ type: type }, payload)));
                    } catch (e) {}
                }
            }

            document.addEventListener('play', (e) => {
                if (document.documentElement.dataset.aerolithTabMuted === 'true' && e.target && 'muted' in e.target) {
                    e.target.muted = true;
                }
                document.documentElement.dataset.aerolithMediaPlaying = 'true';
                __aerolithNotifyHost('media_playing', { value: true });
            }, true);
            document.addEventListener('pause', () => {
                const audios = Array.from(document.querySelectorAll('audio, video'));
                const anyPlaying = audios.some(a => !a.paused && !a.ended);
                document.documentElement.dataset.aerolithMediaPlaying = anyPlaying ? 'true' : 'false';
                __aerolithNotifyHost('media_playing', { value: anyPlaying });
            }, true);
            document.addEventListener('ended', () => {
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

            // 5. Client-Side SPA Navigation & History Tracking (pushState, replaceState, popstate, hashchange)
            function __aerolithNotifyLocation() {
                try {
                    __aerolithRedirectShorts();
                    __aerolithNotifyHost('spa_navigation', {
                        url: window.location.href,
                        title: document.title || ''
                    });
                } catch (e) {}
            }

            if (window.history) {
                const __origPushState = history.pushState;
                history.pushState = function() {
                    const ret = __origPushState.apply(this, arguments);
                    setTimeout(__aerolithNotifyLocation, 0);
                    return ret;
                };
                const __origReplaceState = history.replaceState;
                history.replaceState = function() {
                    const ret = __origReplaceState.apply(this, arguments);
                    setTimeout(__aerolithNotifyLocation, 0);
                    return ret;
                };
            }
            window.addEventListener('popstate', __aerolithNotifyLocation);
            window.addEventListener('hashchange', __aerolithNotifyLocation);

            // 6. Real-Time Short-Form Content Neutralizer (YouTube Shorts & Instagram Reels)
            function __aerolithRedirectShorts() {
                try {
                    const href = window.location.href;
                    if (href.includes('youtube.com/shorts/')) {
                        const marker = '/shorts/';
                        const idx = href.indexOf(marker);
                        if (idx !== -1) {
                            const after = href.substring(idx + marker.length);
                            const idEnd = after.search(/[\/\?\#]/);
                            const id = idEnd === -1 ? after : after.substring(0, idEnd);
                            if (id) {
                                const search = window.location.search ? window.location.search.replace('?', '&') : '';
                                window.location.replace('https://www.youtube.com/watch?v=' + id + search);
                            }
                        }
                    } else if (href.includes('instagram.com/reel/')) {
                        const marker = 'instagram.com/reel/';
                        const idx = href.indexOf(marker);
                        if (idx !== -1) {
                            const after = href.substring(idx + marker.length);
                            const idEnd = after.search(/[\/\?\#]/);
                            const id = idEnd === -1 ? after : after.substring(0, idEnd);
                            if (id) {
                                window.location.replace('https://www.instagram.com/p/' + id + '/');
                            }
                        }
                    }
                } catch (e) {}
            }

            document.addEventListener('click', function(e) {
                const link = e.target.closest && e.target.closest('a');
                if (!link || !link.href) return;
                const u = link.href;
                if (u.includes('youtube.com/shorts/')) {
                    const marker = '/shorts/';
                    const idx = u.indexOf(marker);
                    if (idx !== -1) {
                        const after = u.substring(idx + marker.length);
                        const idEnd = after.search(/[\/\?\#]/);
                        const id = idEnd === -1 ? after : after.substring(0, idEnd);
                        if (id) {
                            e.preventDefault();
                            e.stopPropagation();
                            window.location.href = 'https://www.youtube.com/watch?v=' + id;
                        }
                    }
                } else if (u.includes('instagram.com/reel/')) {
                    const marker = 'instagram.com/reel/';
                    const idx = u.indexOf(marker);
                    if (idx !== -1) {
                        const after = u.substring(idx + marker.length);
                        const idEnd = after.search(/[\/\?\#]/);
                        const id = idEnd === -1 ? after : after.substring(0, idEnd);
                        if (id) {
                            e.preventDefault();
                            e.stopPropagation();
                            window.location.href = 'https://www.instagram.com/p/' + id + '/';
                        }
                    }
                }
            }, true);

            __aerolithRedirectShorts();
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
                    const root = document.head || document.documentElement || document.body;
                    root && root.appendChild(el);
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
                if (window.ipc) {
                    try { window.ipc.postMessage(JSON.stringify({ type: 'reader_closed' })); } catch (e) {}
                }
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
                if (window.ipc) {
                    try { window.ipc.postMessage(JSON.stringify({ type: 'reader_closed' })); } catch (e) {}
                }
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
            if (window.ipc) {
                try { window.ipc.postMessage(JSON.stringify({ type: 'reader_opened' })); } catch (e) {}
            }

            // Escape key dismisses reader mode
            window.addEventListener('keydown', function escReader(e) {
                if (e.key === 'Escape') {
                    const el = document.getElementById('aerolith-reader-view');
                    if (el) {
                        el.remove();
                        if (window.ipc) {
                            try { window.ipc.postMessage(JSON.stringify({ type: 'reader_closed' })); } catch (e) {}
                        }
                    }
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
        blocker.block_short_form = false;
        assert_eq!(blocker.get_cosmetic_css(), "");
        assert_eq!(blocker.get_neutralizer_script(), "");
    }

    #[test]
    fn test_is_short_form_url() {
        assert!(AdBlocker::is_short_form_url("https://www.youtube.com/shorts/dQw4w9WgXcQ"));
        assert!(AdBlocker::is_short_form_url("https://m.youtube.com/shorts/12345"));
        assert!(AdBlocker::is_short_form_url("https://youtu.be/shorts/abcdef"));
        assert!(AdBlocker::is_short_form_url("https://www.instagram.com/reel/C312345/"));
        assert!(AdBlocker::is_short_form_url("https://instagram.com/reels"));
        assert!(AdBlocker::is_short_form_url("https://www.tiktok.com/@creator/video/123"));
        assert!(AdBlocker::is_short_form_url("https://tiktok.com"));
        assert!(AdBlocker::is_short_form_url("https://www.facebook.com/reel/555123"));
        assert!(AdBlocker::is_short_form_url("https://facebook.com/reels/"));
        assert!(AdBlocker::is_short_form_url("https://snapchat.com/spotlight"));

        // Standard non-short-form URLs must return false
        assert!(!AdBlocker::is_short_form_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(!AdBlocker::is_short_form_url("https://www.instagram.com/p/C312345/"));
        assert!(!AdBlocker::is_short_form_url("https://duckduckgo.com/?q=shorts"));
        assert!(!AdBlocker::is_short_form_url("https://github.com/rust-lang/rust"));
    }

    #[test]
    fn test_sanitize_short_form_url() {
        // YouTube Shorts -> Desktop Watch
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://www.youtube.com/shorts/dQw4w9WgXcQ"),
            Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://m.youtube.com/shorts/dQw4w9WgXcQ?si=abc123xyz"),
            Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ&si=abc123xyz".to_string())
        );
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://youtube.com/shorts/abc-123_xyz/"),
            Some("https://www.youtube.com/watch?v=abc-123_xyz".to_string())
        );

        // Instagram Reels -> Post / Home
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://www.instagram.com/reel/C7aBcDeFgHi/"),
            Some("https://www.instagram.com/p/C7aBcDeFgHi/".to_string())
        );
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://www.instagram.com/reels/"),
            Some("https://www.instagram.com/".to_string())
        );

        // Facebook Reels -> Watch / Home
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://www.facebook.com/reel/123456789/"),
            Some("https://www.facebook.com/watch/?v=123456789".to_string())
        );
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://www.facebook.com/reels/"),
            Some("https://www.facebook.com/".to_string())
        );

        // TikTok -> Block Page
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://www.tiktok.com/@user/video/99999"),
            Some("about:blank#short_form_blocked".to_string())
        );

        // Snapchat Spotlight -> Home
        assert_eq!(
            AdBlocker::sanitize_short_form_url("https://snapchat.com/spotlight"),
            Some("https://www.snapchat.com/".to_string())
        );

        // Normal URLs unchanged
        assert_eq!(AdBlocker::sanitize_short_form_url("https://www.youtube.com/watch?v=123"), None);
        assert_eq!(AdBlocker::sanitize_short_form_url("https://duckduckgo.com"), None);
    }

    #[test]
    fn test_short_form_cosmetic_css_and_toggle() {
        let blocker = AdBlocker::new();
        assert!(blocker.block_short_form);
        let css = blocker.get_cosmetic_css();
        assert!(css.contains("ytd-reel-shelf-renderer"));
        assert!(css.contains("ytd-rich-shelf-renderer[is-shorts]"));
        assert!(css.contains("a[title=\"Shorts\"]"));
        assert!(css.contains("a[href*=\"/reels/\"]"));

        let toggle_on = AdBlocker::get_short_form_toggle_script(true);
        let toggle_off = AdBlocker::get_short_form_toggle_script(false);
        assert!(toggle_on.contains("aerolith-short-form-shield"));
        assert!(toggle_on.contains("ytd-reel-shelf-renderer"));
        assert!(toggle_off.contains("aerolith-short-form-shield"));
        assert!(toggle_off.contains("s.remove()"));
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
        assert!(reader_script.contains("reader_opened"));
        assert!(reader_script.contains("reader_closed"));
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
        assert!(script.contains("ad_blocked"));
        assert!(script.contains("media_playing"));
        assert!(script.contains("ended"));
        assert!(script.contains("scroll"));
        assert!(script.contains("HTMLCanvasElement.prototype.toDataURL"));
        assert!(script.contains("globalPrivacyControl"));
        assert!(script.contains("hardwareConcurrency"));
    }
}
