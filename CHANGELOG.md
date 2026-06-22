# Changelog

All notable changes to ZaiMeter will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- **Test suite no longer time-rots** - `test_query_recent_readings_ordered` used fixed 2026-06-13 fixtures inside an 8-day recency window, turning the released 5.0.0 suite red after ~2026-06-21. Anchored to UTC now minus 2 days. Test-only; no binary change.

## [5.0.0] - 2026-06-18

Retargeted from Claude/Windows to the z.ai GLM Coding Plan on macOS, and renamed
ClaudeMeter to ZaiMeter. The z.ai backend is the default; the original Claude
provider is retained behind a config flag.

### Added
- **z.ai (GLM Coding Plan) usage provider** - decodes the 5-hour and weekly token limits and the built-in tools quota from z.ai's account-monitoring API (`/api/monitor/usage/quota/limit`); default backend
- **Provider config toggle** (`provider`: `zai` | `claude`) with a `Provider` dispatch enum; token read from `GLM_API_KEY` in `~/.hermes/.env` (mirrors the `zai` shell launcher)

### Changed
- **Renamed ClaudeMeter to ZaiMeter**: binary `zaimeter`, bundle id `com.klivak.zaimeter`, app bundle, app-data folder, LaunchAgent, menu bar, notifications, and i18n
- **"Open Z.ai Usage"** opens the real z.ai coding-plan usage page
- **plan_override is ignored for z.ai** so the API's own level (e.g. "GLM Max") is shown
- History DB queries parameterized by provider (`query_24h_chart`, `query_recent_readings`)

### Removed
- **Update checker** (no fork releases to check)

## [4.0.1] - 2026-05-31

### Fixed
- **macOS app bundle build** - compile the Swift menu bar app with -parse-as-library for GitHub macOS runners

## [4.0.0] - 2026-05-31

### Added
- **Native macOS menu bar app** — added a Swift/AppKit `NSStatusItem` app with live percentage, freshness state, Refresh, Open Claude, update check, config import/export, logs, and autostart controls
- **macOS `.app` bundle release** — CI now packages `ZaiMeter.app` with the Rust polling agent embedded as `zaimeter-agent`
- **macOS data freshness status** — the agent writes `status.json` with `Live`, `Refreshing`, cached/no-data, and API error states for the menu UI
- **Portable macOS logs** — the macOS agent writes `zaimeter.log` under `~/Library/Application Support/ZaiMeter`
- **Optional macOS notarization hook** — release workflow notarizes the app zip when Apple Developer secrets are configured

### Changed
- **Dependency refresh** — updated compatible Rust dependencies in `Cargo.lock`

## [3.1.4] - 2026-05-31

### Fixed
- **macOS CI build script** — gated Windows resource compilation in `build.rs` so macOS builds no longer require the Windows-only `winres` build dependency

## [3.1.3] - 2026-05-31

### Fixed
- **CI clippy compatibility** — restored crate-wide lint allowances that were lost when the Windows entrypoint was split out of `main.rs`, so `cargo clippy -- -D warnings` passes again

## [3.1.2] - 2026-05-31

### Added
- **macOS low-memory background agent** — added a native Rust macOS entrypoint, LaunchAgent installer script, and GitHub Actions builds for Apple Silicon without Electron/WebView overhead

### Changed
- **Login expiry warnings disabled by default** — `token_expiry_warning` now defaults to `false`, including old config files that do not have the field

### Fixed
- **Manual refresh no longer gets dropped** — user-triggered refreshes queue one follow-up poll when another poll is already running, avoiding stale cached display after opening the dashboard or pressing Refresh/F5
- **Usage API cache bypass** — usage requests now include cache-busting query parameters and no-cache headers for fresher responses
- **Cross-platform credentials module** — Windows Credential Manager access is now gated to Windows, while file-based Claude credentials work cross-platform

## [3.1.1] - 2026-04-26

### Fixed
- **Security: rustls-webpki CRL parsing panic** — updated rustls-webpki 0.103.12 → 0.103.13 to fix RUSTSEC-2026-0104 (reachable panic when parsing certificate revocation lists with an empty `BIT STRING` in `onlySomeReasons` of an `IssuingDistributionPoint` extension)

