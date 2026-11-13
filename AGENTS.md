# Aerolith Browser — Agent Developer Guide & Token Optimization Directives

> **CRITICAL DIRECTIVE FOR ALL AGENTS**: Do NOT read unnecessary files or dump entire directories into context. Follow the targeted module routing below to complete tasks in minimal tokens and turns.

---

## 1. Quick Module Routing (Targeted Reads Only)

**Never read `main.rs` unless modifying global window lifecycle or top-level event routing.** Jump directly to the specific module responsible for your task:

| Task / Feature Target | Exact File to Read & Edit | Approximate Size |
|---|---|---|
| **Add / Edit Toolbar Buttons, CSS, HTML, Icons, UI JS** | [`src/ui.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/ui.rs) | ~40 KB (HTML/CSS/JS template) |
| **Add / Modify Tab State, Lifecycle, Zoom, Mute, Audio** | [`src/tabs.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/tabs.rs) | ~15 KB (Tab struct & TabManager) |
| **Add / Modify URL Normalization, Protocols, Security** | [`src/url_utils.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/url_utils.rs) | ~5 KB (URL parsing, sanitization, encoding) |
| **Add / Modify IPC Events, Message Schemas, Constants** | [`src/types.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/types.rs) | ~3 KB (`UserEvent`, `ChromeMsg`, `TabInfo`) |
| **Modify Chrome Height, Dropdown Expansion, Bounds** | [`src/geometry.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/geometry.rs) | ~3 KB (`physical_chrome_height`, `content_rect`) |
| **Adblock, Tracking Protection, Shield, Short-Form Filters** | [`src/adblock.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/adblock.rs) | ~45 KB (Trie domain filter & CSS rules) |
| **Bookmarks, History, redb Database, Settings** | [`src/storage.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/storage.rs) | ~26 KB (ACID transactional redb storage) |
| **Low-Memory Throttling & Hibernation Thresholds** | [`src/governor.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/governor.rs) | ~4 KB (Kernel memory notification FFI) |
| **Directories, File Paths, OSK Keyboard Launch** | [`src/paths.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/paths.rs) | ~2 KB (LocalAppData paths & failover) |
| **WebView Creation, Gestures, DevTools, Script Injection** | [`src/webview_factory.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/webview_factory.rs) | ~7 KB (wry WebView builders) |
| **Main Window Loop & Event Dispatch Router** | [`src/main.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/main.rs) | ~15 KB (High-level event loop only) |

---

## 2. Inviolable Architecture & Z-Order Rules (DO NOT BREAK)

1. **Child WebViews Must Use `build_as_child(window)`**:
   - In `wry-0.57` on Windows, `WebViewBuilder::build(window)` sets `is_child = false`, which ignores custom bounds and calls `resize_to_parent()`, positioning the WebView at `(0, 0, full_width, full_height)` and covering the Chrome Header!
   - **ALWAYS** use `build_as_child(window)?` and immediately call `wv.set_bounds(bounds)`.
2. **Re-assert Chrome Header Bounds on Every Tab Action**:
   - When creating or activating any tab, call:
     ```rust
     reassert_chrome_bounds(&chrome_wv, &chrome_expansion_mode, current_scale_factor, current_size);
     ```
   - This guarantees `chrome_wv` remains pinned at `y = 0..chrome_h` and is never occluded in the Win32 child HWND Z-order.
3. **Safe Horizontal Tab Strip Panning (No Vertical Scroll)**:
   - In `src/ui.rs`, never use element `scrollIntoView()` on tabs, as it can vertically scroll the viewport. Use `tabStrip.scrollTo({ left: ..., behavior: 'smooth' })`.

---

## 3. Fast Testing & Build Protocol (Bypassing File Locks)

The repository resides inside Google Drive on Windows. Google Drive file streaming can momentarily lock compiled binaries (`LNK1104` / `LNK1201`).

To test instantly with **zero file-lock collisions** and **sub-second runtimes (0.16s)**:
```powershell
$env:CARGO_TARGET_DIR = "$env:TEMP\aerolith_target"
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test --manifest-path aerolith-browser\Cargo.toml
```

To build the optimized release binary:
```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build --release --manifest-path aerolith-browser\Cargo.toml
```

---

## 4. How to Add a Feature in 3 Steps (Minimal Tokens)

### Example: Adding a New Chrome Toolbar Action
1. **Frontend UI** ([`src/ui.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/ui.rs)):
   - Add button HTML to `.toolbar` or `.tab-bar-actions`.
   - Add click handler: `btnMyAction.onclick = () => postToRust('my_action', { foo: 'bar' });`.
2. **IPC & Dispatch** ([`src/main.rs`](file:///c:/Users/noah/My%20Drive%20(nrimre@gmail.com)/antigravity%20projects/aerolith-browser/src/main.rs)):
   - Add `"my_action" => { ... }` in `handle_chrome_msg`.
   - Delegate heavy work to `tab_mgr` or helper module.
3. **Run Unit Tests**:
   - Run the fast test command above. Verify all 51+ unit tests pass.

---

## 5. Privacy & Git Commit Directives (STRICT)

- **Zero PII**: Git author must strictly remain:
  ```text
  nri2 <113144550+nri2@users.noreply.github.com>
  ```
  Never include personal names, local paths containing personal directories, or personal email addresses in commits.
- **Zero Scratch Scripts**: Never create temporary `.py` or `.rs` scripts in the workspace. Add unit tests directly into the relevant module under `#[cfg(test)]`.
- **All Changes Pushed to Remote**: Always conclude completed work by pushing to `origin/master`:
  ```bash
  git -C aerolith-browser push origin master
  ```
