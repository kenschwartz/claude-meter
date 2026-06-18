# CLAUDE.md - Project Instructions for AI Assistants

> Context and rules for Claude Code and other AI assistants working on ZaiMeter.

## Project Summary

ZaiMeter is a macOS menu-bar app that monitors **z.ai GLM Coding Plan** usage limits
(5-hour + weekly token limits) in real time. It is a two-process app: a Rust "agent"
binary polls the z.ai API on an interval and writes a JSON state file; a native Swift
menu-bar app (AppKit) reads that file and renders the status item, dropdown, 24h chart,
and a "free weekly flush" celebration.

The z.ai provider is the default. The original Claude (Anthropic) provider is kept behind
a config flag for parity, but the app's purpose is z.ai.

Forked from klivak/ClaudeMeter (a Windows Claude tray app). macOS-only in practice; the
Windows sources remain in tree under `cfg(windows)` but are not the target.

Author: klivak (upstream) | Fork: z.ai GLM Coding Plan | License: MIT

## Build & Run (macOS)

```bash
# Rust agent only (host target) - fast dev loop
cargo build
cargo run -- --once        # single poll, writes status.json, exits

# Lint / format / test (clippy must pass with ZERO warnings)
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test

# Full native .app (release cargo for aarch64-apple-darwin + swiftc)
scripts/build-macos-app.sh          # -> target/aarch64-apple-darwin/release/ZaiMeter.app

# Install + autostart
ditto target/aarch64-apple-darwin/release/ZaiMeter.app /Applications/ZaiMeter.app
scripts/install-macos-launchagent.sh   # writes/loads ~/Library/LaunchAgents/com.klivak.zaimeter.plist
```

The agent accepts `--once` (single poll + exit), `--refresh` (same), `--status` (print
status.json), or `--agent` (run by the Swift UI). Default is the poll loop.

## Architecture

**Two processes, one JSON file.** No IPC, no sockets.

- Rust agent (`src/macos_app.rs` -> binary `zaimeter`, bundled as `zaimeter-agent`):
  reads config + token, constructs a `Provider`, polls on `polling_interval_seconds`,
  writes `~/Library/Application Support/ZaiMeter/status.json` and appends to
  `zaimeter.db` (SQLite history) each poll.
- Swift UI (`macos/ZaiMeterApp.swift` -> `ZaiMeter`): `NSStatusItem` menu-bar app. Spawns
  the agent as a child, reads `status.json` every 5s, renders title/dropdown/chart/celebration.

State lives in `~/Library/Application Support/ZaiMeter/`: `status.json`, `zaimeter.db`,
`zaimeter.log`, `config.json`, `celebrate_state.json`.

### Source map
- `src/main.rs` - entry; cfg-gates to `macos_app::run()` on macOS, `windows_app::run()` on Windows.
- `src/macos_app.rs` - the macOS agent: poll loop, credential/fetch via `Provider`, history save, `publish_status` (writes status.json), free-flush celebration logic.
- `src/providers/mod.rs` - `Provider` enum (`Claude` | `Zai`) with `fetch()` (reads its own credential + fetches) and `name()`/`login_hint()`.
- `src/providers/zai.rs` - `ZaiClient`: hits z.ai `quota/limit`, decodes into `UsageResponse`.
- `src/providers/claude.rs` - `ClaudeClient` + the shared `UsageResponse` / `UsageMetric` types all providers return.
- `src/credentials.rs` - `read_claude_token()` (Anthropic OAuth) and `read_zai_token()` (GLM_API_KEY).
- `src/config.rs` - `Config` (incl. `provider`), load/save, mtime hot-reload, `validate()`.
- `src/db.rs` - SQLite `usage_history` (provider, metric, utilization, resets_at); chart/readings queries are provider-parameterized.
- `macos/ZaiMeterApp.swift` - the native menu-bar UI.

## z.ai Usage API (verified)

All `GET`, header `Authorization: Bearer <GLM_API_KEY>`.

`https://api.z.ai/api/monitor/usage/quota/limit` -> `{ code, success, msg, data: { level, limits: [...] } }`.
Each limit has `type`, `unit`, `number`, `percentage`, `nextResetTime` (epoch ms). Decode
(in `zai.rs`, confirmed against the live account + `opencode-glm-quota`):
- `TOKENS_LIMIT` unit=3 number=5 -> `five_hour` (5-hour token limit)
- `TOKENS_LIMIT` unit=6 number=1 -> `seven_day` (weekly token limit)
- `TIME_LIMIT` -> extra `tools_5h` (built-in tools quota)
- `data.level` ("max"/"pro"/"free"/...) -> plan label "GLM Max" etc.

`https://api.z.ai/manage-apikey/coding-plan/personal/usage` is the human-facing usage page
(the "Open Z.ai Usage" menu item).

**Token source:** `GLM_API_KEY` in `~/.hermes/.env` (canonical fleet secrets file, 0600),
mirroring the `zai` shell launcher. Fallbacks: `ANTHROPIC_AUTH_TOKEN`, then `GLM_API_KEY`
env vars. The token is a secret - read it, never log it, never persist it elsewhere.

## Config (`config.json`)

Key fields:
- `provider`: `"zai"` (default) or `"claude"` - selects the backend.
- `polling_interval_seconds`: poll cadence (min 60).
- `plan_override`: Claude tier label (Pro/Max 5x/Max 20x); **ignored for z.ai**.
- `celebrate_free_flush` + tunables: the weekly-flush party.
- `show_startup_notification`, `theme`, etc.

Missing fields default in via `#[serde(default = ...)]`; `validate()` coerces bad values.

## Conventions

- **No em dashes anywhere** (output, comments, docs, code, commits). Use a hyphen, colon, or rephrase.
- **clippy must pass with zero warnings** (`-D warnings`).
- **Error handling:** never panic in release; `Result<>` everywhere; keep last-known data on API failure.
- **i18n (`src/i18n/`) is Windows-only** (`#[cfg(windows)]`); the macOS UI hardcodes its strings.
- **Naming:** snake_case files/functions, PascalCase types.

## Invariants (load-bearing - do not break)

- Metric keys `"five_hour"` and `"seven_day"` are emitted by both providers and selected on
  by the Swift title/pace logic and the `query_24h_chart` SQL. Do not rename them.
- `status.json` is provider-neutral (state/title/detail/plan/percent/metrics/tier_note/
  last_api_update/data_age_seconds/error/chart/chart_resets/celebrate). Changing its schema
  breaks the Swift reader.
- The GLM_API_KEY token must never reach a log line, status.json, the DB, an error string, or a notification.
- Builds clean: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`.

## Testing

`cargo test` covers the provider decoders, config validation, db queries, and the
celebration flush logic. There is no GUI test harness; verify UI changes by building the
`.app` and watching the menu bar. A quick data-layer check: `cargo run -- --once` then
inspect `~/Library/Application Support/ZaiMeter/status.json`.
