## 🥷 Project: `nunchucks`

**Tagline:** *Strike fast. Run commands. Stay deadly.*

A cross-platform global hotkey launcher that maps key combos to actions like shell scripts, webhooks, sound effects, or animations.

---

## ✨ Core Concept

A "chuck" = a hotkey + an action. You define these in a config. When the combo is pressed, `nunchucks` strikes.

Example config (`~/.nunchucks.toml`):

```toml
# Global settings
[settings]
log_level = "info" # debug, info, warn, error
daemon_mode = true
notification_timeout = 2000 # ms

[chucks.launch_server]
keys = ["Cmd+Shift+L"]
command = "cd ~/dev/myapp && ./start.sh"
sound = "sfx/hii-ya.wav"
ascii = "ascii/flame.txt"
notification = "🚀 Server launched!"
timeout = 5000 # ms

[chucks.deploy]
keys = ["Ctrl+Alt+D"]
webhook = "https://my-deploy-hook.netlify.app"
sound = "sfx/swoosh.wav"
notification = "🚀 Deploying..."
retry_count = 3
```

---

## 🧰 Features (v1)

| Feature | Notes |
| ----------------------- | ------------------------------------------------------------- |
| Global hotkey listener | Platform-specific (start with macOS/Linux via `device_query`) |
| Config file (TOML) | Easy to parse + human-readable |
| Run shell commands | Use `std::process::Command` safely |
| Trigger webhooks | Use `ureq` or `reqwest` |
| Play sound effects | Use `rodio` or `cpal` |
| ASCII animation support | Print fun visuals to terminal or overlay on trigger |
| Daemon mode | Background listener with optional logging |
| `nunchucks test` | Simulate and debug config actions from terminal |
| Error handling | Graceful fallbacks + user notifications |
| Config validation | Schema validation + runtime checks |
| Notifications | Native OS notifications for action status |
| Timeout handling | Configurable timeouts for long-running actions |
| Retry logic | Automatic retries for failed webhooks/commands |

---

## 🔮 Stretch Goals (v2+)

* `--learn` mode: watches what commands you run frequently and suggests automations
* Plugin system (e.g. Lua or WASM-based)
* TUI mode (`nunchucks dash`) to browse and trigger chucks manually
* Remote-trigger mode via socket or HTTP
* "Combo chaining" - press one combo, run multiple actions in sequence
* Action history and statistics
* Conditional execution based on system state
* Hot reload of configuration
* Action templates and variables
* Cross-platform clipboard integration
* Action scheduling and timers

---

## 🧪 Suggested Crates

| Purpose | Crate |
| ------------------- | ---------------------------------------------------------------------------------- |
| Global key listener | [`device_query`](https://crates.io/crates/device_query) or `winit` (fallback) |
| Config parsing | [`toml`](https://crates.io/crates/toml), [`serde`](https://crates.io/crates/serde) |
| Shell execution | `std::process`, maybe `duct` |
| Sound playback | [`rodio`](https://crates.io/crates/rodio) |
| ASCII output | Custom or via `termion`, `crossterm`, `ratatui` |
| Logging | `tracing`, `log`, or just println! for fun mode |
| Notifications | [`notify-rust`](https://crates.io/crates/notify-rust) |
| Error handling | [`anyhow`](https://crates.io/crates/anyhow), [`thiserror`](https://crates.io/crates/thiserror) |
| Async runtime | [`tokio`](https://crates.io/crates/tokio) |
| Validation | [`validator`](https://crates.io/crates/validator) |

---

## 🛠 Project Scaffold

```
nunchucks/
├── Cargo.toml
├── src/
│ ├── main.rs
│ ├── config.rs # Loads and parses .nunchucks.toml
│ ├── hotkeys.rs # Keybinding registration and dispatch
│ ├── actions.rs # Command/webhook/sound handling
│ ├── ascii.rs # Optional ASCII playback
│ ├── audio.rs # Sound trigger logic
│ ├── error.rs # Custom error types and handling
│ ├── notification.rs # OS notification handling
│ └── validation.rs # Config validation logic
├── assets/
│ ├── sfx/
│ └── ascii/
└── tests/
 ├── integration/
 └── unit/
```

---

## 🚨 Error Handling

* Graceful degradation when features aren't available
* Clear error messages with actionable feedback
* Logging with different verbosity levels
* Automatic retry for transient failures
* User notifications for critical errors

---

## 🔍 Configuration Validation

* Schema validation for TOML structure
* Runtime checks for:
 * Valid key combinations
 * Existing sound/ASCII files
 * Valid webhook URLs
 * Command existence
 * Permission checks
* Hot reload with validation
* Config migration helpers

---

## 📝 Development Guidelines

* Use async/await for I/O operations
* Implement proper error propagation
* Write comprehensive tests
* Document public APIs
* Follow Rust best practices
* Use semantic versioning
* Maintain changelog
