# mprobe

A beautiful terminal-based system monitor written in Rust, inspired by [btop](https://github.com/aristocratos/btop).

## Features

- **CPU Monitoring** - Real-time CPU usage with per-core breakdown and historical graphs
- **Memory Monitoring** - RAM and swap usage with visual progress bars and history
- **Network Monitoring** - Upload/download speeds with live graphs
- **Process Management** - Full process list with filtering, sorting, and tree view
- **Modern UI** - Clean, dark theme with color-coded usage levels

## Installation

### From source

```bash
git clone https://github.com/darkdenlion/mprobe.git
cd mprobe
cargo build --release
```

The binary will be available at `target/release/mprobe`.

## Usage

```bash
./target/release/mprobe
# or if installed to PATH
mprobe
```

## Keybindings

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `Tab` / `Shift+Tab` | Switch between tabs |
| `j` / `Down` | Scroll down in process list |
| `k` / `Up` | Scroll up in process list |
| `g` | Go to top of process list |
| `G` | Go to bottom of process list |
| `/` | Toggle filter mode |
| `Esc` | Clear filter |
| `t` | Toggle tree view |
| `s` | Cycle sort column (PID, Name, CPU, Memory) |
| `r` | Reverse sort order |

## Dependencies

- [ratatui](https://github.com/ratatui/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information gathering
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime

## Requirements

- Rust 1.70 or later
- A terminal with Unicode support

## License

MIT
