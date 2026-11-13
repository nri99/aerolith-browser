use crate::types::HOME_URL;

#[inline]
pub fn clean_js_json(json: &str) -> String {
    json.replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029")
}

pub fn sanitize_title(title: &str) -> String {
    if title.chars().count() > 256 {
        title.chars().take(256).collect()
    } else {
        title.to_string()
    }
}

pub const TRACKING_QUERY_PARAMS: &[&str] = &[
    "utm_source",
    "utm_medium",
    "utm_campaign",
    "utm_term",
    "utm_content",
    "utm_id",
    "utm_source_platform",
    "utm_creative_format",
    "utm_marketing_tactic",
    "fbclid",
    "fbadid",
    "gclid",
    "gbraid",
    "wbraid",
    "dclid",
    "msclkid",
    "twclid",
    "ttclid",
    "igshid",
    "mc_eid",
    "mc_cid",
    "_hsenc",
    "_hsmi",
    "hscctstracking",
    "yclid",
    "wickedid",
    "zanpid",
    "si",
];

pub fn is_tracking_param(name: &str) -> bool {
    let lower = name.trim().to_lowercase();
    lower.starts_with("utm_") || TRACKING_QUERY_PARAMS.iter().any(|&tp| tp == lower)
}

pub fn strip_tracking_parameters(url: &str) -> String {
    let trimmed = url.trim();
    if !trimmed.contains('?') {
        return trimmed.to_string();
    }

    let (base_and_query, fragment) = match trimmed.find('#') {
        Some(idx) => (&trimmed[..idx], Some(&trimmed[idx..])),
        None => (trimmed, None),
    };

    let q_pos = match base_and_query.find('?') {
        Some(idx) => idx,
        None => return trimmed.to_string(),
    };

    let base = &base_and_query[..q_pos];
    let query = &base_and_query[q_pos + 1..];

    let mut kept_params = Vec::new();
    let mut modified = false;

    for param in query.split('&') {
        if param.is_empty() {
            continue;
        }
        let key = match param.find('=') {
            Some(eq_pos) => &param[..eq_pos],
            None => param,
        };
        if is_tracking_param(key) {
            modified = true;
        } else {
            kept_params.push(param);
        }
    }

    if !modified {
        return trimmed.to_string();
    }

    let mut result = base.to_string();
    if !kept_params.is_empty() {
        result.push('?');
        result.push_str(&kept_params.join("&"));
    }
    if let Some(frag) = fragment {
        result.push_str(frag);
    }
    result
}

pub fn build_search_url(query: &str, engine: &str) -> String {
    let encoded = urlencoding_simple(query);
    match engine.trim().to_lowercase().as_str() {
        "google" => format!("https://www.google.com/search?q={}", encoded),
        "brave" => format!("https://search.brave.com/search?q={}", encoded),
        "bing" => format!("https://www.bing.com/search?q={}", encoded),
        _ => format!("https://duckduckgo.com/?q={}", encoded),
    }
}

pub fn normalize_url_with_engine(input: &str, engine: &str) -> String {
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
    if lower == "about:home" || lower == "about:start" || lower == "about:aerolith" {
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
        return build_search_url(trimmed, engine);
    }

    // Case-insensitive scheme check for supported navigation protocols
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("file://") {
        return strip_tracking_parameters(trimmed);
    }

    // If input contains another protocol scheme (e.g. ftp://, ws://, gopher://, custom://),
    // treat as a search query rather than prepending https:// which creates a malformed URL
    if lower.contains("://") {
        return build_search_url(trimmed, engine);
    }

    // Localhost / Loopback addresses (default to HTTP)
    if lower.starts_with("localhost") || lower.starts_with("127.0.0.1") || lower.starts_with("[::1]") {
        return strip_tracking_parameters(&format!("http://{}", trimmed));
    }

    // IPv4 address detection (e.g. 192.168.1.1 or 10.0.0.1:8080)
    let host_part = trimmed.split('/').next().unwrap_or("").split(':').next().unwrap_or("");
    let octets: Vec<&str> = host_part.split('.').collect();
    if octets.len() == 4 && octets.iter().all(|seg| seg.parse::<u8>().is_ok()) {
        return strip_tracking_parameters(&format!("http://{}", trimmed));
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
                return strip_tracking_parameters(&format!("https://{}", trimmed));
            }
        }
    }

    // Otherwise fallback to search query
    build_search_url(trimmed, engine)
}