## [3.1.0] - 2026-04-26

### Added
- **Toggle for startup tray notification** — new `show_startup_notification` config option (default: true) and a settings-panel toggle to silence the "Running in system tray" balloon shown on every launch
- **Settings toggle for token-expiry warning** — the existing `token_expiry_warning` flag is now exposed in the settings panel as "Show login expiry warning" (was previously only editable via config.json)

### Fixed
- **Update-available balloon now opens the GitHub release page** — clicking the toast/balloon for a newer version opens the release URL in the default browser instead of doing nothing
- **`token_expiry_warning` config flag was ignored** — the option existed since v2.2.6 but wasn't actually checked when firing the notification; now properly suppresses the balloon when set to false

## [3.0.1] - 2026-04-20

### Fixed
- **Security: rustls-webpki name constraints vulnerabilities** — updated rustls-webpki 0.103.10 → 0.103.12 to fix RUSTSEC-2026-0098 (URI names incorrectly accepted) and RUSTSEC-2026-0099 (wildcard certificate name constraints)

## [3.0.0] - 2026-04-04

### Added
- **Credential file watcher** — monitors `~/.claude/` directory for changes and triggers immediate re-poll when credentials are updated, instead of waiting for the next polling cycle
- **Network connectivity monitor** — detects network interface changes (e.g., WiFi reconnect) via Windows IP Helper API and triggers an immediate poll when connectivity is restored
- **Sleep/wake progressive retry** — after resuming from sleep/hibernate, retries at 2s, 5s, 15s, 30s intervals instead of a single immediate poll; stops retrying on first successful response
- **Subscription type badge** — the plan name (Pro, Max, Max 5X, Max 20X) is now displayed as a colored pill-shaped badge in the dashboard header instead of plain text
- **Web API fallback** — when OAuth credentials are unavailable or expired, ZaiMeter can optionally fall back to the claude.ai web API using session cookies; configure `web_api_session_key` and `web_api_org_id` in config.json
- **Aggregated notifications** — when multiple usage thresholds are crossed simultaneously, a single batched notification is shown instead of multiple separate alerts

### Fixed
- **Security: rustls-webpki CRL vulnerability** — updated rustls-webpki 0.103.9 → 0.103.10 to fix RUSTSEC-2026-0049

### Changed
- **CI: Node.js 24 migration** — updated `actions/checkout` v4 → v6 across all workflows to resolve Node.js 20 deprecation warnings
- **CI: audit workflow** — replaced `rustsec/audit-check@v2` (which required issue-creation permissions) with direct `cargo audit` for simpler, more reliable security scanning

## [2.2.6] - 2026-03-16

### Fixed
- **Security: quinn-proto DoS vulnerability** — updated quinn-proto 0.11.13 → 0.11.14 to fix RUSTSEC-2026-0037 (high severity)

### Added
- **Token expiry warning** — new `token_expiry_warning` config option (default: true)

## [2.2.5] - 2026-03-09

### Fixed
- **UI thread blocking** — removed synchronous file I/O (config reload) and registry reads (theme detection) from the paint handler that ran every 16ms frame; config and theme are now refreshed on a 5-second timer instead
- **Popup open delay** — removed blocking config reload from popup show path
- **DB blocking on main thread** — moved SQLite insert/query operations to a background thread so the UI stays responsive during database writes after each poll

## [2.2.4] - 2026-03-09

### Changed
- **Adaptive polling interval** — normal interval widened to 120–300s (was 90–180s); tightens to 120–180s in the last 15 minutes of each hour when limits are about to reset

## [2.2.3] - 2026-03-08

### Fixed
- **Autostart broken after update** — removed version number from release binary filename so users can replace `zaimeter.exe` in-place without breaking the autostart registry path

## [2.2.2] - 2026-03-07

### Fixed
- **Autostart not surviving exe relocation** — autostart registry entry is now synced on every launch, so moving the exe to a new folder (e.g., after downloading a new release) no longer silently breaks startup with Windows

## [2.2.1] - 2026-03-05

