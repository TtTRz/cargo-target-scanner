# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-03-05

### Added

- Recursive scanning for Rust projects with `target` directories
- Size visualization with color-coded indicators
- Build target details (debug, release, cross-compilation targets)
- Selective deletion of whole projects or individual build targets
- Async deletion with loading overlay
- Toast notifications for success/failure
- Search and sort (by size or name)
- i18n support (Chinese / English)
- WeUI-style design
- CLI defaults to scan-only mode; use `--delete` for interactive deletion
- CLI colorized output with size-based color coding (green/yellow/orange/red)
- CLI spinner animation during scanning with real-time progress
- GUI scanning overlay with animated progress (project count, size, elapsed time)
- `--gui` flag to launch GUI mode (CLI is now the default)
