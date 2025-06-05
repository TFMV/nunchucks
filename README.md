# 🥷 Nunchucks

> *Strike fast. Run commands. Stay deadly.*

A cross-platform global hotkey launcher that maps key combos to actions like shell scripts, webhooks, sound effects, or animations. Think of it as your digital nunchucks - quick, deadly, and always ready to strike!

## 🎯 Features

- **Global Hotkeys**: Map any key combination to actions
- **Shell Commands**: Execute commands with style
- **Webhooks**: Trigger remote actions
- **Sound Effects**: Add audio feedback (optional "HII-YA!")
- **ASCII Art**: Display cool animations
- **Notifications**: Get visual feedback
- **Retry Logic**: Keep trying until you succeed
- **Daemon Mode**: Run silently in the background

## 🚀 Quick Start

1. Install Rust (if you haven't already):

 ```bash
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```

2. Clone and build:

 ```bash
 git clone https://github.com/TFMV/nunchucks.git
 cd nunchucks
 cargo build --release
 ```

3. Create your config at `~/.nunchucks.toml`:

 ```toml
 [settings]
 log_level = "info"
 daemon_mode = true
 notification_timeout = 2000

 [chucks.deploy]
 keys = ["Cmd+Shift+D"]
 command = "cd ~/myapp && ./deploy.sh"
 notification = "🚀 Deploying..."
 sound = "assets/sfx/swoosh.wav"
 ascii = "assets/ascii/ninja.txt"
 ```

4. Run it:

 ```bash
 ./target/release/nunchucks
 ```

## 🎨 Example Configurations

### Launch Server

```toml
[chucks.launch_server]
keys = ["Cmd+Shift+L"]
command = "cd ~/dev/myapp && ./start.sh"
sound = "sfx/hii-ya.wav"
ascii = "ascii/flame.txt"
notification = "🔥 Server launched!"
```