### Fixed
- **Plan header spacing** — restored visible space between `Plan` label and Max tier text in the dashboard header
- **CI format check** — applied `cargo fmt` formatting for newly added i18n/render code so `cargo fmt --check` passes

## [2.2.0] - 2026-03-05

### Added
- **Slide animation** — smooth horizontal slide transition between Dashboard and Settings views
- **Dashboard layouts** — three layout modes selectable in Settings: Minimal (single largest metric, large percentage), Standard (existing view), Detailed (metrics with inline sparkline charts)
- **Gradient progress bars** — full-spectrum green→amber→coral gradient across progress bars reflecting utilization position; falls back to solid colors when custom color overrides are active
- **Pie chart tray icon** — new "Pie" icon style showing all metrics as proportional colored sectors in a mini pie chart
- **Rate of change indicators** — arrow symbols (↑↗→↘↓) next to each metric percentage showing utilization trend based on hourly rate of change from historical data
- **Focus Assist integration** — automatically suppresses all notifications when Windows Focus Assist / Do Not Disturb is active (uses SHQueryUserNotificationState API)

## [1.10.4] - 2026-03-05

### Added
- **30-day chart view** — usage history chart now has a "24h | 7d | 30d" toggle to switch between 24-hour, 7-day, and 30-day views with daily bucket intervals

## [1.10.3] - 2026-03-05

### Fixed
- **Polling interval randomization** — API polling now uses a random interval between 90-180 seconds instead of a fixed interval, reducing the risk of rate limiting and looking more natural to the API
- **Rate limit retry floor** — minimum retry-after on 429 responses raised from 1s to 90s

## [1.10.2] - 2026-03-05

### Fixed
- **Tooltip plan name** — compact plan display: "Max 5X" → "5x", "Max 20X" → "20x" for cleaner tooltip
- **Tooltip extra metrics** — single extra metric now shown inline with name and reset time instead of "+1 extra"

## [1.10.1] - 2026-03-05

### Fixed
- **Tooltip format** — restored reset times and spacing in tray tooltip; extra metrics shown as "+N extra" summary; removed redundant app name header

## [1.10.0] - 2026-03-05

### Added
- **7-day chart toggle** — usage history chart now has a "24h | 7d" toggle to switch between 24-hour and 7-day views with 4-hour bucket intervals
- **Copy button** — clipboard icon in the popup footer copies all current usage metrics to the clipboard for easy sharing

### Fixed
- **Tooltip truncation** — tray hover tooltip now uses a compact format (no empty lines or reset times) to fit all metrics within the 127-character Win32 limit

## [1.9.0] - 2026-03-05

### Added
- **Tooltip with error info** — hovering the tray icon now shows the error type (token expired, connection error, rate limited, etc.) when data is unavailable, and a warning indicator when stale data is shown
- **Token expiry warning** — proactive balloon notification when the OAuth token is about to expire within 1 hour, with guidance to run `claude login` to refresh
- **Wake-from-sleep polling** — app immediately polls the API when the system resumes from sleep/hibernate, instead of waiting for the next timer tick (up to 10 min with backoff)

## [1.8.3] - 2026-03-05

### Added
- **Cached data on startup** — last known usage data is loaded from the local database at launch, so the dashboard shows data immediately even if the first API poll fails (e.g. rate limited)

### Fixed
- **Popup clipping on error screen** — the "Status ↗" link added in v1.8.0 was cut off because the popup height calculation didn't account for it
- **Concurrent poll requests** — multiple simultaneous API requests (e.g. from timer + manual refresh) could trigger 429 rate limits; added a guard to prevent overlapping polls

## [1.8.2] - 2026-03-05

### Fixed
- **"Retry after 0s"** — rate limit retry-after header with value 0 now shows minimum 1s; non-numeric values default to 60s

## [1.8.1] - 2026-03-05

### Fixed
- **Extra Usage not displayed** — `extra_usage` field from the API (monthly credits utilization) was silently ignored because `resets_at` was missing from the response; added `#[serde(default)]` so missing optional fields parse correctly

## [1.8.0] - 2026-03-05

