# 🦀 Cargo Target Scanner

A tool to scan, visualize, and clean up Rust project `target` directories, helping you reclaim disk space. Supports both **GUI** and **CLI** modes.

## Features

- **Dual Mode** — CLI by default; use `--gui` to launch the desktop GUI
- **Recursive Scanning** — Recursively scan a directory tree to find all Rust projects with `target` folders
- **Size Visualization** — Display each project's `target` directory size with color-coded indicators (green → red)
- **Build Target Details** — Show individual build targets (`debug`, `release`, cross-compilation targets) with sizes
- **Selective Deletion** — Select whole projects or individual build targets for deletion
- **Async Deletion** — Delete operations run in a background thread with a loading overlay (GUI)
- **Toast Notifications** — Success/failure toast messages after deletion (GUI)
- **Search & Sort** — Filter projects by name or path; sort by size or name
- **i18n** — Supports Chinese and English, switchable at runtime (GUI, default: Chinese)
- **WeUI Design** — Clean, modern UI following WeChat WeUI design guidelines (GUI)

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Dioxus](https://dioxuslabs.com/) 0.7 (Desktop)
- [clap](https://github.com/clap-rs/clap) — Command-line argument parsing
- [rfd](https://github.com/PolyMeilex/rfd) — Native file dialog
- [Tokio](https://tokio.rs/) — Async runtime

## Requirements

- Rust 2024 edition (1.85+)
- macOS / Linux / Windows

## Installation

### From crates.io

```bash
cargo install cargo-target-scanner
```

### From source

```bash
git clone https://github.com/TtTRz/cargo-target-scanner.git
cd cargo-target-scanner
cargo build --release
```

## Usage

### CLI Mode (default)

By default, the tool runs in CLI mode and **only scans** (no deletion).

```bash
# Scan home directory (scan only)
cargo run

# Scan a specific path
cargo run -- --path /path/to/projects

# Scan and interactively select targets to delete
cargo run -- --delete

# Delete all targets without prompting
cargo run -- --delete-all

# Sort by name instead of size
cargo run -- --sort-name
```

### GUI Mode

```bash
cargo run -- --gui
# or
./target/release/cargo-target-scanner --gui
```

1. Set the scan root directory (defaults to `$HOME`)
2. Click **Start Scan** to find all Rust projects with `target` directories
3. Browse projects sorted by size; use the search bar to filter
4. Select projects or individual build targets (debug / release / cross-compilation)
5. Click **Delete** and confirm to clean up selected targets
6. Click the 🌐 button in the top-right corner to switch between Chinese and English

### Options

| Flag | Description |
|------|-------------|
| `--gui` | Launch GUI mode |
| `-p, --path <PATH>` | Scan root path (defaults to `$HOME`) |
| `--delete` | Enable interactive deletion (CLI) |
| `--delete-all` | Delete all found targets without prompting (CLI) |
| `--sort-name` | Sort by name instead of size (CLI) |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

## License

MIT
