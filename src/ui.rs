pub fn get_chrome_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no">
<title>Aerolith Chrome</title>
<style>
  :root {
    --bg-primary: #121316;
    --bg-secondary: #1a1c23;
    --bg-hover: #262933;
    --bg-active: #2e323e;
    --border-color: #2a2d39;
    --text-primary: #f0f2f5;
    --text-muted: #8c93a8;
    --accent: #5e6ad2;
    --accent-hover: #717de0;
    --shield-active: #10b981;
    --shield-inactive: #6b7280;
    --star-active: #f59e0b;
    --radius-sm: 6px;
    --radius-md: 10px;
    --radius-lg: 14px;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }

  * {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    user-select: none;
    -webkit-user-select: none;
    touch-action: manipulation;
  }

  /* Accessibility keyboard focus ring */
  button:focus-visible,
  .tab:focus-visible,
  .btn-new-tab:focus-visible,
  .shield-pill:focus-visible,
  .drawer-item:focus-visible,
  .drawer-tab:focus-visible,
  .drawer-close-btn:focus-visible,
  .find-btn:focus-visible,
  .tab-audio-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  html, body {
    margin: 0;
    padding: 0;
    height: 100%;
    width: 100%;
    position: fixed;
    top: 0;
    left: 0;
    overflow: hidden;
    background: transparent;
    color: var(--text-primary);
    font-family: var(--font);
    display: flex;
    flex-direction: column;
  }

  .chrome-header {
    height: 84px;
    width: 100%;
    background: var(--bg-primary);
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid var(--border-color);
    position: relative;
    z-index: 100;
    flex-shrink: 0;
    overflow: hidden;
  }

  /* Row 1: Tab Bar Container */
  .tab-bar-container {
    height: 38px;
    background: #0d0e12;
    display: flex;
    align-items: stretch;
    position: relative;
  }

  .tab-strip {
    flex: 1;
    display: flex;
    align-items: flex-end;
    padding: 0 8px;
    gap: 4px;
    overflow-x: auto;
    scrollbar-width: none;
    touch-action: pan-x;
    -webkit-overflow-scrolling: touch;
  }
  .tab-strip::-webkit-scrollbar { display: none; }

  .tab {
    height: 32px;
    min-width: 130px;
    max-width: 200px;
    flex: 1;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-bottom: none;
    border-radius: var(--radius-sm) var(--radius-sm) 0 0;
    display: flex;
    align-items: center;
    padding: 0 10px;
    gap: 8px;
    font-size: 12px;
    font-weight: 500;
    color: var(--text-muted);
    cursor: pointer;
    position: relative;
    transition: background 0.15s, color 0.15s;
    flex-shrink: 0;
  }
  .tab:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
  .tab.active {
    background: var(--bg-primary);
    color: var(--text-primary);
    border-color: var(--border-color);
    border-top: 2px solid var(--accent);
  }
  .tab.sleeping {
    opacity: 0.65;
    border-style: dashed;
  }
  .tab.sleeping:hover {
    opacity: 0.95;
  }
  .tab-icon {
    font-size: 13px;
    opacity: 0.8;
  }
  .tab-title {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Audio Playing / Muted Tab Button */
  .tab-audio-btn {
    width: 22px;
    height: 22px;
    border-radius: 4px;
    border: none;
    background: transparent;
    color: var(--accent);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.15s, opacity 0.15s;
  }
  .tab-audio-btn.muted {
    color: var(--text-muted);
    opacity: 0.65;
  }
  .tab-audio-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    opacity: 1;
  }

  /* Touch-optimized Close Tab Button */
  .tab-close {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.15s, color 0.15s;
  }
  .tab-close:hover {
    background: rgba(255, 255, 255, 0.18);
    color: #ef4444;
  }

  .btn-new-tab {
    width: 32px;
    height: 30px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 18px;
    margin-bottom: 2px;
    transition: background 0.15s;
    flex-shrink: 0;
  }
  .btn-new-tab:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  /* Sticky Tab Bar Actions (Shield Pill pinned to right) */
  .tab-bar-actions {
    display: flex;
    align-items: center;
    padding: 0 8px;
    background: #0d0e12;
    border-left: 1px solid var(--border-color);
    flex-shrink: 0;
    z-index: 2;
  }

  .shield-pill {
    padding: 0 8px;
    height: 26px;
    border-radius: var(--radius-md);
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    font-weight: 600;
    color: var(--shield-active);
    cursor: pointer;
    transition: background 0.15s;
    white-space: nowrap;
  }
  .shield-pill.inactive {
    color: var(--shield-inactive);
  }
  .shield-pill:hover {
    background: var(--bg-hover);
  }

  /* Row 2: Toolbar & Omnibox */
  .toolbar {
    height: 46px;
    background: var(--bg-primary);
    display: flex;
    align-items: center;
    padding: 0 8px;
    gap: 6px;
  }

  .btn-nav {
    width: 34px;
    height: 34px;
    border-radius: var(--radius-sm);
    border: none;
    background: transparent;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    cursor: pointer;
    transition: background 0.15s, opacity 0.15s, color 0.15s;
    flex-shrink: 0;
  }
  .btn-nav:hover:not(:disabled) {
    background: var(--bg-hover);
  }
  .btn-nav:active:not(:disabled) {
    background: var(--bg-active);
  }
  .btn-nav:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }
  .btn-nav.active {
    color: var(--accent);
    background: var(--bg-hover);
  }
  #btnShortForm.active {
    color: #10b981;
    background: rgba(16, 185, 129, 0.15);
  }
  .btn-nav.loading {
    color: #ef4444;
  }

  /* Omnibox */
  .omnibox-container {
    flex: 1;
    position: relative;
    height: 34px;
  }

  .omnibox-wrapper {
    height: 100%;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    display: flex;
    align-items: center;
    padding: 0 10px;
    gap: 8px;
    transition: border-color 0.2s, box-shadow 0.2s;
  }
  .omnibox-wrapper:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px rgba(94, 106, 210, 0.25);
    background: var(--bg-hover);
  }

  .omnibox-icon {
    font-size: 13px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: default;
  }

  .omnibox-input {
    flex: 1;
    height: 100%;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-family: inherit;
    font-size: 13px;
    font-weight: 450;
  }
  .omnibox-input::placeholder {
    color: var(--text-muted);
  }

  .btn-bookmark {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 15px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 4px;
    transition: color 0.15s, background 0.15s;
  }
  .btn-bookmark.bookmarked {
    color: var(--star-active);
  }
  .btn-bookmark:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  /* Suggestions Dropdown */
  .suggestions-dropdown {
    position: absolute;
    top: 38px;
    left: 0;
    right: 0;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.65);
    z-index: 1000;
    display: none;
    overflow-y: auto;
    max-height: 280px;
  }
  .suggestions-dropdown.visible {
    display: block;
  }

  .suggestion-item {
    padding: 8px 12px;
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
    transition: background 0.15s;
  }
  .suggestion-item:last-child {
    border-bottom: none;
  }
  .suggestion-item:hover, .suggestion-item.selected {
    background: var(--bg-hover);
  }
  .suggestion-icon {
    font-size: 13px;
    opacity: 0.6;
  }
  .suggestion-text {
    flex: 1;
    overflow: hidden;
  }
  .suggestion-title {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .suggestion-url {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* In-Page Search Bar */
  .find-bar {
    position: absolute;
    top: 45px;
    right: 12px;
    height: 34px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    display: none;
    align-items: center;
    padding: 0 6px 0 10px;
    gap: 4px;
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.6);
    z-index: 1500;
  }
  .find-bar.visible {
    display: flex;
  }
  .find-input {
    width: 170px;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-family: inherit;
    font-size: 12px;
  }
  .find-input::placeholder {
    color: var(--text-muted);
  }
  .find-btn {
    width: 24px;
    height: 24px;
    border-radius: 4px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }
  .find-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  /* Drawer Modal */
  .drawer-overlay {
    position: fixed;
    top: 84px;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.65);
    z-index: 2000;
    display: none;
  }
  .drawer-overlay.visible {
    display: flex;
  }

  .drawer-content {
    width: 380px;
    height: 100%;
    background: var(--bg-primary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    box-shadow: 5px 0 25px rgba(0, 0, 0, 0.5);
  }
  .drawer-tabs {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border-color);
    padding-right: 8px;
    background: #0d0e12;
  }
  .drawer-tab {
    flex: 1;
    padding: 12px;
    text-align: center;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
  }
  .drawer-tab.active {
    color: var(--accent);
    border-bottom: 2px solid var(--accent);
  }
  .drawer-close-btn {
    width: 30px;
    height: 30px;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 16px;
    cursor: pointer;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.15s, color 0.15s;
  }
  .drawer-close-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
  .drawer-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }
  .drawer-item {
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
    transition: background 0.15s;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }
  .drawer-item:hover, .drawer-item:focus {
    background: var(--bg-hover);
  }
  .drawer-item-text {
    flex: 1;
    overflow: hidden;
  }
  .drawer-item-title {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .drawer-item-url {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-top: 3px;
  }
  .drawer-item-delete {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    border: none;
    background: transparent;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.15s, color 0.15s;
  }
  .drawer-item-delete:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }
  .drawer-footer {
    padding: 12px;
    border-top: 1px solid var(--border-color);
    display: flex;
    justify-content: flex-end;
    background: var(--bg-secondary);
  }
  .btn-clear-history {
    padding: 8px 14px;
    border-radius: var(--radius-sm);
    border: 1px solid #ef4444;
    background: transparent;
    color: #ef4444;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }
  .btn-clear-history:hover {
    background: #ef4444;
    color: #ffffff;
  }
  .drawer-loading, .drawer-empty {
    padding: 24px;
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
  }

  /* Tab Context Menu */
  .tab-context-menu {
    position: fixed;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.75);
    padding: 4px;
    min-width: 180px;
    z-index: 3000;
    display: none;
  }
  .tab-context-menu-item {
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: background 0.12s;
  }
  .tab-context-menu-item:hover {
    background: var(--bg-hover);
  }
  .tab-context-menu-item.disabled {
    opacity: 0.35;
    cursor: not-allowed;
    pointer-events: none;
  }
  .tab-context-menu-separator {
    height: 1px;
    background: var(--border-color);
    margin: 4px 0;
  }

  /* Zoom Badge in Omnibox */
  .zoom-badge {
    padding: 2px 7px;
    border-radius: var(--radius-sm);
    background: var(--bg-hover);
    border: 1px solid var(--border-color);
    font-size: 11px;
    font-weight: 600;
    color: var(--accent-hover);
    cursor: pointer;
    transition: opacity 0.3s ease, background 0.15s;
    white-space: nowrap;
    user-select: none;
    flex-shrink: 0;
  }
  .zoom-badge:hover {
    background: var(--bg-active);
  }

  /* Settings Panel inside Drawer */
  .drawer-settings-panel {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-bottom: 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }
  .settings-section:last-child {
    border-bottom: none;
  }
  .settings-section-title {
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--accent-hover);
    margin-bottom: 4px;
  }
  .settings-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-muted);
  }
  .settings-select {
    width: 100%;
    height: 36px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    padding: 0 10px;
    font-size: 13px;
    outline: none;
    cursor: pointer;
  }
  .settings-select:focus {
    border-color: var(--accent);
  }
  .settings-checkbox-row {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    cursor: pointer;
    user-select: none;
  }
  .settings-checkbox-row input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--accent);
    cursor: pointer;
  }
  .settings-stat-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 12px;
    color: var(--text-muted);
    padding: 4px 0;
  }
  .settings-stat-val {
    font-weight: 600;
    color: var(--shield-active);
  }
  .btn-hibernate-action {
    margin-top: 8px;
    padding: 10px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border-color);
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
    text-align: center;
  }
  .btn-hibernate-action:hover {
    background: var(--bg-hover);
    border-color: var(--accent);
  }