### Added
- **Detailed error diagnostics** — instead of a generic "Claude Code not detected" for all failures, the app now shows specific error messages: token expired, network error, rate limited, server error, or credentials not found
- **Error blink** — tray icon blinks when a poll error occurs and there is no cached data, stops when the user opens the popup
- **Status link on error screen** — "Status ↗" link to status.claude.com is now shown on the error/not-detected screen, not just on the main dashboard

### Fixed
- **Data loss on transient errors** — previously, a single failed API poll would wipe all cached usage data and show "not detected"; now previous data is preserved until a successful poll replaces it

## [1.7.2] - 2026-03-03

### Added
- **5 new languages** — Arabic, Romanian, Danish, Finnish, Hungarian (25 languages total)

### Fixed
- **Tray icon color mismatch** — icon color now matches the displayed value (5-hour session %) instead of using the max across all limits, which caused a yellow icon even at 3% session usage

## [1.7.1] - 2026-03-03

### Added
- **Quiet hours** — suppress notifications during configurable time window (default 22:00–08:00, disabled by default). Set `quiet_hours.enabled: true` in config.json
- **Theme popup picker** — click Theme in settings to choose from a popup menu instead of cycling through options

### Fixed
- **Chart time alignment** — 24h usage chart now always renders 48 fixed 30-minute slots aligned with x-axis labels. Previously, sparse data was stretched across the full chart width, misaligning bars with time labels
- **Config language validation** — added 5 missing languages (ru, th, id, sv, cs) to validator; previously selecting these languages via config.json would reset to "auto" on next load

## [1.7.0] - 2026-03-03

### Added
- **5 new languages** — Russian, Thai, Indonesian, Swedish, Czech (20 languages total)
- **Popup language picker** — click Language in settings to open a popup menu with all 20 languages instead of cycling through them one by one
- **Widget hover tooltip** — hover the mini-widget to see full usage breakdown (all metrics, reset times, plan info)

### Changed
- Widget now shows 5-hour session utilization instead of max across all metrics (fixes misleading orange color)
- Version strings now use `CARGO_PKG_VERSION` — no more hardcoded version numbers in source code

### Fixed
- Widget color mismatch — widget showed orange (high %) when only the weekly limit was high, even though the 5-hour session was low

## [1.6.1] - 2026-03-03

### Fixed
- **Single notification per threshold jump** — when usage crosses multiple thresholds at once (e.g., 0% → 96%), only the highest applicable threshold fires a notification instead of all three

## [1.6.0] - 2026-03-03

### Added
- **Configurable tray icon style** — choose between Number (default), Ring (circular progress), or Bar (vertical fill) via Settings or `tray_icon_style` in config.json
- **i18n for icon style** — "Icon style", "Number", "Ring", "Bar" translated in all 15 languages
- **Missing i18n keys for Ukrainian** — added Show widget, Check for updates, Accessibility patterns, Update available translations

### Changed
- Settings panel expanded from 8 to 9 rows (added icon style selector)

## [1.5.0] - 2026-03-03

### Added
- **5 new languages** — Hindi, Turkish, Dutch, Polish, Vietnamese (15 languages total)
- **Notification screenshot** in README for Smart Notifications section

### Changed
- Language cycling now covers all 15 languages in order

## [1.4.0] - 2026-03-03

### Added
- **5 new languages** — Portuguese, Japanese, Korean, Chinese (Simplified), Italian (10 languages total)
- **Better language picker** — settings panel shows full native language name (e.g. "Português", "日本語") instead of language code
- **Mini floating widget** — always-on-top PiP window showing current usage % with color-coded background. Draggable, click to open dashboard. Disabled by default; enable in Settings
- **Auto-update checker** — checks GitHub Releases on startup for newer versions, shows balloon notification if available. Enabled by default; toggle in Settings
- **Accessibility patterns** — colorblind-friendly overlays on progress bars: dots (green), diagonal stripes (yellow), cross-hatch (red). Disabled by default; enable in Settings
- **Custom color themes** — override any theme color via `custom_colors` in config.json (hex values like `"#ff0000"`)

### Changed
- Settings panel expanded from 5 to 8 rows (added widget toggle, update checker toggle, accessibility patterns toggle)
- Language cycling now covers all 10 languages in order

