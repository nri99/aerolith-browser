use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

const HISTORY_TABLE: TableDefinition<&str, &str> = TableDefinition::new("history");
const BOOKMARKS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("bookmarks");
const SESSION_TABLE: TableDefinition<&str, &str> = TableDefinition::new("session");
const SETTINGS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("settings");

pub const MAX_SESSION_TABS: usize = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub visit_count: u64,
    pub last_visited: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkEntry {
    pub url: String,
    pub title: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub tabs: Vec<String>,
    pub active_index: usize,
}

/// Sanitizes credentials out of URL authority segments strictly compliant with RFC 3986
/// (e.g. https://user:pass@example.com/path -> https://example.com/path)
/// Crucially preserves queries or paths containing '@' (e.g. duckduckgo.com/?q=user@gmail.com)
pub fn sanitize_url_credentials(url: &str) -> String {
    let trimmed = url.trim();
    if let Some(scheme_end) = trimmed.find("://") {
        let auth_start = scheme_end + 3;
        let rest = &trimmed[auth_start..];
        let auth_len = rest.find(|c| c == '/' || c == '?' || c == '#').unwrap_or(rest.len());
        let authority = &rest[..auth_len];
        let path_and_query = &rest[auth_len..];

        if let Some(last_at) = authority.rfind('@') {
            let host_port = &authority[last_at + 1..];
            format!("{}{}{}", &trimmed[..auth_start], host_port, path_and_query)
        } else {
            trimmed.to_string()
        }
    } else {
        trimmed.to_string()
    }
}

#[derive(Clone)]
pub struct BrowserStorage {
    db: Arc<Database>,
}

