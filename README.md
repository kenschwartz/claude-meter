<div align="center">

# ⚡ ZaiMeter

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/klivak)

**Real-time Claude AI usage monitor for Windows and macOS — track your subscription limits from the tray or menu bar**

Ultra-lightweight Rust app that monitors Claude Pro & Max usage caps in real time.
See your 5-hour session, weekly limits, Sonnet & Opus quotas — without opening a browser.

**🦀 Purposefully built in Rust — uses under 10 MB RAM. Less than Notepad.**

[![Build](https://github.com/klivak/zaimeter/actions/workflows/build.yml/badge.svg)](https://github.com/klivak/zaimeter/actions/workflows/build.yml)
[![Audit](https://github.com/klivak/zaimeter/actions/workflows/audit.yml/badge.svg)](https://github.com/klivak/zaimeter/actions/workflows/audit.yml)
[![Release](https://img.shields.io/github/v/release/klivak/zaimeter)](https://github.com/klivak/zaimeter/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Windows](https://img.shields.io/badge/platform-Windows%2010%2F11-0078D6?logo=windows)](https://github.com/klivak/zaimeter/releases)
[![macOS](https://img.shields.io/badge/platform-macOS%2012%2B-000000?logo=apple)](https://github.com/klivak/zaimeter/releases)
[![RAM](https://img.shields.io/badge/RAM-under%2010MB-brightgreen)](#-why-rust)
[![VirusTotal](https://img.shields.io/badge/VirusTotal-scanned-4CAF50?logo=virustotal&logoColor=white)](https://github.com/klivak/zaimeter/releases/latest)

[Download](#-quick-start) · [Features](#-features) · [Usage](#-how-to-use) · [FAQ](#-faq)

<br>

<img src="screenshots/main.png" alt="ZaiMeter dashboard showing Claude AI usage bars and reset timers" width="360"> <img src="screenshots/main2.png" alt="ZaiMeter light theme with usage history chart" width="360">

</div>

---
☕ [Buy me a coffee](https://ko-fi.com/klivak) if this project helped you

---

## 🤔 Why ZaiMeter?

Tired of hitting Claude AI rate limits mid-conversation? ZaiMeter sits quietly in your Windows system tray or macOS menu bar and shows you **exactly** how much of your Anthropic subscription quota remains — 5-hour session utilization, weekly usage caps, Sonnet and Opus limits — all without opening a browser tab or checking the Claude dashboard manually.

## 🦀 Why Rust?

ZaiMeter is **purposefully built in Rust** to be as lightweight as physically possible. While most similar tools use Electron (which bundles an entire Chromium browser) or Python (which needs a runtime), ZaiMeter compiles to native Windows and macOS binaries with no bundled browser runtime.

| App | RAM Usage | Binary Size | Dependencies |
|-----|-----------|-------------|-------------|
| **ZaiMeter (Rust)** | **~2 MB** | **~3 MB** | **None** |
| Windows Notepad | ~10 MB | built-in | — |
| Electron-based tray apps | 80–150 MB | ~80 MB | Chromium |
| Python-based monitors | 25–45 MB | ~15 MB | Python runtime |
| .NET-based monitors | 15–25 MB | ~1 MB | .NET runtime |

<img src="screenshots/task-manager.png" alt="Windows Task Manager showing ZaiMeter using only 1.9 MB RAM" width="540">

**Single portable `.exe` on Windows** and a native **`.app` bundle on macOS** — no Electron, no .NET, no Java, no Python, no Node.js. Download → run → done.

## ⬇ Quick Start

### Step 1: Install Claude Code (one-time)

ZaiMeter reads your Claude credentials automatically. You need [Claude Code](https://claude.ai/download) installed and logged in:

```bash
# Install Claude Code (if not already)
# Download from https://claude.ai/download

# Log in (creates OAuth token that ZaiMeter will use)
claude
```

### Step 2: Download & Run ZaiMeter

#### Windows

1. **Download** [`zaimeter.exe`](https://github.com/klivak/zaimeter/releases/latest) from Releases
2. **Place** it anywhere — Desktop, tools folder, USB drive (it's portable)
3. **Double-click** to run
4. **Look** for the colored circle icon in your system tray (bottom-right near the clock)

That's it! No configuration needed. ZaiMeter auto-detects your plan and starts monitoring.

#### macOS

1. **Download** [`ZaiMeter-macos-arm64.app.zip`](https://github.com/klivak/zaimeter/releases/latest) from Releases
2. **Unzip** it and move `ZaiMeter.app` to `/Applications`
3. **Open** the app; it appears as a native menu bar item
4. Use the menu for **Refresh Now**, **Open Claude Usage**, **Settings/config**, **Import/Export Config**, **Autostart**, and **Open Logs**

The raw `zaimeter-macos-arm64` binary is also published for CLI/agent use, but the `.app.zip` is the recommended macOS download.

### Step 3 (Optional): Enable Auto-Start

Right-click the tray icon → check ✅ **"Start with Windows"**

On macOS, use the menu bar item → **Enable Autostart**. The app uses a LaunchAgent and starts `ZaiMeter.app` from `/Applications` when available.

## ✨ Features

### Claude AI Monitoring (Automatic)

| Metric | Description |
|--------|-------------|
| 5-hour session | Rolling session utilization with countdown timer |
| 7-day weekly | Weekly usage cap with reset timer |
| 7-day Sonnet | Sonnet-specific limit (shown if applicable) |
| 7-day Opus | Opus-specific limit (Max plans only) |
| Plan badge | Color-coded badge (Pro/Max/5X/20X) with automatic detection |
| Future metrics | Any new API fields are auto-displayed |

### ChatGPT / Codex (Optional)

OpenAI does not provide a public API for checking ChatGPT Plus/Pro subscription usage. ZaiMeter includes an optional panel (disabled by default) with a direct link to your ChatGPT usage page. Enable it in Settings if you want quick access.

### System Tray

- **🔢 Dynamic % icon** — shows actual utilization number (e.g. "42") with color-coded background
- **⭕ Icon styles** — choose between Number (default), Ring (circular progress), Bar (vertical fill), or Pie (multi-metric pie chart) in Settings
- **🟢🟡🔴 Color coding** — green (<50%), yellow (50-79%), red (>=80%), gray (no data) with transparent icon backgrounds
- **💬 Rich tooltip** — hover to see all metrics, reset times, and plan info
- **📋 Context menu** — right-click for refresh, export CSV, settings, links
- **📊 Dashboard** — left-click to open the detailed popup
- **⚠ Blink on critical** — tray icon blinks when usage exceeds 90%

<img src="screenshots/hover.png" alt="ZaiMeter system tray tooltip showing Claude usage percentages and reset times" width="280">

### macOS Menu Bar

- **Native `NSStatusItem`** — shows current Claude usage directly in the macOS menu bar
- **Freshness state** — displays whether data is live, refreshing, cached, stale, or blocked by an API error
- **Manual refresh** — `Refresh Now` forces a new poll instead of relying on cached data
- **Quick actions** — open Claude usage, check for updates, open config, import/export config, toggle autostart, and open logs
- **Portable logs** — writes `zaimeter.log` under `~/Library/Application Support/ZaiMeter`
- **Agent status file** — writes `status.json` for the menu bar UI under `~/Library/Application Support/ZaiMeter`

### 📊 Dashboard

- **Dashboard layouts** — three modes: Minimal (single largest metric), Standard (all bars), Detailed (metrics with inline sparkline charts)
- **Gradient progress bars** — full-spectrum green→amber→coral gradient with rate-of-change trend arrows (↑↗→↘↓)
- **Easing animations** — smooth ease-out progress bars with cascading staggered appearance (~60fps)
- **Fade-in animation** — popup appears with accelerating opacity transition
- **Slide animation** — smooth horizontal slide between Dashboard and Settings views
- **24-hour / 7-day / 30-day chart** — usage history with toggleable time ranges, session reset lines, and hover tooltips
- **Clickable plan name** — click the plan name in the header to open claude.ai/settings/usage
- **D2D-rendered UI** — custom-drawn gear icon, close button, and checkboxes using Direct2D primitives
- **Keyboard shortcuts** — ESC to close, F5 to refresh
- **Auto-refresh** — automatically polls when data is older than 60 seconds
- **Acrylic backdrop** — Windows 11 translucent blur effect (falls back gracefully on Win10)
- **Segoe UI Variable** — uses Windows 11's variable font with automatic fallback to Segoe UI
- **Hide Extra Usage** — toggle in Settings to hide the extra_usage metric from the dashboard
- **Notification toggles** — Settings exposes "Show startup notification" (silence the "Running in tray" balloon on launch) and "Show login expiry warning" (silence the `claude login` reminder)
- **Clickable update balloon** — clicking the "Update available" tray balloon opens the GitHub release page in your browser

### 🎨 Themes

- **Dark** — easy on the eyes (Catppuccin Mocha palette)
- **Light** — for bright environments (Catppuccin Latte palette)
- **Auto** (default) — follows your Windows system theme automatically

<img src="screenshots/settings.png" alt="ZaiMeter settings — theme, language, notifications, icon style" width="360">

### 🌐 Languages (35)

- 🇬🇧 English (default)
- 🇺🇦 Українська
- 🇪🇸 Español
- 🇩🇪 Deutsch
- 🇫🇷 Français
- 🇵🇹 Português
- 🇮🇹 Italiano
- 🇮🇳 हिन्दी
- 🇹🇷 Türkçe
- 🇳🇱 Nederlands
- 🇵🇱 Polski
- 🇻🇳 Tiếng Việt
- 🇷🇺 Русский
- 🇹🇭 ภาษาไทย
- 🇮🇩 Bahasa Indonesia
- 🇸🇪 Svenska
- 🇨🇿 Čeština
- 🇯🇵 日本語
- 🇰🇷 한국어
- 🇨🇳 简体中文
- 🇧🇬 Български
- 🇬🇷 Ελληνικά
- 🇮🇱 עברית
- 🇲🇾 Bahasa Melayu
- 🇳🇴 Norsk
- 🇸🇦 العربية
- 🇷🇴 Română
- 🇩🇰 Dansk
- 🇫🇮 Suomi
- 🇭🇺 Magyar
- 🇵🇭 Filipino
- 🇧🇩 বাংলা
- 🇮🇷 فارسی
- 🇸🇰 Slovenčina
- 🇷🇸 Српски

### 🧩 Mini Widget

- **Floating PiP window** — always-on-top 52x28px window showing current usage %
- **Color-coded** — green/yellow/red background matching usage level
- **Draggable** — drag anywhere on screen
- **Click to open** — click the widget to open the full dashboard
- **Disabled by default** — enable in Settings → "Show widget"

### ♿ Accessibility

- **Colorblind patterns** — progress bars show pattern overlays: dots (green), diagonal stripes (yellow), cross-hatch (red)
- **Disabled by default** — enable in Settings → "Accessibility patterns"

### 🔄 Auto-Update

- **Checks GitHub Releases** on startup for newer versions
- **Balloon notification** — shows a tray balloon if a new version is available
- **Enabled by default** — toggle in Settings → "Check for updates"

### 🔔 Smart Notifications

- Windows toast notifications at configurable thresholds (50%, 75%, 90% by default)
- **Aggregated alerts** — when multiple thresholds are crossed simultaneously, a single batched notification is shown instead of separate alerts
- **Informative alerts** — shows metric name, current %, exceeded threshold, and reset countdown
- **Sound alerts** — system notification sound (configurable on/off)
- **Startup notification** — confirmation that ZaiMeter is running in the tray
- **Deduplication** — won't spam; resets when usage drops below threshold

<img src="screenshots/notification.png" alt="ZaiMeter Windows toast notification warning about Claude usage threshold" width="360">

### 📤 Data Export

- **CSV export** — right-click tray → "Export History (CSV)" to save full usage history
- **SQLite database** — 30-day rolling history stored next to the .exe

### ⚙ Smart Polling

- **Adaptive interval** — polls every 120–300s normally; tightens to 120–180s in the last 15 minutes before each hour (when limits are about to reset)
- **Randomized timing** — each poll interval is randomly chosen to avoid predictable patterns
- **Idle detection** — pauses API polling when PC is idle for 5+ minutes
- **Exponential backoff** — on API errors, interval doubles (2x, 4x, 8x) up to 10 min cap
- **Rate-limit handling** — graceful 429 response parsing with retry-after
- **Sleep/wake progressive retry** — after resuming from sleep/hibernate, retries at 2s, 5s, 15s, 30s intervals until a successful response
- **Network change detection** — detects when network connectivity is restored and triggers an immediate poll
- **Credential file watcher** — monitors `~/.claude/` for changes and re-polls immediately when credentials are updated
- **Web API fallback** — optional fallback to claude.ai web API when OAuth is unavailable (configure `web_api_session_key` and `web_api_org_id`)
- **Config validation** — sanitizes all values on load (polling interval 30-600s, thresholds 1-100%)

## ⚙ Configuration

`config.json` is auto-created on first launch:

| Platform | Default location |
|----------|------------------|
| Windows | Next to `zaimeter.exe` |
| macOS | `~/Library/Application Support/ZaiMeter/config.json` |

```json
{
  "version": "1.0.0",
  "polling_interval_seconds": 120,
  "notifications": {
    "enabled": true,
    "thresholds": [50, 75, 90],
    "sound": true
  },
  "autostart": false,
  "compact_mode": false,
  "theme": "auto",
  "language": "auto",
  "show_chatgpt_section": false,
  "show_widget": false,
  "check_updates": true,
  "accessibility_patterns": false,
  "tray_icon_style": "number",
  "dashboard_layout": "standard",
  "hide_extra_usage": false,
  "custom_colors": {},
  "quiet_hours": {
    "enabled": false,
    "start": "22:00",
    "end": "08:00"
  },
  "web_api_session_key": null,
  "web_api_org_id": null
}
```

| Field | Default | Range | Description |
|-------|---------|-------|-------------|
| `polling_interval_seconds` | `120` | 30–600 | How often to check usage (validated on load) |
| `notifications.enabled` | `true` | — | Enable/disable toast notifications |
| `notifications.thresholds` | `[50,75,90]` | 1–100 | Usage % levels that trigger alerts |
| `notifications.sound` | `true` | — | Play system sound with notifications |
| `theme` | `"auto"` | auto/dark/light | Color theme |
| `language` | `"auto"` | auto/en/uk/.../zh | UI language (35 languages) |
| `compact_mode` | `false` | — | Compact dashboard layout |
| `show_chatgpt_section` | `false` | — | Show ChatGPT quick-link panel |
| `autostart` | `false` | — | Start with Windows or macOS LaunchAgent |
| `show_widget` | `false` | — | Show floating mini-widget |
| `check_updates` | `true` | — | Check for updates on startup |
| `tray_icon_style` | `"number"` | number/ring/bar/pie | Tray icon style: number (%), ring (circular), bar (vertical), pie (multi-metric) |
| `accessibility_patterns` | `false` | — | Colorblind overlay patterns on progress bars |
| `dashboard_layout` | `"standard"` | minimal/standard/detailed | Dashboard layout mode |
| `hide_extra_usage` | `false` | — | Hide extra_usage metric from dashboard |
| `custom_colors` | `{}` | hex strings | Override theme colors (e.g. `{"green": "#00ff00"}`) |
| `quiet_hours.enabled` | `false` | — | Suppress notifications during quiet hours |
| `quiet_hours.start` | `"22:00"` | HH:MM | Quiet hours start time |
| `quiet_hours.end` | `"08:00"` | HH:MM | Quiet hours end time |
| `web_api_session_key` | `null` | string | Session key for claude.ai web API fallback |
| `web_api_org_id` | `null` | string | Organization ID for claude.ai web API fallback |

## ⌨ Keyboard Shortcuts

| Key | Action |
|-----|--------|
| **ESC** | Close dashboard popup |
| **F5** | Refresh usage data |

## 🔨 Building from Source

### Windows

```bash
git clone https://github.com/klivak/zaimeter.git
cd zaimeter
cargo build --release
# Output: target/release/zaimeter.exe (~3 MB)
```

**Requirements:** Rust 1.75+ and Windows SDK (included with [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)).

### macOS

```bash
git clone https://github.com/klivak/zaimeter.git
cd zaimeter
sh scripts/build-macos-app.sh
# Output:
# target/aarch64-apple-darwin/release/ZaiMeter.app
# target/aarch64-apple-darwin/release/ZaiMeter-macos-arm64.app.zip
# target/aarch64-apple-darwin/release/zaimeter-macos-arm64
```

**Requirements:** Rust stable, Xcode Command Line Tools, Swift compiler, and macOS 12+.

## 🔑 How Authentication Works

ZaiMeter does **not** ask for your password or API key. It reuses the OAuth token that [Claude Code](https://claude.ai/download) already stores on your machine.

**Token lookup order:**

| # | Location | Used by |
|---|----------|---------|
| 1 | `~/.claude/.credentials.json` | Claude Code v2.x+ on Windows and macOS |
| 2 | Windows Credential Manager (`Claude Code-credentials`) | Claude Code v1.x on Windows (legacy) |

When you run `claude` and log in via the browser, Claude Code saves an OAuth token to `~/.claude/.credentials.json`. ZaiMeter reads this file to authenticate with the Anthropic Usage API — no extra setup needed.

**What's stored in the file:**

```json
{
  "claudeAiOauth": {
    "accessToken": "sk-ant-oat01-...",
    "refreshToken": "sk-ant-ort01-...",
    "expiresAt": 1772467364905,
    "subscriptionType": "max"
  }
}
```

ZaiMeter uses `accessToken` to fetch your usage data and `subscriptionType` to display your plan (Pro/Max). It never modifies this file.

> **Troubleshooting:** If ZaiMeter shows "Credentials not found", run `claude` in a terminal and log in. Then click Refresh in ZaiMeter. On macOS, use **Refresh Now** from the menu bar item and check **Open Logs** if the status remains cached or stale.

## ❓ FAQ

**Q: Does it work without Claude Code installed?**
A: ZaiMeter launches but shows a "Credentials not found" message with a link to claude.ai. You need Claude Code logged in so ZaiMeter can read the OAuth token from `~/.claude/.credentials.json`.

**Q: How much RAM does it really use?**
A: Typically **3–8 MB**. Built in Rust with native Win32 API — no Electron, no browser engine.

**Q: Is it safe? Does it send my data anywhere?**
A: ZaiMeter is fully open source. It only communicates with `api.anthropic.com` to fetch YOUR usage data using YOUR existing OAuth token. Zero telemetry. Every release binary is automatically scanned by [VirusTotal](https://www.virustotal.com/) (60+ antivirus engines) — check the scan link in each [release](https://github.com/klivak/zaimeter/releases/latest).

**Q: Why isn't ChatGPT tracking automatic?**
A: OpenAI deliberately does not expose ChatGPT subscription usage via any public API.

**Q: How do I check my Claude usage limits?**
A: Just run ZaiMeter — it reads your Claude Code OAuth token and shows all your limits (5-hour session, weekly cap, Sonnet/Opus quotas) in a system tray popup. No manual checking needed.

**Q: Does it work with Claude Pro, Max 5x, and Max 20x plans?**
A: Yes. ZaiMeter auto-detects your plan tier and displays the correct limits for Pro, Max, Max 5x, and Max 20x subscriptions.

**Q: What is the Claude 5-hour session limit?**
A: Claude enforces a rolling 5-hour usage window. ZaiMeter shows your current utilization percentage and a countdown to when it resets.

**Q: Can I run ZaiMeter from a USB drive?**
A: On Windows, yes. It's a single portable `.exe` with zero dependencies. On macOS, use the `.app` bundle for the menu bar UI or the raw `zaimeter-macos-arm64` binary for CLI/agent use.

**Q: Does the macOS version have a real menu bar UI?**
A: Yes. Starting with v4.0.1, ZaiMeter ships a native AppKit menu bar app with usage %, freshness status, force refresh, Claude link, config import/export, autostart toggle, update check, and log access.

**Q: How do I know if the value is cached?**
A: On macOS, the menu shows freshness state such as Live, seconds/minutes old, cached/no data, refreshing, or API error. On Windows, use the tray refresh/dashboard behavior and settings to force a refresh.

**Q: Does it support multiple languages?**
A: Yes — 35 languages: English, Ukrainian, Spanish, German, French, Portuguese, Italian, Hindi, Turkish, Dutch, Polish, Vietnamese, Russian, Thai, Indonesian, Swedish, Czech, Japanese, Korean, Chinese (Simplified), Bulgarian, Greek, Hebrew, Malay, Norwegian, Arabic, Romanian, Danish, Finnish, Hungarian, Filipino, Bengali, Persian, Slovak, and Serbian.

## 📄 License

[MIT](LICENSE) — free for personal and commercial use.

---

<div align="center">

**🦀 Purposefully built in Rust for minimal footprint and maximum reliability**
**3–8 MB RAM · Single .exe · Zero dependencies · Open source**

Made by [klivak](https://github.com/klivak)

*Claude is a trademark of Anthropic. ChatGPT is a trademark of OpenAI.*
*ZaiMeter is an independent open-source project with no official affiliation.*

</div>