## [1.3.6] - 2026-03-03

### Added
- **VirusTotal scanning** — every release binary is automatically scanned by VirusTotal (60+ antivirus engines) with a link to the full report in release notes
- **VirusTotal badge** in README for transparency and trust

## [1.3.0] - 2026-03-03

### Added
- **Claude Status link** — "Status" link on the Claude section header opens https://status.claude.com/

### Fixed
- **Tray icon shows session %, not weekly max** — when a 5-hour session is active, the tray icon now shows the session utilization (e.g. 7%) instead of the weekly maximum (e.g. 41%)
- **Tray icon shows "..." when no active session** — instead of showing the weekly limit number, the icon displays "..." when no 5-hour session is running
- **Chart no longer shows phantom activity** — usage history chart now filters out records with no active session; old invalid data is cleaned up on startup
- **Tray icon text readability** — white text on green background (was black, hard to read)
- **Notifications use native balloon tips** — replaced unreliable PowerShell toast notifications with Win32 balloon tips that always show both title and body text

## [1.2.0] - 2026-03-03

### Added
- **Dynamic tray icon with % number** — shows actual utilization percentage on the tray icon
- **Gradient progress bars** — smooth color gradients on metric bars
- **Animated progress bars** — bars smoothly fill on popup open (~60fps lerp)
- **Popup fade-in animation** — smooth opacity transition when opening dashboard
- **Chart bar hover tooltip** — hover chart bars to see exact % and time
- **CSV export** — export full usage history from context menu
- **Mica backdrop** — Windows 11 translucent Mica effect on popup
- **Keyboard shortcuts** — ESC to close popup, F5 to refresh
- **Notification sound** — system beep with notifications (configurable)
- **Informative notifications** — shows metric name, current %, exceeded threshold, and reset time
- **Startup notification** — "Running in system tray" toast on launch
- **Auto-refresh on popup open** — triggers poll if data is older than 60 seconds
- **Tray icon blink** — icon blinks when usage exceeds 90% until popup is opened
- **Idle detection** — pauses API polling when PC is idle for 5+ minutes (saves bandwidth)
- **Retry with exponential backoff** — on API errors, poll interval doubles (up to 10 min cap)
- **Rate-limit (429) handling** — graceful retry-after parsing for Anthropic API
- **Config validation** — sanitizes polling interval, thresholds, theme, and language on load

### Fixed
- PowerShell notification window no longer flashes on startup (CREATE_NO_WINDOW flag)
- Tray icon text contrast — black text on green/yellow, white text on red/gray for readability

## [1.1.0] - 2026-03-02

### Added
- Direct2D + DirectWrite hardware-accelerated rendering (replaces GDI)
- DWM dark title bar integration
- Session reset lines (dashed vertical) on 24h usage chart
- Tooltip spacing between metric values
- Screenshots in README (dashboard, tooltip, settings)

### Fixed
- DPI scaling at 125%/150% — popup no longer clips content
- Memory reclaimed when popup is closed (D2D resources released + working set trimmed)
- Settings gear and close button visibility at non-100% DPI
- Credential error display improvements

## [1.0.0] - 2026-02-25

### Added
- Initial release
- Claude usage monitoring (5-hour, 7-day, Sonnet, Opus + dynamic metrics)
- Auto-detection of Claude Pro/Max plan
- Future-proof API parsing (unknown metrics auto-displayed)
- OAuth token retrieval from Windows Credential Manager
- System tray with dynamic color-coded icons (green/yellow/red/gray)
- Rich tooltip with full usage summary on hover
- Dashboard popup with progress bars and countdown timers
- ChatGPT/Codex info section with link to usage page (hidden by default)
- 24-hour usage history chart from SQLite database
- Windows toast notifications at configurable thresholds (50%, 75%, 90%)
- Auto-start with Windows (registry + batch script)
- Compact mode toggle
- Theme: Dark / Light / Auto (follows Windows system theme)
- Languages: English, Українська, Español, Deutsch, Français
- Auto language detection from Windows settings
- Portable config.json next to .exe
- Single .exe, zero dependencies, under 10 MB RAM
- Built with Rust for minimal memory footprint
