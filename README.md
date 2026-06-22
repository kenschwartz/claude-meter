# ZaiMeter

A macOS menu-bar app that shows your **z.ai GLM Coding Plan** usage in real time: the
5-hour token limit, the weekly token limit, and the built-in tools quota. Lightweight
native Swift UI backed by a tiny Rust agent. Under 10 MB RAM.

ZaiMeter polls z.ai's account-monitoring API every couple of minutes and renders the
result as a menu-bar status item with a dropdown breakdown, a 24-hour history chart, and
a small celebration when your weekly counter gets a "free flush" (resets out of band).

> Forked from [klivak/ClaudeMeter](https://github.com/klivak/claudemeter), retargeted from
> Claude/Windows to z.ai/macOS. The original Claude (Anthropic) provider is still in the
> tree behind a config flag; z.ai is the default.

## What it shows

- **Menu-bar title:** max utilization across your limits, e.g. `12%`.
- **Dropdown:** per-limit breakdown with reset countdowns:
  - `5-hour session` - the 5-hour token limit
  - `Weekly (7-day)` - the weekly token limit (with a pace projection)
  - `Tools 5h` - the built-in tools quota (search / web-reader / zread)
- **Plan label:** from your z.ai level, e.g. `GLM Max`.
- **24h chart:** recent 5-hour-limit history.
- **Free-flush celebration:** when z.ai zeroes your weekly counter out of band, the menu
  bar throws a brief party until you start denting the fresh bucket again.

## Requirements

- macOS on Apple Silicon (the build targets `aarch64-apple-darwin`).
- The Rust toolchain (stable, `aarch64-apple-darwin`) and Xcode Command Line Tools
  (`swiftc`, `sips`, `codesign`). If `cargo` is not on PATH (common with Homebrew
  `rustup`, which ships no `~/.cargo/bin` proxies), add the toolchain bin to PATH,
  e.g. in `~/.zshenv`: `export PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH"`.
- A z.ai GLM Coding Plan subscription.
- Your z.ai token as `GLM_API_KEY` in `~/.hermes/.env` (the same file the `zai` shell
  launcher reads). Fallbacks: the `ANTHROPIC_AUTH_TOKEN` or `GLM_API_KEY` env vars.

## Build & install

```bash
# Lint + test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test

# Build the native .app (release Rust + Swift)
scripts/build-macos-app.sh

# Install and enable autostart at login
ditto target/aarch64-apple-darwin/release/ZaiMeter.app /Applications/ZaiMeter.app
scripts/install-macos-launchagent.sh
```

Then launch it: `open /Applications/ZaiMeter.app` (or log out/in - the LaunchAgent starts
it automatically).

## How it works

Two processes share one JSON file (`~/Library/Application Support/ZaiMeter/status.json`):

- The **Rust agent** (`zaimeter`, bundled as `zaimeter-agent`) reads your token, polls
  `https://api.z.ai/api/monitor/usage/quota/limit`, decodes the limits into gauges, writes
  `status.json`, and appends a snapshot to `zaimeter.db`.
- The **Swift menu-bar app** (`ZaiMeter`) spawns the agent and re-reads `status.json` every
  5 seconds to render the menu bar.

Config lives in `~/Library/Application Support/ZaiMeter/config.json`.

## Configuration

Edit `config.json` (or use "Open Config" in the dropdown) and the app hot-reloads it.

| field | default | notes |
|---|---|---|
| `provider` | `"zai"` | `"zai"` (z.ai GLM) or `"claude"` (Anthropic OAuth, the original backend) |
| `polling_interval_seconds` | `120` | minimum 60 |
| `celebrate_free_flush` | `true` | the weekly-flush party and its tunables |
| `plan_override` | `null` | Claude tier label; ignored for z.ai |

## Development

```bash
cargo run -- --once      # one poll, writes status.json, exits - quick data-layer check
cargo run -- --status    # print the current status.json
```

Source layout: `src/macos_app.rs` (agent), `macos/ZaiMeterApp.swift` (UI),
`src/providers/{mod,zai,claude}.rs` (backends), `src/credentials.rs` (token),
`src/config.rs`, `src/db.rs`. See `CLAUDE.md` for the full architecture and the verified
z.ai API contract.

## License

MIT. Based on klivak/ClaudeMeter.