#[inline]
#[allow(dead_code)]
pub fn normalize_url(input: &str) -> String {
    normalize_url_with_engine(input, "duckduckgo")
}

pub fn urlencoding_simple(s: &str) -> String {
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_js_json() {
        let input = "hello\u{2028}world\u{2029}!";
        let cleaned = clean_js_json(input);
        assert_eq!(cleaned, "hello\\u2028world\\u2029!");
        assert!(!cleaned.contains('\u{2028}'));
        assert!(!cleaned.contains('\u{2029}'));
    }

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
    fn test_normalize_url_unsupported_scheme_routes_to_search() {
        let ftp = normalize_url("ftp://example.com/file.zip");
        assert!(ftp.starts_with("https://duckduckgo.com/?q="));

        let ws = normalize_url("ws://localhost:8080");
        assert!(ws.starts_with("https://duckduckgo.com/?q="));
    }

    #[test]
    fn test_normalize_url_domain_with_custom_port() {
        assert_eq!(normalize_url("example.com:8080"), "https://example.com:8080");
        assert_eq!(normalize_url("sub.domain.org:3000/api"), "https://sub.domain.org:3000/api");
    }

    #[test]
    fn test_normalize_url_length_clamp() {
        let giant_str = "a".repeat(5000);
        let normalized = normalize_url(&giant_str);
        assert!(normalized.len() <= 4096 * 3);
    }

    #[test]
    fn test_urlencoding_simple() {
        assert_eq!(urlencoding_simple("hello world"), "hello+world");
        assert_eq!(urlencoding_simple("rust & web"), "rust+%26+web");
        assert_eq!(urlencoding_simple("a/b?c=d"), "a%2Fb%3Fc%3Dd");
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
    fn test_sanitize_title_clamping() {
        let normal = "Normal Title - Aerolith";
        assert_eq!(sanitize_title(normal), normal);

        let giant = "X".repeat(500);
        let clamped = sanitize_title(&giant);
        assert_eq!(clamped.chars().count(), 256);
        assert_eq!(clamped, "X".repeat(256));
    }

    #[test]
    fn test_build_search_url_engines() {
        assert_eq!(build_search_url("rust lang", "duckduckgo"), "https://duckduckgo.com/?q=rust+lang");
        assert_eq!(build_search_url("rust lang", "google"), "https://www.google.com/search?q=rust+lang");
        assert_eq!(build_search_url("rust lang", "brave"), "https://search.brave.com/search?q=rust+lang");
        assert_eq!(build_search_url("rust lang", "bing"), "https://www.bing.com/search?q=rust+lang");
    }

    #[test]
    fn test_normalize_url_with_engine() {
        assert_eq!(normalize_url_with_engine("about:home", "google"), HOME_URL);
        assert_eq!(normalize_url_with_engine("about:start", "google"), HOME_URL);
        assert_eq!(normalize_url_with_engine("surface go 2", "google"), "https://www.google.com/search?q=surface+go+2");
        assert_eq!(normalize_url_with_engine("surface go 2", "brave"), "https://search.brave.com/search?q=surface+go+2");
    }

    #[test]
    fn test_strip_tracking_parameters() {
        // Facebook & Google tracking tags
        let dirty1 = "https://example.com/shop?product=42&utm_source=twitter&utm_medium=cpc&fbclid=IwAR2xyz&gclid=CjwKxyz#details";
        let clean1 = strip_tracking_parameters(dirty1);
        assert_eq!(clean1, "https://example.com/shop?product=42#details");

        // Single tracking parameter with no others
        let dirty2 = "https://example.com/?fbclid=123456";
        let clean2 = strip_tracking_parameters(dirty2);
        assert_eq!(clean2, "https://example.com/");

        // Clean URLs remain unchanged
        let clean_original = "https://example.com/search?q=rust+lang&page=2";
        assert_eq!(strip_tracking_parameters(clean_original), clean_original);

        // URLs without query string
        let no_query = "https://example.com/article#intro";
        assert_eq!(strip_tracking_parameters(no_query), no_query);
    }
}
