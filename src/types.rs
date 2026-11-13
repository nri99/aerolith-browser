use serde::{Deserialize, Serialize};

pub const LOGICAL_CHROME_HEIGHT: f64 = 84.0;
pub const DEFAULT_WIDTH: u32 = 1280;
pub const DEFAULT_HEIGHT: u32 = 853;
pub const HOME_URL: &str = "about:home";
pub const MAX_TABS: usize = 20;

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
    pub is_audio_playing: bool,
    pub is_muted: bool,
    pub blocked_count: u32,
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
    ReaderStateChanged {
        tab_id: usize,
        active: bool,
    },
    AdBlocked {
        tab_id: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum TabState {
    Active,
    Tier1Throttled,
    Tier2Hibernated,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_info_serialization_includes_is_sleeping() {
        let info = TabInfo {
            id: 1,
            title: "Test Tab".to_string(),
            url: "https://example.com".to_string(),
            is_sleeping: true,
            is_audio_playing: false,
            is_muted: false,
            blocked_count: 5,
        };
        let json = serde_json::to_string(&info).expect("Failed to serialize TabInfo");
        assert!(json.contains("\"is_sleeping\":true"));
        assert!(json.contains("\"blocked_count\":5"));

        let deserialized: TabInfo = serde_json::from_str(&json).expect("Failed to deserialize TabInfo");
        assert_eq!(deserialized.id, 1);
        assert_eq!(deserialized.title, "Test Tab");
        assert!(deserialized.is_sleeping);
        assert_eq!(deserialized.blocked_count, 5);
    }

    #[test]
    fn test_tab_info_serialization_audio_and_mute() {
        let info = TabInfo {
            id: 1,
            title: "YouTube Video".to_string(),
            url: "https://www.youtube.com/watch?v=123".to_string(),
            is_sleeping: false,
            is_audio_playing: true,
            is_muted: false,
            blocked_count: 0,
        };
        let json = serde_json::to_string(&info).expect("Failed to serialize TabInfo");
        assert!(json.contains("\"is_audio_playing\":true"));
        assert!(json.contains("\"is_muted\":false"));

        let deserialized: TabInfo = serde_json::from_str(&json).expect("Failed to deserialize TabInfo");
        assert_eq!(deserialized.id, 1);
        assert!(deserialized.is_audio_playing);
        assert!(!deserialized.is_muted);
        assert_eq!(deserialized.blocked_count, 0);
    }

    #[test]
    fn test_tab_state_lifecycle_enum() {
        let s1 = TabState::Active;
        let s2 = TabState::Tier1Throttled;
        let s3 = TabState::Tier2Hibernated;
        assert_ne!(s1, s2);
        assert_ne!(s2, s3);
        assert_eq!(s1, TabState::Active);
    }
}