</style>
</head>
<body>

  <div class="chrome-header">
    <!-- Row 1: Tab Bar Container -->
    <div class="tab-bar-container">
      <div class="tab-strip" id="tabStrip">
        <div class="btn-new-tab" id="btnNewTab" title="New Tab (Ctrl+T)">+</div>
      </div>
      <div class="tab-bar-actions">
        <div class="shield-pill" id="shieldPill" title="Adblock Shield">
          <span>🛡️</span>
          <span id="shieldText">SHIELD ON</span>
        </div>
      </div>
    </div>

    <!-- Row 2: Toolbar -->
    <div class="toolbar">
      <button class="btn-nav" id="btnBack" title="Back (Alt+Left)" disabled>◀</button>
      <button class="btn-nav" id="btnForward" title="Forward (Alt+Right)" disabled>▶</button>
      <button class="btn-nav" id="btnReload" title="Reload (F5)">⟳</button>
      <button class="btn-nav" id="btnHome" title="Home">🏠</button>

      <div class="omnibox-container">
        <div class="omnibox-wrapper">
          <span class="omnibox-icon" id="securityIcon" title="Connection Security">🔒</span>
          <input type="text" class="omnibox-input" id="omniboxInput" placeholder="Search DuckDuckGo or enter URL" spellcheck="false" autocomplete="off" />
          <span id="zoomBadge" class="zoom-badge" style="display:none;" title="Click to reset zoom (100%)">100%</span>
          <button class="btn-bookmark" id="btnBookmark" title="Bookmark Tab (Ctrl+D)">★</button>
        </div>

        <div class="suggestions-dropdown" id="suggestionsDropdown"></div>
      </div>

      <!-- Quick Tool Buttons -->
      <button class="btn-nav" id="btnDrawer" title="Bookmarks & History (Ctrl+H)">📚</button>
      <button class="btn-nav" id="btnReader" title="Reader View (Focus)">📖</button>
      <button class="btn-nav active" id="btnShortForm" title="Focus Shield: Block Short-Form Feeds (YouTube Shorts, Reels, TikTok)">🎯</button>
      <button class="btn-nav" id="btnDarkMode" title="Universal Dark Mode">🌙</button>
      <button class="btn-nav" id="btnKeyboard" title="Open Touch Keyboard">⌨</button>
    </div>

    <!-- In-Page Find Bar -->
    <div class="find-bar" id="findBar">
      <input type="text" class="find-input" id="findInput" placeholder="Find in page..." spellcheck="false" autocomplete="off" />
      <button class="find-btn" id="btnFindPrev" title="Previous (Shift+Enter)">▲</button>
      <button class="find-btn" id="btnFindNext" title="Next (Enter)">▼</button>
      <button class="find-btn" id="btnFindClose" title="Close (Esc)">✕</button>
    </div>
  </div>

  <!-- Drawer Modal (Full-height slide-over) -->
  <div class="drawer-overlay" id="drawerOverlay">
    <div class="drawer-content">
      <div class="drawer-tabs">
        <div class="drawer-tab active" id="tabBookmarks">Bookmarks</div>
        <div class="drawer-tab" id="tabHistory">History</div>
        <div class="drawer-tab" id="tabSettings">Settings</div>
        <button class="drawer-close-btn" id="btnDrawerClose" title="Close Drawer (Esc)">✕</button>
      </div>
      <div class="drawer-list" id="drawerList"></div>
      <div class="drawer-settings-panel" id="drawerSettingsPanel" style="display: none;">
        <div class="settings-section">
          <label class="settings-label" for="settingsSearchEngine">Default Search Engine</label>
          <select id="settingsSearchEngine" class="settings-select">
            <option value="duckduckgo">DuckDuckGo (Privacy)</option>
            <option value="google">Google Search</option>
            <option value="brave">Brave Search</option>
            <option value="bing">Microsoft Bing</option>
          </select>
        </div>

        <div class="settings-section">
          <label class="settings-checkbox-row">
            <input type="checkbox" id="settingsClearOnExit" />
            <span>Clear Browsing History & Session on Exit</span>
          </label>
        </div>

        <div class="settings-section">
          <div class="settings-section-title">Surface Go 2 Memory Governor</div>
          <div class="settings-stat-row">
            <span>Available RAM:</span>
            <span id="settingsRamFree" class="settings-stat-val">-- MB</span>
          </div>
          <div class="settings-stat-row">
            <span>Memory Pressure:</span>
            <span id="settingsRamLoad" class="settings-stat-val">--%</span>
          </div>
          <button id="btnHibernateAll" class="btn-hibernate-action">💤 Hibernate Background Tabs Now</button>
        </div>
      </div>
      <div class="drawer-footer" id="drawerFooter" style="display: none;">
        <button class="btn-clear-history" id="btnClearHistory">Clear Browsing History</button>
      </div>
    </div>
  </div>

  <!-- Tab Right-Click Context Menu -->
  <div id="tabContextMenu" class="tab-context-menu">
    <div class="tab-context-menu-item" data-action="duplicate">📑 Duplicate Tab</div>
    <div class="tab-context-menu-item" data-action="reload">⟳ Reload Tab</div>
    <div class="tab-context-menu-item" data-action="mute">🔇 Mute / Unmute</div>
    <div class="tab-context-menu-separator"></div>
    <div class="tab-context-menu-item" data-action="close">✕ Close Tab</div>
    <div class="tab-context-menu-item" data-action="close_others">🚫 Close Other Tabs</div>
    <div class="tab-context-menu-item" data-action="close_right">➡️ Close Tabs to Right</div>
    <div class="tab-context-menu-separator"></div>
    <div class="tab-context-menu-item" data-action="reopen">↩️ Reopen Closed Tab</div>
  </div>

