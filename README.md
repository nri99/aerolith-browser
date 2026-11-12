# Aerolith Browser

An ultra-lightweight, hardware-optimized web browser written in Rust, engineered specifically for resource-constrained hardware such as the Microsoft Surface Go 2 (Intel Core m3-8100Y, 8 GB RAM).

## Key Architectural Features

### 1. Two-Tier Adaptive Tab Memory Governor
- **Tier 1 (Light Throttling @ 2 min):** Throttles `requestAnimationFrame` loops to 1 FPS and limits background timers, cutting background tab CPU wakeups to 0.
- **Tier 2 (Deep Hibernation @ 10 min or Low RAM):** Entirely drops the underlying WebView instance to reclaim physical memory (~0 MB RAM footprint per sleeping tab) while preserving scroll position and navigation state. Instant tap-to-wake restores the tab seamlessly.
- **Kernel Low-Memory Hook & Grace Period:** Direct Win32 `kernel32` FFI integration via `CreateMemoryResourceNotification` and `GlobalMemoryStatusEx` with zero external crate overhead. Detects kernel-level memory pressure (<1.5 GB available or load ≥ 80%) with a 45-second multitasking grace period before hibernating inactive tabs.
- **Smart Sleep Exemptions:** Background audio playback (`HTMLMediaElement.play`) and dirty form inputs (`<input>`, `<textarea>`) are tracked via IPC and strictly exempt from hibernation.
- **Scroll Offset Restoration:** High-precision scroll position tracking (`window.scrollX`/`window.scrollY`) automatically restores viewport alignment on tab wake.

### 2. Dual-Layer Anti-Tracking & Ad-Blocking Pipeline
- **Layer 1 (Native Rust Navigation Trie):** Intercepts top-level and sub-frame navigations, popup window requests, and background requests in native Rust code, dropping known ad, tracking, telemetry, and malware hosts before socket allocation.
- **Layer 2 (In-Engine JS Interceptor):** Traps `window.fetch`, `XMLHttpRequest.prototype.open`, and `navigator.sendBeacon`, short-circuiting ad and tracking calls with synthetic HTTP 204 responses.
- **Cosmetic Filtering:** Injects scoped element-hiding CSS into document heads on load to eliminate ad placeholders.

### 3. Touch-First Surface Go 2 UX
- Custom lightweight HTML5/CSS3 tab strip and address bar with dynamic High-DPI scaling (84 logical px, dynamically scaled to 126 physical px on Surface Go 2 @ 150% scaling).
- Sleeping tab indicators (`💤`) with dimmed styling and instant tap-to-wake.
- Real-time Shield indicator badge (`🛡️ SHIELD ON` / `OFF`) with one-click toggling.
- Dark mode toggle with dynamic CSS injection across all active tabs.
- Seamless Windows touch keyboard integration (`TabTip.exe`).
- Hardware zoom support (`Ctrl` + `+`, `Ctrl` + `-`, `Ctrl` + `0`) and Fullscreen mode (`F11`).
- Integrated Bookmark & History omnibox search with prioritized bookmark suggestions.

### 4. Resilient Embedded Storage (`redb`)
- Embedded ACID transactional key-value store (`aerolith_storage.redb`) with zero daemon processes.
- RFC 3986 URL authority credential sanitization (strips `user:pass@` while preserving legitimate query params).
- 15-second visit debouncing to prevent disk I/O thrashing on title changes.
- Multi-instance directory failover (`WebView2Data_<pid>`) preventing `0x800700AA` lock collisions.

## Building and Testing

### Prerequisites
- Rust 1.85+ (x86_64-pc-windows-gnu or x86_64-pc-windows-msvc)
- Windows 10/11 with WebView2 Runtime

### Running Unit Tests
```bash
cargo test
```
*Executes the complete 37-test automated unit test suite verifying URL normalization, memory governor states, high-DPI scaling, adblock trie filtering, session serialization, and storage robustness.*

### Compiling Release Binary
```bash
cargo build --release
```
Produces an optimized, stripped Windows GUI application binary (`target/release/aerolith-browser.exe` or `target/x86_64-pc-windows-gnu/release/aerolith-browser.exe`) with no console window overhead (`#![windows_subsystem = "windows"]`).