impl BrowserStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let base_path = path.as_ref();

        // Attempt to open the primary database. If locked by another running instance or corrupted,
        // fallback gracefully so the browser never crashes on startup.
        let db = match Database::create(base_path) {
            Ok(db) => db,
            Err(_) => {
                // If the file is locked by an existing process, try instance-specific fallback
                let mut fallback_success = None;
                for i in 2..=10 {
                    let fallback_name = format!("{}.instance{}.redb", base_path.to_string_lossy(), i);
                    let fallback_path = Path::new(&fallback_name);
                    if let Ok(db_fallback) = Database::create(fallback_path) {
                        fallback_success = Some(db_fallback);
                        break;
                    }
                }

                if let Some(db) = fallback_success {
                    db
                } else {
                    // If corrupted or unable to open, backup and recreate
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    let corrupted_name = format!("{}.corrupt.{}", base_path.to_string_lossy(), timestamp);
                    let _ = std::fs::rename(base_path, corrupted_name);
                    Database::create(base_path)?
                }
            }
        };

        // Ensure tables exist
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(HISTORY_TABLE)?;
            let _ = write_txn.open_table(BOOKMARKS_TABLE)?;
            let _ = write_txn.open_table(SESSION_TABLE)?;
            let _ = write_txn.open_table(SETTINGS_TABLE)?;
        }
        write_txn.commit()?;

        Ok(Self { db: Arc::new(db) })
    }

    pub fn record_visit(&self, url: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
        let trimmed_url = url.trim();
        if trimmed_url.is_empty()
            || trimmed_url.starts_with("data:")
            || trimmed_url.starts_with("javascript:")
            || trimmed_url.starts_with("about:")
            || trimmed_url.starts_with("blob:")
            || trimmed_url.starts_with("chrome:")
            || trimmed_url.starts_with("edge:")
        {
            return Ok(());
        }

        // Sanitize credentials out of URLs strictly in authority segment
        let sanitized_url = sanitize_url_credentials(trimmed_url);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(HISTORY_TABLE)?;
            
            // Check if existing visit was recorded recently (< 15 seconds)
            // If so, update the title without incrementing visit_count to prevent SPA / title-polling disk thrashing
            let (count, final_title) = if let Some(existing) = table.get(sanitized_url.as_str())? {
                if let Ok(entry) = serde_json::from_str::<HistoryEntry>(existing.value()) {
                    let chosen_title = if !title.trim().is_empty() {
                        title.trim().to_string()
                    } else {
                        entry.title
                    };

                    if now.saturating_sub(entry.last_visited) < 15 {
                        (entry.visit_count, chosen_title)
                    } else {
                        (entry.visit_count + 1, chosen_title)
                    }
                } else {
                    (1, if title.trim().is_empty() { sanitized_url.clone() } else { title.trim().to_string() })
                }
            } else {
                (1, if title.trim().is_empty() { sanitized_url.clone() } else { title.trim().to_string() })
            };

            let entry = HistoryEntry {
                url: sanitized_url.clone(),
                title: final_title,
                visit_count: count,
                last_visited: now,
            };

            let json = serde_json::to_string(&entry)?;
            table.insert(sanitized_url.as_str(), json.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn toggle_bookmark(&self, url: &str, title: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let trimmed_url = url.trim();
        if trimmed_url.is_empty() || trimmed_url.to_lowercase().starts_with("about:") {
            return Ok(false);
        }

        let write_txn = self.db.begin_write()?;
        let is_now_bookmarked = {
            let mut table = write_txn.open_table(BOOKMARKS_TABLE)?;
            if table.get(trimmed_url)?.is_some() {
                table.remove(trimmed_url)?;
                false
            } else {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let entry = BookmarkEntry {
                    url: trimmed_url.to_string(),
                    title: if title.trim().is_empty() { trimmed_url.to_string() } else { title.trim().to_string() },
                    created_at: now,
                };
                let json = serde_json::to_string(&entry)?;
                table.insert(trimmed_url, json.as_str())?;
                true
            }
        };
        write_txn.commit()?;
        Ok(is_now_bookmarked)
    }

    pub fn is_bookmarked(&self, url: &str) -> bool {
        if let Ok(read_txn) = self.db.begin_read() {
            if let Ok(table) = read_txn.open_table(BOOKMARKS_TABLE) {
                if let Ok(Some(_)) = table.get(url) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_bookmarks(&self) -> Vec<BookmarkEntry> {
        let mut bookmarks = Vec::new();
        if let Ok(read_txn) = self.db.begin_read() {
            if let Ok(table) = read_txn.open_table(BOOKMARKS_TABLE) {
                if let Ok(iter) = table.iter() {
                    for item in iter.flatten() {
                        if let Ok(b) = serde_json::from_str::<BookmarkEntry>(item.1.value()) {
                            bookmarks.push(b);
                        }
                    }
                }
            }
        }
        bookmarks.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        bookmarks
    }

    pub fn get_recent_history(&self, limit: usize) -> Vec<HistoryEntry> {
        let mut list = Vec::new();
        if let Ok(read_txn) = self.db.begin_read() {
            if let Ok(table) = read_txn.open_table(HISTORY_TABLE) {
                if let Ok(iter) = table.iter() {
                    for item in iter.flatten() {
                        if let Ok(h) = serde_json::from_str::<HistoryEntry>(item.1.value()) {
                            list.push(h);
                        }
                    }
                }
            }
        }
        list.sort_by(|a, b| b.last_visited.cmp(&a.last_visited));
        list.truncate(limit);
        list
    }

    pub fn delete_bookmark(&self, url: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let trimmed_url = url.trim();
        let write_txn = self.db.begin_write()?;
        let removed = {
            let mut table = write_txn.open_table(BOOKMARKS_TABLE)?;
            table.remove(trimmed_url)?.is_some()
        };
        write_txn.commit()?;
        Ok(removed)
    }

    pub fn delete_history_item(&self, url: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let trimmed_url = url.trim();
        let write_txn = self.db.begin_write()?;
        let removed = {
            let mut table = write_txn.open_table(HISTORY_TABLE)?;
            table.remove(trimmed_url)?.is_some()
        };
        write_txn.commit()?;
        Ok(removed)
    }

    pub fn clear_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(HISTORY_TABLE)?;
            let keys: Vec<String> = {
                let mut k = Vec::new();
                if let Ok(iter) = table.iter() {
                    for item in iter.flatten() {
                        k.push(item.0.value().to_string());
                    }
                }
                k
            };
            for key in keys {
                let _ = table.remove(key.as_str());
            }
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn get_setting_bool(&self, key: &str, default_val: bool) -> bool {
        if let Ok(read_txn) = self.db.begin_read() {
            if let Ok(table) = read_txn.open_table(SETTINGS_TABLE) {
                if let Ok(Some(val)) = table.get(key) {
                    return val.value() == "true";
                }
            }
        }
        default_val
    }

    pub fn set_setting_bool(&self, key: &str, val: bool) -> Result<(), Box<dyn std::error::Error>> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(SETTINGS_TABLE)?;
            table.insert(key, if val { "true" } else { "false" })?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn search(&self, query: &str, max_results: usize) -> Vec<HistoryEntry> {
        let query_trimmed = query.trim();
        if query_trimmed.is_empty() || query_trimmed.len() > 256 {
            return Vec::new();
        }
        let query_lower = query_trimmed.to_lowercase();
        let mut results = Vec::new();
        let mut seen_urls = std::collections::HashSet::new();

        if let Ok(read_txn) = self.db.begin_read() {
            // 1. Search bookmarks first so saved pages are prioritized
            if let Ok(bm_table) = read_txn.open_table(BOOKMARKS_TABLE) {
                if let Ok(iter) = bm_table.iter() {
                    for item in iter.flatten() {
                        if let Ok(b) = serde_json::from_str::<BookmarkEntry>(item.1.value()) {
                            if b.url.to_lowercase().contains(&query_lower)
                                || b.title.to_lowercase().contains(&query_lower)
                            {
                                seen_urls.insert(b.url.clone());
                                results.push(HistoryEntry {
                                    url: b.url,
                                    title: format!("★ {}", b.title),
                                    visit_count: u64::MAX / 2, // Boost priority
                                    last_visited: b.created_at,
                                });
                            }
                        }
                    }
                }
            }

            // 2. Search history for non-bookmarked matches
            if let Ok(table) = read_txn.open_table(HISTORY_TABLE) {
                if let Ok(iter) = table.iter() {
                    for item in iter.flatten() {
                        if let Ok(entry) = serde_json::from_str::<HistoryEntry>(item.1.value()) {
                            if !seen_urls.contains(&entry.url)
                                && (entry.url.to_lowercase().contains(&query_lower)
                                    || entry.title.to_lowercase().contains(&query_lower))
                            {
                                results.push(entry);
                            }
                        }
                    }
                }
            }
        }

        // Sort by visit count and recency (highest first)
        results.sort_by(|a, b| {
            b.visit_count
                .cmp(&a.visit_count)
                .then_with(|| b.last_visited.cmp(&a.last_visited))
        });

        results.truncate(max_results);
        results
    }

    pub fn save_session(&self, tabs: &[String], active_index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let safe_active_index = if tabs.is_empty() {
            0
        } else if active_index >= tabs.len() {
            tabs.len() - 1
        } else {
            active_index
        };
        let state = SessionState {
            tabs: tabs.to_vec(),
            active_index: safe_active_index,
        };
        let json = serde_json::to_string(&state)?;
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(SESSION_TABLE)?;
            table.insert("current", json.as_str())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn load_session(&self) -> Option<SessionState> {
        if let Ok(read_txn) = self.db.begin_read() {
            if let Ok(table) = read_txn.open_table(SESSION_TABLE) {
                if let Ok(Some(val)) = table.get("current") {
                    if let Ok(mut state) = serde_json::from_str::<SessionState>(val.value()) {
                        state.tabs.retain(|u| {
                            let trimmed = u.trim();
                            let lower = trimmed.to_lowercase();
                            !trimmed.is_empty()
                                && (lower.starts_with("http://")
                                    || lower.starts_with("https://")
                                    || lower.starts_with("file://")
                                    || lower.starts_with("about:blank"))
                        });
                        state.tabs.truncate(MAX_SESSION_TABS);
                        if !state.tabs.is_empty() {
                            if state.active_index >= state.tabs.len() {
                                state.active_index = state.tabs.len() - 1;
                            }
                            return Some(state);
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn temp_test_storage() -> (BrowserStorage, std::path::PathBuf) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let mut path = std::env::temp_dir();
        path.push(format!("aerolith_test_{}.redb", timestamp));
        let storage = BrowserStorage::new(&path).expect("Failed to create test storage");
        (storage, path)
    }

    #[test]
    fn test_record_visit_and_credential_sanitization() {
        let (storage, path) = temp_test_storage();

        // Standard URL
        storage.record_visit("https://duckduckgo.com", "DuckDuckGo").unwrap();
        // Sensitive credentials in URL
        storage.record_visit("https://admin:supersecret@myrouter.local/settings", "Router").unwrap();
        // Blocked scheme
        storage.record_visit("javascript:alert(1)", "XSS").unwrap();

        let history = storage.get_recent_history(10);
        let urls: Vec<&str> = history.iter().map(|h| h.url.as_str()).collect();

        assert!(urls.contains(&"https://duckduckgo.com"));
        assert!(urls.contains(&"https://myrouter.local/settings"));
        assert!(!urls.iter().any(|u| u.contains("supersecret")));
        assert!(!urls.iter().any(|u| u.starts_with("javascript:")));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_history_search() {
        let (storage, path) = temp_test_storage();

        storage.record_visit("https://github.com/rust-lang/rust", "Rust Repository").unwrap();
        storage.record_visit("https://crates.io", "Rust Package Registry").unwrap();
        storage.record_visit("https://news.ycombinator.com", "Hacker News").unwrap();

        let results = storage.search("rust", 5);
        assert_eq!(results.len(), 2);

        let results_hn = storage.search("hacker", 5);
        assert_eq!(results_hn.len(), 1);
        assert_eq!(results_hn[0].url, "https://news.ycombinator.com");

        // Bookmarks should be integrated and prioritized with star icon
        storage.toggle_bookmark("https://doc.rust-lang.org", "Rust Documentation").unwrap();
        let results_with_bm = storage.search("rust", 5);
        assert_eq!(results_with_bm.len(), 3);
        assert_eq!(results_with_bm[0].url, "https://doc.rust-lang.org");
        assert!(results_with_bm[0].title.starts_with("★ "));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_bookmarks_toggle() {
        let (storage, path) = temp_test_storage();

        assert!(!storage.is_bookmarked("https://rust-lang.org"));
        let toggled_on = storage.toggle_bookmark("https://rust-lang.org", "Rust").unwrap();
        assert!(toggled_on);
        assert!(storage.is_bookmarked("https://rust-lang.org"));

        let bookmarks = storage.get_bookmarks();
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].url, "https://rust-lang.org");

        let toggled_off = storage.toggle_bookmark("https://rust-lang.org", "Rust").unwrap();
        assert!(!toggled_off);
        assert!(!storage.is_bookmarked("https://rust-lang.org"));

        // about:blank is rejected
        let about_bm = storage.toggle_bookmark("about:blank", "").unwrap();
        assert!(!about_bm);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_session_save_and_load_with_validation() {
        let (storage, path) = temp_test_storage();

        let tabs = vec![
            "https://duckduckgo.com".to_string(),
            "HTTP://EXAMPLE.COM".to_string(),
            "about:blank".to_string(),
            "data:text/html,bad".to_string(), // Invalid scheme - should be pruned on load
        ];

        // Intentionally pass out-of-bounds active index (e.g. 10) to verify safe clamping
        storage.save_session(&tabs, 10).unwrap();

        let loaded = storage.load_session().expect("Expected valid session");
        assert_eq!(loaded.tabs.len(), 3);
        assert!(loaded.tabs.contains(&"https://duckduckgo.com".to_string()));
        assert!(loaded.tabs.contains(&"HTTP://EXAMPLE.COM".to_string()));
        assert!(loaded.tabs.contains(&"about:blank".to_string()));
        assert!(!loaded.tabs.iter().any(|t| t.starts_with("data:")));
        assert_eq!(loaded.active_index, 2); // Clamped to valid index

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_sanitize_url_credentials_rfc3986() {
        // Strip credentials from authority
        assert_eq!(
            sanitize_url_credentials("https://admin:supersecret@myrouter.local/settings"),
            "https://myrouter.local/settings"
        );
        assert_eq!(
            sanitize_url_credentials("http://user:pass@127.0.0.1:8080/api"),
            "http://127.0.0.1:8080/api"
        );
        assert_eq!(
            sanitize_url_credentials("https://user:pass@example.com"),
            "https://example.com"
        );

        // Crucial bug prevention: queries and paths containing '@' MUST NOT be stripped
        assert_eq!(
            sanitize_url_credentials("https://duckduckgo.com/?q=user@gmail.com"),
            "https://duckduckgo.com/?q=user@gmail.com"
        );
        assert_eq!(
            sanitize_url_credentials("https://twitter.com/search?q=@rustlang"),
            "https://twitter.com/search?q=@rustlang"
        );
        assert_eq!(
            sanitize_url_credentials("https://example.com/tags/@important/page"),
            "https://example.com/tags/@important/page"
        );

        // Standard URLs without credentials
        assert_eq!(
            sanitize_url_credentials("https://crates.io/crates/wry"),
            "https://crates.io/crates/wry"
        );
    }

    #[test]
    fn test_record_visit_debounce_prevents_duplicate_increments() {
        let (storage, path) = temp_test_storage();

        // 1. Initial page load (UrlChanged)
        storage.record_visit("https://example.org", "Loading...").unwrap();
        let history1 = storage.get_recent_history(5);
        assert_eq!(history1.len(), 1);
        assert_eq!(history1[0].visit_count, 1);
        assert_eq!(history1[0].title, "Loading...");

        // 2. Immediate title change event (TitleChanged within seconds)
        storage.record_visit("https://example.org", "Example Domain").unwrap();
        let history2 = storage.get_recent_history(5);
        assert_eq!(history2.len(), 1);
        // Visit count must NOT be incremented to 2 by DOM title changes
        assert_eq!(history2[0].visit_count, 1);
        // But title must be updated
        assert_eq!(history2[0].title, "Example Domain");

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_session_truncates_exceeding_max_tabs() {
        let (storage, path) = temp_test_storage();

        let mut many_tabs = Vec::new();
        for i in 0..35 {
            many_tabs.push(format!("https://example{}.com", i));
        }

        storage.save_session(&many_tabs, 30).unwrap();
        let loaded = storage.load_session().expect("Expected session");
        assert_eq!(loaded.tabs.len(), MAX_SESSION_TABS);
        assert_eq!(loaded.active_index, MAX_SESSION_TABS - 1);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_delete_bookmark_and_history_item() {
        let (storage, path) = temp_test_storage();

        // 1. Test bookmark insertion and deletion
        storage.toggle_bookmark("https://doc.rust-lang.org", "Rust Docs").unwrap();
        assert!(storage.is_bookmarked("https://doc.rust-lang.org"));
        assert_eq!(storage.get_bookmarks().len(), 1);

        let removed_bm = storage.delete_bookmark("https://doc.rust-lang.org").unwrap();
        assert!(removed_bm);
        assert!(!storage.is_bookmarked("https://doc.rust-lang.org"));
        assert_eq!(storage.get_bookmarks().len(), 0);

        // 2. Test history deletion
        storage.record_visit("https://example.com/page1", "Page 1").unwrap();
        storage.record_visit("https://example.com/page2", "Page 2").unwrap();
        assert_eq!(storage.get_recent_history(10).len(), 2);

        let removed_hist = storage.delete_history_item("https://example.com/page1").unwrap();
        assert!(removed_hist);
        let remaining = storage.get_recent_history(10);
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].url, "https://example.com/page2");

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_settings_persistence() {
        let (storage, path) = temp_test_storage();

        // Default value when key does not exist
        assert!(storage.get_setting_bool("block_short_form", true));
        assert!(!storage.get_setting_bool("block_short_form", false));

        // Save true and verify
        storage.set_setting_bool("block_short_form", true).unwrap();
        assert!(storage.get_setting_bool("block_short_form", false));

        // Save false and verify
        storage.set_setting_bool("block_short_form", false).unwrap();
        assert!(!storage.get_setting_bool("block_short_form", true));

        let _ = std::fs::remove_file(path);
    }
}