<script>
  let tabsState = [];
  let activeTabIndex = 0;
  let isShieldActive = true;
  let isDarkModeActive = false;
  let isReaderActive = false;
  let isPageLoading = false;
  let searchDebounceTimer = null;
  let originalTypedQuery = '';
  let drawerMode = 'bookmarks'; // 'bookmarks' | 'history'
  let currentExpandedMode = 'none'; // 'none' | 'dropdown' | 'full'

  function postToRust(action, payload) {
    if (window.ipc) {
      window.ipc.postMessage(JSON.stringify({ action, payload: payload || {} }));
    }
  }

  function setExpandedMode(mode) {
    if (currentExpandedMode !== mode) {
      currentExpandedMode = mode;
      postToRust('set_chrome_expanded', { mode });
    }
  }

  // DOM elements
  const tabStrip = document.getElementById('tabStrip');
  const btnNewTab = document.getElementById('btnNewTab');
  const shieldPill = document.getElementById('shieldPill');
  const shieldText = document.getElementById('shieldText');
  const btnBack = document.getElementById('btnBack');
  const btnForward = document.getElementById('btnForward');
  const btnReload = document.getElementById('btnReload');
  const btnHome = document.getElementById('btnHome');
  const btnKeyboard = document.getElementById('btnKeyboard');
  const btnBookmark = document.getElementById('btnBookmark');
  const btnDrawer = document.getElementById('btnDrawer');
  const btnReader = document.getElementById('btnReader');
  const btnShortForm = document.getElementById('btnShortForm');
  const btnDarkMode = document.getElementById('btnDarkMode');
  const omniboxInput = document.getElementById('omniboxInput');
  const suggestionsDropdown = document.getElementById('suggestionsDropdown');
  const drawerOverlay = document.getElementById('drawerOverlay');
  const tabBookmarks = document.getElementById('tabBookmarks');
  const tabHistory = document.getElementById('tabHistory');
  const drawerList = document.getElementById('drawerList');
  const drawerFooter = document.getElementById('drawerFooter');
  const btnClearHistory = document.getElementById('btnClearHistory');
  const btnDrawerClose = document.getElementById('btnDrawerClose');
  const securityIcon = document.getElementById('securityIcon');
  const findBar = document.getElementById('findBar');
  const findInput = document.getElementById('findInput');
  const btnFindPrev = document.getElementById('btnFindPrev');
  const btnFindNext = document.getElementById('btnFindNext');
  const btnFindClose = document.getElementById('btnFindClose');
  const tabSettings = document.getElementById('tabSettings');
  const drawerSettingsPanel = document.getElementById('drawerSettingsPanel');
  const settingsSearchEngine = document.getElementById('settingsSearchEngine');
  const settingsClearOnExit = document.getElementById('settingsClearOnExit');
  const settingsRamFree = document.getElementById('settingsRamFree');
  const settingsRamLoad = document.getElementById('settingsRamLoad');
  const btnHibernateAll = document.getElementById('btnHibernateAll');
  const tabContextMenu = document.getElementById('tabContextMenu');
  const zoomBadge = document.getElementById('zoomBadge');

  let currentZoomLevel = 1.0;
  let zoomFadeTimer = null;

  function triggerZoomDisplay(level) {
    if (!zoomBadge) return;
    currentZoomLevel = Math.max(0.3, Math.min(3.0, level));
    const pct = Math.round(currentZoomLevel * 100);
    if (pct === 100) {
      zoomBadge.style.display = 'none';
      return;
    }
    zoomBadge.textContent = pct + '%';
    zoomBadge.style.display = 'inline-block';
    zoomBadge.style.opacity = '1';
    clearTimeout(zoomFadeTimer);
    zoomFadeTimer = setTimeout(() => {
      zoomBadge.style.opacity = '0';
      setTimeout(() => {
        if (zoomBadge.style.opacity === '0') {
          zoomBadge.style.display = 'none';
        }
      }, 300);
    }, 2500);
  }

  if (zoomBadge) {
    zoomBadge.onclick = (e) => {
      e.stopPropagation();
      postToRust('zoom_reset', {});
      triggerZoomDisplay(1.0);
    };
  }

  let contextMenuTabIndex = -1;

  function updateOmniboxPlaceholder(engine) {
    if (!omniboxInput) return;
    const names = {
      'duckduckgo': 'DuckDuckGo',
      'google': 'Google',
      'brave': 'Brave',
      'bing': 'Bing'
    };
    const name = names[engine] || 'DuckDuckGo';
    omniboxInput.placeholder = 'Search ' + name + ' or enter URL';
  }

  function showTabContextMenu(x, y, tabIdx) {
    contextMenuTabIndex = tabIdx;
    tabContextMenu.style.display = 'block';
    if (currentExpandedMode !== 'full') {
      setExpandedMode('dropdown');
    }
    const menuWidth = 190;
    const clampedX = Math.min(x, window.innerWidth - menuWidth);
    tabContextMenu.style.left = clampedX + 'px';
    tabContextMenu.style.top = Math.max(0, y) + 'px';

    const itemCloseOthers = tabContextMenu.querySelector('[data-action="close_others"]');
    if (itemCloseOthers) {
      if (tabsState.length <= 1) {
        itemCloseOthers.classList.add('disabled');
      } else {
        itemCloseOthers.classList.remove('disabled');
      }
    }
    const itemCloseRight = tabContextMenu.querySelector('[data-action="close_right"]');
    if (itemCloseRight) {
      if (tabIdx >= tabsState.length - 1) {
        itemCloseRight.classList.add('disabled');
      } else {
        itemCloseRight.classList.remove('disabled');
      }
    }
  }

  function hideTabContextMenu() {
    if (tabContextMenu && tabContextMenu.style.display !== 'none') {
      tabContextMenu.style.display = 'none';
      contextMenuTabIndex = -1;
      if (currentExpandedMode === 'dropdown') {
        setExpandedMode('none');
      }
    }
  }

  if (tabContextMenu) {
    tabContextMenu.addEventListener('click', (e) => {
      const item = e.target.closest('.tab-context-menu-item');
      if (!item || item.classList.contains('disabled') || contextMenuTabIndex < 0) return;
      const action = item.dataset.action;
      const idx = contextMenuTabIndex;
      hideTabContextMenu();
      if (action === 'duplicate') {
        postToRust('duplicate_tab', { index: idx });
      } else if (action === 'reload') {
        postToRust('reload_tab', { index: idx });
      } else if (action === 'mute') {
        postToRust('toggle_mute', { index: idx });
      } else if (action === 'close') {
        postToRust('close_tab', { index: idx });
      } else if (action === 'close_others') {
        postToRust('close_other_tabs', { index: idx });
      } else if (action === 'close_right') {
        postToRust('close_tabs_to_right', { index: idx });
      } else if (action === 'reopen') {
        postToRust('reopen_closed_tab', {});
      }
    });
  }

  document.addEventListener('pointerdown', (e) => {
    if (!e.target.closest || !e.target.closest('#tabContextMenu')) {
      hideTabContextMenu();
    }
  });

  if (settingsSearchEngine) {
    settingsSearchEngine.onchange = () => {
      postToRust('set_search_engine', { engine: settingsSearchEngine.value });
      updateOmniboxPlaceholder(settingsSearchEngine.value);
    };
  }
  if (settingsClearOnExit) {
    settingsClearOnExit.onchange = () => {
      postToRust('set_clear_on_exit', { enabled: settingsClearOnExit.checked });
    };
  }
  if (btnHibernateAll) {
    btnHibernateAll.onclick = () => {
      postToRust('hibernate_all', {});
      const orig = btnHibernateAll.textContent;
      btnHibernateAll.textContent = '💤 Background Tabs Hibernated!';
      btnHibernateAll.disabled = true;
      setTimeout(() => {
        btnHibernateAll.textContent = orig;
        btnHibernateAll.disabled = false;
      }, 1500);
    };
  }

  window.updateSettingsData = function(engine, clearOnExit, freeMb, loadPct) {
    if (settingsSearchEngine && engine) {
      settingsSearchEngine.value = engine;
      updateOmniboxPlaceholder(engine);
    }
    if (settingsClearOnExit) settingsClearOnExit.checked = !!clearOnExit;
    if (settingsRamFree) settingsRamFree.textContent = (freeMb !== undefined ? freeMb : '--') + ' MB';
    if (settingsRamLoad) settingsRamLoad.textContent = (loadPct !== undefined ? loadPct : '--') + '%';
  };

  let isShortFormBlocked = true;

  btnShortForm.onclick = () => {
    postToRust('toggle_short_form', {});
  };

  window.updateShortFormState = function(enabled) {
    isShortFormBlocked = enabled;
    btnShortForm.classList.toggle('active', enabled);
    btnShortForm.title = enabled
      ? 'Focus Shield Active: Short-form content blocked (Shorts, Reels, TikTok)'
      : 'Focus Shield Inactive: Short-form content allowed';
  };

  function openFindBar() {
    findBar.classList.add('visible');
    findInput.focus();
    findInput.select();
    if (findInput.value.trim()) {
      executeFind(false);
    }
  }

  function closeFindBar() {
    findBar.classList.remove('visible');
  }

  function executeFind(backward) {
    const q = findInput.value.trim();
    if (q) {
      postToRust('find_in_page', { query: q, backward: !!backward });
    }
  }

  btnFindClose.onclick = () => closeFindBar();
  btnFindNext.onclick = () => executeFind(false);
  btnFindPrev.onclick = () => executeFind(true);

  findInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      executeFind(e.shiftKey);
    } else if (e.key === 'Escape') {
      e.preventDefault();
      closeFindBar();
    }
  });

  findInput.addEventListener('input', () => {
    executeFind(false);
  });

  // Mouse wheel horizontal scrolling for Windows desktop and laptop mice
  tabStrip.addEventListener('wheel', (e) => {
    if (e.deltaY !== 0) {
      e.preventDefault();
      tabStrip.scrollLeft += e.deltaY;
    }
  }, { passive: false });

  // Navigation handlers
  btnBack.onclick = () => {
    closeDrawer();
    closeSuggestions();
    postToRust('back', {});
  };
  btnForward.onclick = () => {
    closeDrawer();
    closeSuggestions();
    postToRust('forward', {});
  };
  btnReload.onclick = () => {
    if (isPageLoading) {
      postToRust('stop', {});
    } else {
      postToRust('reload', {});
    }
  };
  btnHome.onclick = () => {
    closeDrawer();
    closeSuggestions();
    postToRust('home', {});
  };
  btnNewTab.onclick = () => {
    closeDrawer();
    closeSuggestions();
    postToRust('new_tab', { url: 'about:home' });
  };
  btnKeyboard.onclick = () => postToRust('launch_keyboard', {});
  btnBookmark.onclick = () => postToRust('toggle_bookmark', {});
  shieldPill.onclick = () => postToRust('toggle_shield', {});
  btnReader.onclick = () => postToRust('toggle_reader', {});
  btnDarkMode.onclick = () => {
    isDarkModeActive = !isDarkModeActive;
    btnDarkMode.classList.toggle('active', isDarkModeActive);
    postToRust('toggle_dark_mode', { enabled: isDarkModeActive });
  };

  // Reader view state synchronization callable by Rust
  window.setReaderState = function(active) {
    isReaderActive = active;
    btnReader.classList.toggle('active', active);
  };

  // Drawer handlers
  function openDrawer(mode) {
    if (mode) {
      drawerMode = mode;
      tabBookmarks.classList.toggle('active', mode === 'bookmarks');
      tabHistory.classList.toggle('active', mode === 'history');
      if (tabSettings) tabSettings.classList.toggle('active', mode === 'settings');

      if (mode === 'settings') {
        drawerList.style.display = 'none';
        if (drawerSettingsPanel) drawerSettingsPanel.style.display = 'flex';
        drawerFooter.style.display = 'none';
      } else {
        drawerList.style.display = 'block';
        if (drawerSettingsPanel) drawerSettingsPanel.style.display = 'none';
        drawerFooter.style.display = (mode === 'history') ? 'flex' : 'none';
        drawerList.innerHTML = '<div class="drawer-loading">Loading...</div>';
      }
    }
    drawerOverlay.classList.add('visible');
    setExpandedMode('full');
    loadDrawerContent();
  }

  function closeDrawer() {
    drawerOverlay.classList.remove('visible');
    setExpandedMode('none');
  }

  btnDrawer.onclick = () => {
    if (drawerOverlay.classList.contains('visible')) {
      closeDrawer();
    } else {
      openDrawer(drawerMode);
    }
  };

  btnDrawerClose.onclick = () => closeDrawer();

  drawerOverlay.onclick = (e) => {
    if (e.target === drawerOverlay) {
      closeDrawer();
    }
  };

  tabBookmarks.onclick = () => {
    drawerMode = 'bookmarks';
    tabBookmarks.classList.add('active');
    tabHistory.classList.remove('active');
    if (tabSettings) tabSettings.classList.remove('active');
    drawerList.style.display = 'block';
    if (drawerSettingsPanel) drawerSettingsPanel.style.display = 'none';
    drawerFooter.style.display = 'none';
    drawerList.innerHTML = '<div class="drawer-loading">Loading...</div>';
    loadDrawerContent();
  };

  tabHistory.onclick = () => {
    drawerMode = 'history';
    tabHistory.classList.add('active');
    tabBookmarks.classList.remove('active');
    if (tabSettings) tabSettings.classList.remove('active');
    drawerList.style.display = 'block';
    if (drawerSettingsPanel) drawerSettingsPanel.style.display = 'none';
    drawerFooter.style.display = 'flex';
    drawerList.innerHTML = '<div class="drawer-loading">Loading...</div>';
    loadDrawerContent();
  };

  if (tabSettings) {
    tabSettings.onclick = () => {
      drawerMode = 'settings';
      tabSettings.classList.add('active');
      tabBookmarks.classList.remove('active');
      tabHistory.classList.remove('active');
      drawerList.style.display = 'none';
      if (drawerSettingsPanel) drawerSettingsPanel.style.display = 'flex';
      drawerFooter.style.display = 'none';
      loadDrawerContent();
    };
  }

  btnClearHistory.onclick = () => {
    postToRust('clear_history', {});
  };

  function loadDrawerContent() {
    if (drawerMode === 'bookmarks') {
      postToRust('get_bookmarks', {});
    } else if (drawerMode === 'history') {
      postToRust('get_history', {});
    } else if (drawerMode === 'settings') {
      postToRust('get_settings', {});
    }
  }

  // Suggestions Dropdown management
  function showDropdown() {
    suggestionsDropdown.classList.add('visible');
    if (currentExpandedMode !== 'full') {
      setExpandedMode('dropdown');
    }
  }

  function closeSuggestions() {
    suggestionsDropdown.classList.remove('visible');
    selectedSuggestionIndex = -1;
    if (currentExpandedMode === 'dropdown') {
      setExpandedMode('none');
    }
  }

  // Omnibox Navigation & Auto-select on focus
  let selectedSuggestionIndex = -1;
  let isOmniboxFocusing = false;

  omniboxInput.addEventListener('focus', () => {
    isOmniboxFocusing = true;
    omniboxInput.select();
  });
  omniboxInput.addEventListener('mouseup', (e) => {
    if (isOmniboxFocusing) {
      isOmniboxFocusing = false;
      e.preventDefault();
    }
  });

  omniboxInput.addEventListener('keydown', (e) => {
    const items = suggestionsDropdown.querySelectorAll('.suggestion-item');
    const isVisible = suggestionsDropdown.classList.contains('visible') && items.length > 0;

    if (e.key === 'ArrowDown') {
      if (isVisible) {
        e.preventDefault();
        selectedSuggestionIndex += 1;
        if (selectedSuggestionIndex >= items.length) {
          selectedSuggestionIndex = -1;
          omniboxInput.value = originalTypedQuery;
          items.forEach(it => it.classList.remove('selected'));
        } else {
          items.forEach((it, i) => it.classList.toggle('selected', i === selectedSuggestionIndex));
          omniboxInput.value = items[selectedSuggestionIndex].dataset.url || '';
        }
      }
    } else if (e.key === 'ArrowUp') {
      if (isVisible) {
        e.preventDefault();
        selectedSuggestionIndex -= 1;
        if (selectedSuggestionIndex < -1) {
          selectedSuggestionIndex = items.length - 1;
          items.forEach((it, i) => it.classList.toggle('selected', i === selectedSuggestionIndex));
          omniboxInput.value = items[selectedSuggestionIndex].dataset.url || '';
        } else if (selectedSuggestionIndex === -1) {
          omniboxInput.value = originalTypedQuery;
          items.forEach(it => it.classList.remove('selected'));
        } else {
          items.forEach((it, i) => it.classList.toggle('selected', i === selectedSuggestionIndex));
          omniboxInput.value = items[selectedSuggestionIndex].dataset.url || '';
        }
      }
    } else if (e.key === 'Enter') {
      closeSuggestions();
      let targetUrl = omniboxInput.value.trim();
      if (isVisible && selectedSuggestionIndex >= 0 && items[selectedSuggestionIndex]) {
        targetUrl = items[selectedSuggestionIndex].dataset.url || targetUrl;
      }
      if (targetUrl) {
        postToRust('navigate', { url: targetUrl });
        omniboxInput.blur();
      }
      selectedSuggestionIndex = -1;
    } else if (e.key === 'Escape') {
      closeSuggestions();
      if (tabsState[activeTabIndex]) {
        omniboxInput.value = tabsState[activeTabIndex].url || '';
      }
      omniboxInput.blur();
    }
  });

  // Autocomplete search with original query retention
  omniboxInput.addEventListener('input', () => {
    clearTimeout(searchDebounceTimer);
    selectedSuggestionIndex = -1;
    const q = omniboxInput.value;
    originalTypedQuery = q;
    if (q.trim().length > 0) {
      searchDebounceTimer = setTimeout(() => {
        postToRust('search_query', { query: q.trim() });
      }, 50);
    } else {
      closeSuggestions();
    }
  });

  document.addEventListener('click', (e) => {
    if (!e.target.closest || !e.target.closest('.omnibox-container')) {
      closeSuggestions();
    }
  });

  // Security icon indicator
  function updateSecurityIcon(url) {
    if (!securityIcon) return;
    const lower = (url || '').trim().toLowerCase();
    if (lower.startsWith('https://')) {
      securityIcon.textContent = '🔒';
      securityIcon.title = 'Secure Connection (HTTPS)';
      securityIcon.style.color = 'var(--shield-active)';
    } else if (lower.startsWith('http://')) {
      securityIcon.textContent = '⚠️';
      securityIcon.title = 'Insecure Connection (HTTP)';
      securityIcon.style.color = '#ef4444';
    } else if (lower.startsWith('file://')) {
      securityIcon.textContent = '📁';
      securityIcon.title = 'Local File';
      securityIcon.style.color = 'var(--text-muted)';
    } else {
      securityIcon.textContent = '🌐';
      securityIcon.title = 'Internal Page';
      securityIcon.style.color = 'var(--text-muted)';
    }
  }

  // Loading state
  window.setLoading = function(loading) {
    isPageLoading = loading;
    if (loading) {
      btnReload.textContent = '✕';
      btnReload.title = 'Stop Loading (Esc)';
      btnReload.classList.add('loading');
    } else {
      btnReload.textContent = '⟳';
      btnReload.title = 'Reload (F5)';
      btnReload.classList.remove('loading');
    }
  };

  // Exposed functions callable by Rust
  window.updateTabs = function(tabs, activeIdx) {
    tabsState = tabs;
    activeTabIndex = activeIdx;
    renderTabs();
  };

  function renderTabs() {
    const oldTabs = tabStrip.querySelectorAll('.tab');
    oldTabs.forEach(t => t.remove());

    tabsState.forEach((tab, idx) => {
      const tabEl = document.createElement('div');
      tabEl.className = 'tab' + (idx === activeTabIndex ? ' active' : '') + (tab.is_sleeping ? ' sleeping' : '');
      tabEl.tabIndex = 0;

      const iconSpan = document.createElement('span');
      iconSpan.className = 'tab-icon';
      iconSpan.textContent = tab.is_sleeping ? '💤' : '🌐';
      if (tab.is_sleeping) {
        iconSpan.title = 'Sleeping to save RAM (tap to wake)';
      }

      const titleSpan = document.createElement('span');
      titleSpan.className = 'tab-title';
      titleSpan.textContent = tab.title || tab.url || 'New Tab';

      const closeSpan = document.createElement('span');
      closeSpan.className = 'tab-close';
      closeSpan.title = 'Close Tab';
      closeSpan.textContent = '✕';

      tabEl.appendChild(iconSpan);
      tabEl.appendChild(titleSpan);

      if (tab.is_audio_playing || tab.is_muted) {
        const audioBtn = document.createElement('button');
        audioBtn.className = 'tab-audio-btn' + (tab.is_muted ? ' muted' : '');
        audioBtn.title = tab.is_muted ? 'Unmute Tab' : 'Mute Tab';
        audioBtn.textContent = tab.is_muted ? '🔇' : '🔊';
        audioBtn.onclick = (e) => {
          e.stopPropagation();
          postToRust('toggle_mute', { index: idx });
        };
        audioBtn.onpointerdown = (e) => e.stopPropagation();
        audioBtn.ontouchstart = (e) => e.stopPropagation();
        audioBtn.onmousedown = (e) => e.stopPropagation();
        tabEl.appendChild(audioBtn);
      }

      tabEl.appendChild(closeSpan);

      tabEl.onclick = (e) => {
        closeDrawer();
        closeSuggestions();
        if ((!e.target.closest || !e.target.closest('.tab-close')) && (!e.target.closest || !e.target.closest('.tab-audio-btn'))) {
          postToRust('switch_tab', { index: idx });
        }
      };

      tabEl.onkeydown = (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          closeDrawer();
          closeSuggestions();
          postToRust('switch_tab', { index: idx });
        }
      };

      tabEl.onauxclick = (e) => {
        if (e.button === 1) { // Middle click to close tab
          e.preventDefault();
          postToRust('close_tab', { index: idx });
        }
      };

      tabEl.oncontextmenu = (e) => {
        e.preventDefault();
        e.stopPropagation();
        showTabContextMenu(e.clientX, e.clientY, idx);
      };

      closeSpan.onpointerdown = (e) => e.stopPropagation();
      closeSpan.ontouchstart = (e) => e.stopPropagation();
      closeSpan.onmousedown = (e) => e.stopPropagation();
      closeSpan.onauxclick = (e) => e.stopPropagation();
      closeSpan.onclick = (e) => {
        e.stopPropagation();
        postToRust('close_tab', { index: idx });
      };

      tabStrip.insertBefore(tabEl, btnNewTab);
    });

    // Auto-scroll active tab into view horizontally inside tabStrip without vertical scrolling
    const activeTabEl = tabStrip.querySelector('.tab.active');
    if (activeTabEl) {
      const tabLeft = activeTabEl.offsetLeft;
      const tabRight = tabLeft + activeTabEl.offsetWidth;
      const stripLeft = tabStrip.scrollLeft;
      const stripRight = stripLeft + tabStrip.clientWidth;

      if (tabLeft < stripLeft) {
        tabStrip.scrollTo({ left: tabLeft, behavior: 'smooth' });
      } else if (tabRight > stripRight) {
        tabStrip.scrollTo({ left: tabRight - tabStrip.clientWidth, behavior: 'smooth' });
      }
    }
    window.scrollTo(0, 0);
    document.documentElement.scrollTop = 0;
    document.body.scrollTop = 0;
  }

  window.updateAddress = function(url, title, canBack, canFwd, isBookmarked, blockedCount) {
    if (document.activeElement !== omniboxInput) {
      omniboxInput.value = url || '';
    }
    updateSecurityIcon(url);
    btnBack.disabled = !canBack;
    btnForward.disabled = !canFwd;
    if (isBookmarked) {
      btnBookmark.classList.add('bookmarked');
    } else {
      btnBookmark.classList.remove('bookmarked');
    }
    if (isShieldActive) {
      if (blockedCount && blockedCount > 0) {
        shieldText.textContent = blockedCount + ' BLOCKED';
      } else {
        shieldText.textContent = 'SHIELD ON';
      }
    }
  };

  window.showSearchResults = function(results, query) {
    // Prevent race conditions where stale query responses overwrite current text
    if (query !== undefined && query !== omniboxInput.value.trim()) {
      return;
    }

    suggestionsDropdown.innerHTML = '';
    selectedSuggestionIndex = -1;
    if (!results || results.length === 0 || omniboxInput.value.trim() === '') {
      closeSuggestions();
      return;
    }

    results.forEach((r, idx) => {
      const item = document.createElement('div');
      item.className = 'suggestion-item';
      item.dataset.index = idx;
      item.dataset.url = r.url;

      const icon = document.createElement('span');
      icon.className = 'suggestion-icon';
      icon.textContent = '🔍';

      const textWrap = document.createElement('div');
      textWrap.className = 'suggestion-text';

      const titleEl = document.createElement('div');
      titleEl.className = 'suggestion-title';
      titleEl.textContent = r.title || r.url;

      const urlEl = document.createElement('div');
      urlEl.className = 'suggestion-url';
      urlEl.textContent = r.url;

      textWrap.appendChild(titleEl);
      textWrap.appendChild(urlEl);

      item.appendChild(icon);
      item.appendChild(textWrap);

      item.onclick = (e) => {
        if (e.ctrlKey || e.metaKey) {
          closeSuggestions();
          postToRust('new_tab', { url: r.url });
        } else {
          onSelectSuggestion(r.url);
        }
      };

      item.onauxclick = (e) => {
        if (e.button === 1) { // Middle click
          e.preventDefault();
          closeSuggestions();
          postToRust('new_tab', { url: r.url });
        }
      };

      suggestionsDropdown.appendChild(item);
    });

    showDropdown();
  };

  window.showDrawerData = function(items, mode) {
    drawerList.innerHTML = '';
    if (!items || items.length === 0) {
      const empty = document.createElement('div');
      empty.className = 'drawer-empty';
      empty.textContent = `No ${mode} found`;
      drawerList.appendChild(empty);
      return;
    }

    items.forEach(item => {
      const el = document.createElement('div');
      el.className = 'drawer-item';
      el.tabIndex = 0;

      const textWrap = document.createElement('div');
      textWrap.className = 'drawer-item-text';

      const titleEl = document.createElement('div');
      titleEl.className = 'drawer-item-title';
      titleEl.textContent = item.title || item.url;

      const urlEl = document.createElement('div');
      urlEl.className = 'drawer-item-url';
      urlEl.textContent = item.url;

      textWrap.appendChild(titleEl);
      textWrap.appendChild(urlEl);
      el.appendChild(textWrap);

      const deleteBtn = document.createElement('button');
      deleteBtn.className = 'drawer-item-delete';
      deleteBtn.title = `Delete from ${mode} (Del)`;
      deleteBtn.textContent = '✕';
      deleteBtn.onclick = (e) => {
        e.stopPropagation();
        if (mode === 'bookmarks') {
          postToRust('delete_bookmark', { url: item.url });
        } else {
          postToRust('delete_history_entry', { url: item.url });
        }
      };
      el.appendChild(deleteBtn);

      el.onclick = (e) => {
        if (e.target === deleteBtn) return;
        if (e.ctrlKey || e.metaKey) {
          postToRust('new_tab', { url: item.url });
        } else {
          onSelectDrawerItem(item.url);
        }
      };

      el.onauxclick = (e) => {
        if (e.button === 1) { // Middle click
          e.preventDefault();
          postToRust('new_tab', { url: item.url });
        }
      };

      el.onkeydown = (e) => {
        if (e.key === 'Enter') {
          if (e.ctrlKey || e.metaKey) {
            postToRust('new_tab', { url: item.url });
          } else {
            onSelectDrawerItem(item.url);
          }
        } else if (e.key === 'Delete') {
          if (mode === 'bookmarks') {
            postToRust('delete_bookmark', { url: item.url });
          } else {
            postToRust('delete_history_entry', { url: item.url });
          }
        }
      };

      drawerList.appendChild(el);
    });
  };

  window.onSelectDrawerItem = function(url) {
    closeDrawer();
    postToRust('navigate', { url });
  };

  window.onSelectSuggestion = function(url) {
    omniboxInput.value = url;
    closeSuggestions();
    postToRust('navigate', { url });
    omniboxInput.blur();
  };

  window.updateShieldState = function(enabled) {
    isShieldActive = enabled;
    if (enabled) {
      shieldPill.classList.remove('inactive');
      shieldText.textContent = 'SHIELD ON';
    } else {
      shieldPill.classList.add('inactive');
      shieldText.textContent = 'SHIELD OFF';
    }
  };

  // Global Keyboard Shortcuts
  window.addEventListener('keydown', (e) => {
    // Escape: Close find bar, drawer, suggestions dropdown, or stop page load if active
    if (e.key === 'Escape') {
      if (tabContextMenu && tabContextMenu.style.display !== 'none') {
        e.preventDefault();
        hideTabContextMenu();
        return;
      }
      if (findBar && findBar.classList.contains('visible')) {
        e.preventDefault();
        closeFindBar();
        return;
      }
      if (drawerOverlay.classList.contains('visible')) {
        e.preventDefault();
        closeDrawer();
        return;
      }
      if (suggestionsDropdown && suggestionsDropdown.classList.contains('visible')) {
        e.preventDefault();
        closeSuggestions();
        return;
      }
      if (isPageLoading) {
        e.preventDefault();
        postToRust('stop', {});
        return;
      }
    }

    // Ctrl+P: Print & PDF Export
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'p') {
      e.preventDefault();
      postToRust('print', {});
    }
    // Ctrl+Shift+T: Reopen Closed Tab
    else if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === 't') {
      e.preventDefault();
      postToRust('reopen_closed_tab', {});
    }
    // Ctrl+F: Find in Page
    else if (e.ctrlKey && e.key.toLowerCase() === 'f') {
      e.preventDefault();
      openFindBar();
    }
    // Ctrl+= or Ctrl++: Zoom in
    else if (e.ctrlKey && (e.key === '=' || e.key === '+')) {
      e.preventDefault();
      postToRust('zoom_in', {});
      triggerZoomDisplay(currentZoomLevel + 0.1);
    }
    // Ctrl+-: Zoom out
    else if (e.ctrlKey && (e.key === '-' || e.key === '_')) {
      e.preventDefault();
      postToRust('zoom_out', {});
      triggerZoomDisplay(currentZoomLevel - 0.1);
    }
    // Ctrl+0: Reset zoom
    else if (e.ctrlKey && e.key === '0') {
      e.preventDefault();
      postToRust('zoom_reset', {});
      triggerZoomDisplay(1.0);
    }
    // Ctrl+Tab: Next tab
    else if (e.ctrlKey && !e.shiftKey && e.key === 'Tab') {
      e.preventDefault();
      if (tabsState.length > 1) {
        const nextIdx = (activeTabIndex + 1) % tabsState.length;
        postToRust('switch_tab', { index: nextIdx });
      }
    }
    // Ctrl+Shift+Tab: Previous tab
    else if (e.ctrlKey && e.shiftKey && e.key === 'Tab') {
      e.preventDefault();
      if (tabsState.length > 1) {
        const prevIdx = (activeTabIndex + tabsState.length - 1) % tabsState.length;
        postToRust('switch_tab', { index: prevIdx });
      }
    }
    // Ctrl+1 through Ctrl+8: Direct tab select
    else if (e.ctrlKey && e.key >= '1' && e.key <= '8') {
      e.preventDefault();
      const targetIdx = parseInt(e.key, 10) - 1;
      if (targetIdx < tabsState.length) {
        postToRust('switch_tab', { index: targetIdx });
      }
    }
    // Ctrl+9: Select last tab
    else if (e.ctrlKey && e.key === '9') {
      e.preventDefault();
      if (tabsState.length > 0) {
        postToRust('switch_tab', { index: tabsState.length - 1 });
      }
    }
    // Ctrl+T: New Tab
    else if (e.ctrlKey && !e.shiftKey && e.key.toLowerCase() === 't') {
      e.preventDefault();
      postToRust('new_tab', { url: 'about:home' });
    }
    // Ctrl+W: Close Active Tab
    else if (e.ctrlKey && e.key.toLowerCase() === 'w') {
      e.preventDefault();
      postToRust('close_tab', { index: activeTabIndex });
    }
    // Ctrl+R or F5: Reload
    else if ((e.ctrlKey && e.key.toLowerCase() === 'r') || e.key === 'F5') {
      e.preventDefault();
      postToRust('reload', {});
    }
    // Ctrl+L or F6: Focus Omnibox
    else if ((e.ctrlKey && e.key.toLowerCase() === 'l') || e.key === 'F6') {
      e.preventDefault();
      omniboxInput.focus();
      omniboxInput.select();
    }
    // Ctrl+D: Toggle Bookmark
    else if (e.ctrlKey && e.key.toLowerCase() === 'd') {
      e.preventDefault();
      postToRust('toggle_bookmark', {});
    }
    // Ctrl+H: Toggle History Drawer
    else if (e.ctrlKey && e.key.toLowerCase() === 'h') {
      e.preventDefault();
      if (drawerOverlay.classList.contains('visible') && drawerMode === 'history') {
        closeDrawer();
      } else {
        openDrawer('history');
      }
    }
    // Alt+Left: Back
    else if (e.altKey && e.key === 'ArrowLeft') {
      e.preventDefault();
      postToRust('back', {});
    }
    // Alt+Right: Forward
    else if (e.altKey && e.key === 'ArrowRight') {
      e.preventDefault();
      postToRust('forward', {});
    }
    // F11: Toggle Fullscreen
    else if (e.key === 'F11') {
      e.preventDefault();
      postToRust('toggle_fullscreen', {});
    }
  });

  // Notify Rust that chrome UI is mounted
  window.addEventListener('DOMContentLoaded', () => {
    postToRust('chrome_ready', {});
  });
