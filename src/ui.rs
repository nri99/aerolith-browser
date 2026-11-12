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
  }

  body {
    background: var(--bg-primary);
    color: var(--text-primary);
    font-family: var(--font);
    height: 84px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid var(--border-color);
  }

  /* Row 1: Tab Strip */
  .tab-strip {
    height: 38px;
    background: #0d0e12;
    display: flex;
    align-items: flex-end;
    padding: 0 8px;
    gap: 4px;
    overflow-x: auto;
    scrollbar-width: none;
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
  .tab-close {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    color: var(--text-muted);
    cursor: pointer;
  }
  .tab-close:hover {
    background: rgba(255, 255, 255, 0.15);
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
  }
  .btn-new-tab:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .strip-spacer {
    flex: 1;
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
    transition: background 0.15s, opacity 0.15s;
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
    opacity: 0.6;
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
  }
  .btn-bookmark.bookmarked {
    color: var(--star-active);
  }
  .btn-bookmark:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  /* Shield Button */
  .shield-pill {
    padding: 0 8px;
    height: 28px;
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
  }
  .shield-pill.inactive {
    color: var(--shield-inactive);
  }
  .shield-pill:hover {
    background: var(--bg-hover);
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
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    z-index: 1000;
    display: none;
    overflow: hidden;
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

  /* Drawer Modal */
  .drawer-overlay {
    position: fixed;
    top: 84px;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.6);
    z-index: 2000;
    display: none;
  }
  .drawer-overlay.visible {
    display: flex;
  }

  .drawer-content {
    width: 360px;
    height: 100%;
    background: var(--bg-primary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
  }
  .drawer-tabs {
    display: flex;
    border-bottom: 1px solid var(--border-color);
  }
  .drawer-tab {
    flex: 1;
    padding: 12px;
    text-align: center;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    cursor: pointer;
  }
  .drawer-tab.active {
    color: var(--accent);
    border-bottom: 2px solid var(--accent);
  }
  .drawer-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }
  .drawer-item {
    padding: 10px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  }
  .drawer-item:hover {
    background: var(--bg-hover);
  }
</style>
</head>
<body>

  <!-- Row 1: Tabs -->
  <div class="tab-strip" id="tabStrip">
    <div class="btn-new-tab" id="btnNewTab" title="New Tab (Ctrl+T)">+</div>
    <div class="strip-spacer"></div>
    <div class="shield-pill" id="shieldPill" title="Adblock Shield">
      <span>🛡️</span>
      <span id="shieldText">SHIELD ON</span>
    </div>
  </div>

  <!-- Row 2: Toolbar -->
  <div class="toolbar">
    <button class="btn-nav" id="btnBack" title="Back" disabled>◀</button>
    <button class="btn-nav" id="btnForward" title="Forward" disabled>▶</button>
    <button class="btn-nav" id="btnReload" title="Reload (F5)">⟳</button>
    <button class="btn-nav" id="btnHome" title="Home">🏠</button>

    <div class="omnibox-container">
      <div class="omnibox-wrapper">
        <span class="omnibox-icon" id="securityIcon">🔒</span>
        <input type="text" class="omnibox-input" id="omniboxInput" placeholder="Search DuckDuckGo or enter URL" spellcheck="false" autocomplete="off" />
        <button class="btn-bookmark" id="btnBookmark" title="Bookmark Tab">★</button>
      </div>

      <div class="suggestions-dropdown" id="suggestionsDropdown"></div>
    </div>

    <!-- Quick Tool Buttons -->
    <button class="btn-nav" id="btnDrawer" title="Bookmarks & History (Library)">📚</button>
    <button class="btn-nav" id="btnReader" title="Reader View (Focus)">📖</button>
    <button class="btn-nav" id="btnDarkMode" title="Universal Dark Mode">🌙</button>
    <button class="btn-nav" id="btnKeyboard" title="Open Touch Keyboard">⌨</button>
  </div>

  <!-- Drawer Modal -->
  <div class="drawer-overlay" id="drawerOverlay">
    <div class="drawer-content">
      <div class="drawer-tabs">
        <div class="drawer-tab active" id="tabBookmarks">Bookmarks</div>
        <div class="drawer-tab" id="tabHistory">History</div>
      </div>
      <div class="drawer-list" id="drawerList"></div>
    </div>
  </div>

<script>
  let tabsState = [];
  let activeTabIndex = 0;
  let isShieldActive = true;
  let isDarkModeActive = false;
  let searchDebounceTimer = null;
  let drawerMode = 'bookmarks'; // 'bookmarks' | 'history'

  function postToRust(action, payload) {
    if (window.ipc) {
      window.ipc.postMessage(JSON.stringify({ action, payload }));
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
  const btnDarkMode = document.getElementById('btnDarkMode');
  const omniboxInput = document.getElementById('omniboxInput');
  const suggestionsDropdown = document.getElementById('suggestionsDropdown');
  const drawerOverlay = document.getElementById('drawerOverlay');
  const tabBookmarks = document.getElementById('tabBookmarks');
  const tabHistory = document.getElementById('tabHistory');
  const drawerList = document.getElementById('drawerList');

  // Navigation handlers
  btnBack.onclick = () => postToRust('back', {});
  btnForward.onclick = () => postToRust('forward', {});
  btnReload.onclick = () => postToRust('reload', {});
  btnHome.onclick = () => postToRust('home', {});
  btnNewTab.onclick = () => postToRust('new_tab', { url: 'https://duckduckgo.com' });
  btnKeyboard.onclick = () => postToRust('launch_keyboard', {});
  btnBookmark.onclick = () => postToRust('toggle_bookmark', {});
  shieldPill.onclick = () => postToRust('toggle_shield', {});
  btnReader.onclick = () => postToRust('toggle_reader', {});
  btnDarkMode.onclick = () => {
    isDarkModeActive = !isDarkModeActive;
    btnDarkMode.style.color = isDarkModeActive ? '#5e6ad2' : 'var(--text-primary)';
    postToRust('toggle_dark_mode', { enabled: isDarkModeActive });
  };

  // Drawer handlers
  btnDrawer.onclick = () => {
    const isVis = drawerOverlay.classList.contains('visible');
    if (isVis) {
      drawerOverlay.classList.remove('visible');
    } else {
      drawerOverlay.classList.add('visible');
      loadDrawerContent();
    }
  };

  drawerOverlay.onclick = (e) => {
    if (e.target === drawerOverlay) {
      drawerOverlay.classList.remove('visible');
    }
  };

  tabBookmarks.onclick = () => {
    drawerMode = 'bookmarks';
    tabBookmarks.classList.add('active');
    tabHistory.classList.remove('active');
    loadDrawerContent();
  };

  tabHistory.onclick = () => {
    drawerMode = 'history';
    tabHistory.classList.add('active');
    tabBookmarks.classList.remove('active');
    loadDrawerContent();
  };

  function loadDrawerContent() {
    if (drawerMode === 'bookmarks') {
      postToRust('get_bookmarks', {});
    } else {
      postToRust('get_history', {});
    }
  }

  // Omnibox Navigation
  let selectedSuggestionIndex = -1;

  omniboxInput.addEventListener('keydown', (e) => {
    const items = suggestionsDropdown.querySelectorAll('.suggestion-item');
    const isVisible = suggestionsDropdown.classList.contains('visible') && items.length > 0;

    if (e.key === 'ArrowDown') {
      if (isVisible) {
        e.preventDefault();
        selectedSuggestionIndex = (selectedSuggestionIndex + 1) % items.length;
        items.forEach((it, i) => it.classList.toggle('selected', i === selectedSuggestionIndex));
        omniboxInput.value = items[selectedSuggestionIndex].dataset.url || '';
      }
    } else if (e.key === 'ArrowUp') {
      if (isVisible) {
        e.preventDefault();
        selectedSuggestionIndex = selectedSuggestionIndex <= 0 ? items.length - 1 : selectedSuggestionIndex - 1;
        items.forEach((it, i) => it.classList.toggle('selected', i === selectedSuggestionIndex));
        omniboxInput.value = items[selectedSuggestionIndex].dataset.url || '';
      }
    } else if (e.key === 'Enter') {
      suggestionsDropdown.classList.remove('visible');
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
      suggestionsDropdown.classList.remove('visible');
      selectedSuggestionIndex = -1;
    }
  });

  // Autocomplete search
  omniboxInput.addEventListener('input', () => {
    clearTimeout(searchDebounceTimer);
    selectedSuggestionIndex = -1;
    const q = omniboxInput.value.trim();
    if (q.length > 0) {
      searchDebounceTimer = setTimeout(() => {
        postToRust('search_query', { query: q });
      }, 50);
    } else {
      suggestionsDropdown.classList.remove('visible');
    }
  });

  document.addEventListener('click', (e) => {
    if (!e.target.closest('.omnibox-container')) {
      suggestionsDropdown.classList.remove('visible');
      selectedSuggestionIndex = -1;
    }
  });

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
      tabEl.appendChild(closeSpan);

      tabEl.onclick = (e) => {
        if (!e.target.classList.contains('tab-close')) {
          postToRust('switch_tab', { index: idx });
        }
      };

      tabEl.onauxclick = (e) => {
        if (e.button === 1) { // Middle click to close tab
          e.preventDefault();
          postToRust('close_tab', { index: idx });
        }
      };

      closeSpan.onclick = (e) => {
        e.stopPropagation();
        postToRust('close_tab', { index: idx });
      };

      tabStrip.insertBefore(tabEl, btnNewTab);
    });
  }

  window.updateAddress = function(url, title, canBack, canFwd, isBookmarked) {
    if (document.activeElement !== omniboxInput) {
      omniboxInput.value = url || '';
    }
    btnBack.disabled = !canBack;
    btnForward.disabled = !canFwd;
    if (isBookmarked) {
      btnBookmark.classList.add('bookmarked');
    } else {
      btnBookmark.classList.remove('bookmarked');
    }
  };

  window.showSearchResults = function(results) {
    suggestionsDropdown.innerHTML = '';
    selectedSuggestionIndex = -1;
    if (!results || results.length === 0 || omniboxInput.value.trim() === '') {
      suggestionsDropdown.classList.remove('visible');
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

      item.onclick = () => onSelectSuggestion(r.url);
      suggestionsDropdown.appendChild(item);
    });

    suggestionsDropdown.classList.add('visible');
  };

  window.showDrawerData = function(items, mode) {
    drawerList.innerHTML = '';
    if (!items || items.length === 0) {
      const empty = document.createElement('div');
      empty.style.cssText = 'padding: 20px; text-align: center; color: var(--text-muted); font-size: 13px;';
      empty.textContent = `No ${mode} found`;
      drawerList.appendChild(empty);
      return;
    }

    items.forEach(item => {
      const el = document.createElement('div');
      el.className = 'drawer-item';

      const titleEl = document.createElement('div');
      titleEl.style.cssText = 'font-size: 12px; font-weight: 500; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;';
      titleEl.textContent = item.title || item.url;

      const urlEl = document.createElement('div');
      urlEl.style.cssText = 'font-size: 11px; color: var(--text-muted); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; margin-top: 3px;';
      urlEl.textContent = item.url;

      el.appendChild(titleEl);
      el.appendChild(urlEl);

      el.onclick = () => onSelectDrawerItem(item.url);
      drawerList.appendChild(el);
    });
  };

  window.onSelectDrawerItem = function(url) {
    drawerOverlay.classList.remove('visible');
    postToRust('navigate', { url });
  };

  window.onSelectSuggestion = function(url) {
    omniboxInput.value = url;
    suggestionsDropdown.classList.remove('visible');
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

  function escapeHtml(s) {
    return (s || '')
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#39;');
  }

  // Global Keyboard Shortcuts
  window.addEventListener('keydown', (e) => {
    // Ctrl+T: New Tab
    if (e.ctrlKey && e.key.toLowerCase() === 't') {
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
      drawerMode = 'history';
      tabHistory.classList.add('active');
      tabBookmarks.classList.remove('active');
      drawerOverlay.classList.toggle('visible');
      if (drawerOverlay.classList.contains('visible')) {
        loadDrawerContent();
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
