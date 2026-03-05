# 🦀 Cargo Target Scanner

A desktop GUI tool to scan, visualize, and clean up Rust project `target` directories, helping you reclaim disk space.

## Features

- **Recursive Scanning** — Recursively scan a directory tree to find all Rust projects with `target` folders
- **Size Visualization** — Display each project's `target` directory size with color-coded indicators (green → red)
- **Build Target Details** — Show individual build targets (`debug`, `release`, cross-compilation targets) with sizes
- **Selective Deletion** — Select whole projects or individual build targets for deletion
- **Async Deletion** — Delete operations run in a background thread with a loading overlay, keeping the UI responsive
- **Toast Notifications** — Success/failure toast messages after deletion
- **Search & Sort** — Filter projects by name or path; sort by size or name (asc/desc)
- **i18n** — Supports Chinese and English, switchable at runtime (default: Chinese)
- **WeUI Design** — Clean, modern UI following WeChat WeUI design guidelines

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Dioxus](https://dioxuslabs.com/) 0.7 (Desktop)
- [rfd](https://github.com/PolyMeilex/rfd) — Native file dialog
- [Tokio](https://tokio.rs/) — Async runtime

## Requirements

- Rust 2024 edition (1.85+)
- macOS / Linux / Windows

## Build & Run

```bash
git clone https://github.com/your-username/cargo-target-scanner.git
cd cargo-target-scanner

# Run in development mode
cargo run

# Build release binary
cargo build --release
```

## Usage

1. Set the scan root directory (defaults to `$HOME`)
2. Click **Start Scan** to find all Rust projects with `target` directories
3. Browse projects sorted by size; use the search bar to filter
4. Select projects or individual build targets (debug / release / cross-compilation)
5. Click **Delete** and confirm to clean up selected targets
6. Click the 🌐 button in the top-right corner to switch between Chinese and English

## License

MIT