</script>
</body>
</html>
"#
}

pub fn get_speed_dial_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>New Tab</title>
<style>
  :root {
    --bg-primary: #121316;
    --bg-card: #1a1c23;
    --bg-hover: #262933;
    --border-color: #2a2d39;
    --text-primary: #f0f2f5;
    --text-muted: #8c93a8;
    --accent: #5e6ad2;
    --radius-md: 12px;
    --radius-lg: 16px;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    background-color: var(--bg-primary);
    color: var(--text-primary);
    font-family: var(--font);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 24px;
    user-select: none;
    -webkit-user-select: none;
  }
  .home-container {
    width: 100%;
    max-width: 680px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 32px;
  }
  .brand {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }
  .brand-logo {
    font-size: 48px;
    line-height: 1;
    filter: drop-shadow(0 4px 12px rgba(94, 106, 210, 0.4));
  }
  .brand-title {
    font-size: 26px;
    font-weight: 700;
    letter-spacing: -0.5px;
    color: var(--text-primary);
  }
  .brand-subtitle {
    font-size: 13px;
    color: var(--text-muted);
    font-weight: 500;
  }
  .search-wrapper {
    width: 100%;
    position: relative;
  }
  .search-input {
    width: 100%;
    height: 50px;
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: 0 20px 0 48px;
    font-size: 15px;
    color: var(--text-primary);
    outline: none;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.25);
    transition: border-color 0.2s, box-shadow 0.2s;
  }
  .search-input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px rgba(94, 106, 210, 0.25);
  }
  .search-icon {
    position: absolute;
    left: 18px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 16px;
    color: var(--text-muted);
    pointer-events: none;
  }
  .tiles-grid {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
  }
  @media (max-width: 500px) {
    .tiles-grid { grid-template-columns: repeat(2, 1fr); }
  }
  .tile {
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    padding: 16px 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    text-decoration: none;
    color: var(--text-primary);
    cursor: pointer;
    transition: transform 0.15s, background 0.15s, border-color 0.15s;
  }
  .tile:hover {
    background: var(--bg-hover);
    border-color: var(--accent);
    transform: translateY(-2px);
  }
  .tile-icon {
    font-size: 24px;
    width: 44px;
    height: 44px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .tile-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    text-align: center;
  }
  .telemetry-bar {
    width: 100%;
    background: rgba(26, 28, 35, 0.7);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    padding: 12px 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
    color: var(--text-muted);
  }
  .telemetry-status {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .telemetry-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 8px #10b981;
  }
  .btn-hibernate {
    background: var(--bg-hover);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }
  .btn-hibernate:hover {
    background: var(--accent);
    color: #fff;
  }
