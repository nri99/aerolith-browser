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
        <button class="drawer-close-btn" id="btnDrawerClose" title="Close Drawer (Esc)">✕</button>
      </div>
      <div class="drawer-list" id="drawerList"></div>
      <div class="drawer-footer" id="drawerFooter" style="display: none;">
        <button class="btn-clear-history" id="btnClearHistory">Clear Browsing History</button>
      </div>
    </div>
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
  btnBack.onclick = () => postToRust('back', {});
  btnForward.onclick = () => postToRust('forward', {});
  btnReload.onclick = () => {
    if (isPageLoading) {
      postToRust('stop', {});
    } else {
      postToRust('reload', {});
    }
  };
  btnHome.onclick = () => postToRust('home', {});
  btnNewTab.onclick = () => postToRust('new_tab', { url: 'https://duckduckgo.com' });
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
      if (mode === 'bookmarks') {
        tabBookmarks.classList.add('active');
        tabHistory.classList.remove('active');
        drawerFooter.style.display = 'none';
      } else {
        tabHistory.classList.add('active');
        tabBookmarks.classList.remove('active');
        drawerFooter.style.display = 'flex';
      }
    }
    drawerList.innerHTML = '<div class="drawer-loading">Loading...</div>';
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
    drawerFooter.style.display = 'none';
    drawerList.innerHTML = '<div class="drawer-loading">Loading...</div>';
    loadDrawerContent();
  };

  tabHistory.onclick = () => {
    drawerMode = 'history';
    tabHistory.classList.add('active');
    tabBookmarks.classList.remove('active');
    drawerFooter.style.display = 'flex';
    drawerList.innerHTML = '<div class="drawer-loading">Loading...</div>';
    loadDrawerContent();
  };

  btnClearHistory.onclick = () => {
    postToRust('clear_history', {});
  };

  function loadDrawerContent() {
    if (drawerMode === 'bookmarks') {
      postToRust('get_bookmarks', {});
    } else {
      postToRust('get_history', {});
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
    if (!e.target.closest('.omnibox-container')) {
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
        if (!e.target.closest('.tab-close') && !e.target.closest('.tab-audio-btn')) {
          postToRust('switch_tab', { index: idx });
        }
      };

      tabEl.onkeydown = (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          postToRust('switch_tab', { index: idx });
        }
      };

      tabEl.onauxclick = (e) => {
        if (e.button === 1) { // Middle click to close tab
          e.preventDefault();
          postToRust('close_tab', { index: idx });
        }
      };

      closeSpan.onpointerdown = (e) => e.stopPropagation();
      closeSpan.ontouchstart = (e) => e.stopPropagation();
      closeSpan.onmousedown = (e) => e.stopPropagation();
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

  window.updateAddress = function(url, title, canBack, canFwd, isBookmarked) {
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
    // Escape: Close find bar, drawer, or stop page load if active
    if (e.key === 'Escape') {
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
      if (isPageLoading) {
        e.preventDefault();
        postToRust('stop', {});
        return;
      }
    }

    // Ctrl+Shift+T: Reopen Closed Tab
    if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === 't') {
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
    }
    // Ctrl+-: Zoom out
    else if (e.ctrlKey && (e.key === '-' || e.key === '_')) {
      e.preventDefault();
      postToRust('zoom_out', {});
    }
    // Ctrl+0: Reset zoom
    else if (e.ctrlKey && e.key === '0') {
      e.preventDefault();
      postToRust('zoom_reset', {});
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
      postToRust('new_tab', { url: 'https://duckduckgo.com' });
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
