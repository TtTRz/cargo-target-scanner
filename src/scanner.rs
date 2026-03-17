use std::collections::HashSet;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::model::{BuildTarget, ProjectInfo};

pub fn scan_directory_collect(
    dir: &Path,
    results: &Arc<Mutex<Vec<ProjectInfo>>>,
    done_flag: &Arc<AtomicBool>,
    cancel_flag: &Arc<AtomicBool>,
    skip_dirs: &HashSet<&str>,
) {
    scan_recursive(dir, results, cancel_flag, skip_dirs);
    done_flag.store(true, Ordering::SeqCst);
}

fn scan_recursive(
    dir: &Path,
    results: &Arc<Mutex<Vec<ProjectInfo>>>,
    cancel_flag: &Arc<AtomicBool>,
    skip_dirs: &HashSet<&str>,
) {
    if cancel_flag.load(Ordering::Relaxed) {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    let mut has_cargo_toml = false;
    let mut subdirs = Vec::new();

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name == "Cargo.toml"
            && let Ok(ft) = entry.file_type()
            && ft.is_file()
        {
            has_cargo_toml = true;
        }

        if let Ok(ft) = entry.file_type()
            && ft.is_dir()
            && !name.starts_with('.')
            && !skip_dirs.contains(name.as_ref())
        {
            subdirs.push(entry.path());
        }
    }

    if has_cargo_toml {
        let target_path = dir.join("target");
        if target_path.is_dir() {
            let target_size = calc_dir_size(&target_path);
            if target_size > 0 {
                let name = extract_project_name(dir);
                let build_targets = find_build_targets(&target_path);

                let info = ProjectInfo {
                    name,
                    path: dir.to_path_buf(),
                    target_path,
                    target_size,
                    build_targets,
                    selected: false,
                };

                if let Ok(mut list) = results.lock() {
                    list.push(info);
                }
            }
        }

        for subdir in &subdirs {
            let dir_name = subdir
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            if dir_name != "target" {
                scan_recursive(subdir, results, cancel_flag, skip_dirs);
            }
        }
    } else {
        for subdir in &subdirs {
            scan_recursive(subdir, results, cancel_flag, skip_dirs);
        }
    }
}

pub fn default_skip_dirs() -> HashSet<&'static str> {
    [
        "node_modules",
        ".git",
        "Library",
        ".Trash",
        "Applications",
        ".cache",
        ".npm",
        ".rustup",
        ".cargo",
        "Pictures",
        "Music",
        "Movies",
    ]
    .into_iter()
    .collect()
}

fn extract_project_name(project_dir: &Path) -> String {
    let cargo_toml = project_dir.join("Cargo.toml");
    if let Ok(content) = fs::read_to_string(&cargo_toml) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name")
                && let Some(val) = trimmed.split('=').nth(1)
            {
                let name = val.trim().trim_matches('"').trim_matches('\'');
                if !name.is_empty() {
                    return name.to_string();
                }
            }
        }
    }
    project_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn find_build_targets(target_dir: &Path) -> Vec<BuildTarget> {
    let mut targets = Vec::new();

    if let Ok(entries) = fs::read_dir(target_dir) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type()
                && ft.is_dir()
            {
                let name = entry.file_name().to_string_lossy().to_string();
                let entry_path = entry.path();

                if matches!(name.as_str(), "debug" | "release") {
                    let size = calc_dir_size(&entry_path);
                    targets.push(BuildTarget {
                        name,
                        path: entry_path,
                        size,
                        selected: false,
                    });
                    continue;
                }
                // Cross-compilation targets like aarch64-apple-darwin
                if name.contains('-') && !name.starts_with('.') && name != "tmp" {
                    let has_profile =
                        entry_path.join("debug").is_dir() || entry_path.join("release").is_dir();
                    if has_profile {
                        let size = calc_dir_size(&entry_path);
                        targets.push(BuildTarget {
                            name,
                            path: entry_path,
                            size,
                            selected: false,
                        });
                    }
                }
            }
        }
    }

    targets.sort_by(|a, b| b.size.cmp(&a.size));
    targets
}

pub fn calc_dir_size(path: &Path) -> u64 {
    #[cfg(unix)]
    {
        let mut seen_inodes: HashSet<(u64, u64)> = HashSet::new();
        dir_size(path, &mut seen_inodes)
    }
    #[cfg(not(unix))]
    {
        dir_size(path)
    }
}

#[cfg(unix)]
fn dir_size(path: &Path, seen_inodes: &mut HashSet<(u64, u64)>) -> u64 {
    let mut total: u64 = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            // Use symlink_metadata to avoid following symlinks
            let meta = match entry.path().symlink_metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            // Skip symlinks entirely
            if meta.file_type().is_symlink() {
                continue;
            }

            if meta.is_dir() {
                total += dir_size(&entry.path(), seen_inodes);
            } else {
                // Deduplicate hardlinks: skip if we've already counted this inode
                let inode_key = (meta.dev(), meta.ino());
                if meta.nlink() > 1 && !seen_inodes.insert(inode_key) {
                    continue; // Already counted
                }
                total += meta.len();
            }
        }
    }

    total
}

#[cfg(not(unix))]
fn dir_size(path: &Path) -> u64 {
    let mut total: u64 = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let meta = match entry.path().symlink_metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            if meta.file_type().is_symlink() {
                continue;
            }

            if meta.is_dir() {
                total += dir_size(&entry.path());
            } else {
                total += meta.len();
            }
        }
    }

    total
}