</style>
</head>
<body>
  <div class="home-container">
    <div class="brand">
      <div class="brand-logo">⚡</div>
      <h1 class="brand-title">Aerolith</h1>
      <p class="brand-subtitle">Fast, Lightweight & Shielded for Surface Go 2</p>
    </div>

    <div class="search-wrapper">
      <span class="search-icon">🔍</span>
      <input type="text" class="search-input" id="homeSearchInput" placeholder="Search the web or type a URL..." autofocus autocomplete="off" />
    </div>

    <div class="tiles-grid">
      <div class="tile" data-url="https://duckduckgo.com">
        <div class="tile-icon">🦆</div>
        <div class="tile-label">DuckDuckGo</div>
      </div>
      <div class="tile" data-url="https://www.youtube.com">
        <div class="tile-icon">▶️</div>
        <div class="tile-label">YouTube</div>
      </div>
      <div class="tile" data-url="https://github.com">
        <div class="tile-icon">🐙</div>
        <div class="tile-label">GitHub</div>
      </div>
      <div class="tile" data-url="https://www.wikipedia.org">
        <div class="tile-icon">📚</div>
        <div class="tile-label">Wikipedia</div>
      </div>
      <div class="tile" data-url="https://www.reddit.com">
        <div class="tile-icon">💬</div>
        <div class="tile-label">Reddit</div>
      </div>
      <div class="tile" data-url="https://news.ycombinator.com">
        <div class="tile-icon">🔶</div>
        <div class="tile-label">Hacker News</div>
      </div>
      <div class="tile" data-url="https://www.rust-lang.org">
        <div class="tile-icon">🦀</div>
        <div class="tile-label">Rust Lang</div>
      </div>
      <div class="tile" data-url="https://docs.rs">
        <div class="tile-icon">📖</div>
        <div class="tile-label">Docs.rs</div>
      </div>
    </div>

    <div class="telemetry-bar">
      <div class="telemetry-status">
        <div class="telemetry-dot"></div>
        <span>Surface Go 2 Memory Governor: <strong>Active</strong></span>
      </div>
      <button class="btn-hibernate" id="btnHomeHibernate">Hibernate Background Tabs</button>
    </div>
  </div>

  <script>
    function nav(target) {
      if (window.ipc) {
        window.ipc.postMessage(JSON.stringify({ action: 'home_navigate', payload: { query: target } }));
      } else if (window.chrome && window.chrome.webview) {
        window.chrome.webview.postMessage(JSON.stringify({ action: 'home_navigate', payload: { query: target } }));
      } else {
        window.location.href = target;
      }
    }

    const input = document.getElementById('homeSearchInput');
    input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        const val = input.value.trim();
        if (val) nav(val);
      }
    });

    document.querySelectorAll('.tile').forEach(tile => {
      tile.addEventListener('click', () => {
        const url = tile.dataset.url;
        if (url) nav(url);
      });
    });

    const btnHomeHibernate = document.getElementById('btnHomeHibernate');
    if (btnHomeHibernate) {
      btnHomeHibernate.addEventListener('click', () => {
        const msg = JSON.stringify({ action: 'hibernate_all', payload: {} });
        if (window.ipc) window.ipc.postMessage(msg);
        else if (window.chrome && window.chrome.webview) window.chrome.webview.postMessage(msg);
        const orig = btnHomeHibernate.textContent;
        btnHomeHibernate.textContent = 'Tabs Hibernated! 💤';
        btnHomeHibernate.disabled = true;
        setTimeout(() => {
          btnHomeHibernate.textContent = orig;
          btnHomeHibernate.disabled = false;
        }, 1500);
      });
    }
  </script>
</body>
</html>
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_dial_html_contains_critical_elements() {
        let html = get_speed_dial_html();
        assert!(html.contains("id=\"homeSearchInput\""), "Speed dial search input must exist");
        assert!(html.contains("id=\"btnHomeHibernate\""), "Speed dial hibernate button must exist");
        assert!(html.contains("DuckDuckGo"), "Speed dial must have DuckDuckGo tile");
        assert!(html.contains("GitHub"), "Speed dial must have GitHub tile");
    }

    #[test]
    fn test_chrome_html_contains_tab_context_menu_and_settings() {
        let html = get_chrome_html();
        assert!(html.contains("id=\"tabContextMenu\""), "Tab context menu element must exist");
        assert!(html.contains("data-action=\"duplicate\""), "Duplicate action in context menu must exist");
        assert!(html.contains("data-action=\"close_others\""), "Close others action in context menu must exist");
        assert!(html.contains(".tab-context-menu-item.disabled"), "Disabled state for tab context menu items must exist");
        assert!(html.contains("function updateOmniboxPlaceholder"), "Dynamic omnibox placeholder function must exist");
        assert!(html.contains("id=\"zoomBadge\""), "Zoom badge in omnibox must exist");
        assert!(html.contains("id=\"tabSettings\""), "Settings drawer tab must exist");
        assert!(html.contains("id=\"settingsSearchEngine\""), "Search engine selector must exist");
        assert!(html.contains("id=\"btnHibernateAll\""), "Hibernate all button in drawer must exist");
    }

    #[test]
    fn test_chrome_html_contains_critical_ui_elements() {
        let html = get_chrome_html();
        assert!(html.contains("id=\"tabStrip\""), "Tab strip container must exist");
        assert!(html.contains("id=\"btnNewTab\""), "New tab button must exist");
        assert!(html.contains("id=\"omniboxInput\""), "Omnibox input must exist");
        assert!(html.contains("id=\"suggestionsDropdown\""), "Suggestions dropdown must exist");
        assert!(html.contains("id=\"drawerOverlay\""), "Drawer overlay must exist");
        assert!(html.contains("id=\"findBar\""), "Find in page bar must exist");
        assert!(html.contains("id=\"shieldPill\""), "Adblock shield pill must exist");
    }

    #[test]
    fn test_chrome_html_closest_safety_guards() {
        let html = get_chrome_html();
        // Regression test: document click must guard against non-element targets where .closest is undefined
        assert!(
            html.contains("!e.target.closest || !e.target.closest('.omnibox-container')"),
            "Document click listener must safely guard e.target.closest"
        );
        assert!(
            html.contains("(!e.target.closest || !e.target.closest('.tab-close'))"),
            "Tab element click listener must safely guard e.target.closest"
        );
    }

    #[test]
    fn test_chrome_html_escape_dismisses_suggestions() {
        let html = get_chrome_html();
        assert!(
            html.contains("suggestionsDropdown && suggestionsDropdown.classList.contains('visible')"),
            "Global Escape key handler must dismiss suggestions dropdown"
        );
    }

    #[test]
    fn test_chrome_html_navigation_dismisses_overlays() {
        let html = get_chrome_html();
        assert!(html.contains("btnNewTab.onclick = () => {\n    closeDrawer();\n    closeSuggestions();"));
        assert!(html.contains("btnHome.onclick = () => {\n    closeDrawer();\n    closeSuggestions();"));
        assert!(html.contains("closeSpan.onauxclick = (e) => e.stopPropagation();"));
    }
}
