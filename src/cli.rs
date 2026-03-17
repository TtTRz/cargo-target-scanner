use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::scanner;
use crate::utils::format_size;

// ANSI color codes — aligned with WeUI official color specification
// --weui-BRAND: #07C160 (green), --weui-RED: #FA5151, --weui-ORANGE: #FA9D3B
// --weui-YELLOW: #FFC300, --weui-BLUE: #10AEFF, --weui-LINK: #576B95
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[38;2;7;193;96m"; // #07C160 --weui-BRAND
const YELLOW: &str = "\x1b[38;2;255;195;0m"; // #FFC300 --weui-YELLOW
const RED: &str = "\x1b[38;2;250;81;81m"; // #FA5151 --weui-RED
const CYAN: &str = "\x1b[38;2;16;174;255m"; // #10AEFF --weui-BLUE
const BOLD_GREEN: &str = "\x1b[1;38;2;7;193;96m"; // #07C160
const BOLD_YELLOW: &str = "\x1b[1;38;2;255;195;0m"; // #FFC300
const BOLD_RED: &str = "\x1b[1;38;2;250;81;81m"; // #FA5151
const BOLD_CYAN: &str = "\x1b[1;38;2;16;174;255m"; // #10AEFF
const BOLD_MAGENTA: &str = "\x1b[1;38;2;87;107;149m"; // #576B95 --weui-LINK
const ORANGE: &str = "\x1b[38;2;250;157;59m"; // #FA9D3B --weui-ORANGE
const BOLD_ORANGE: &str = "\x1b[1;38;2;250;157;59m"; // #FA9D3B

const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * MB;

/// Color a size string based on how large it is
fn colored_size(size: u64) -> String {
    let s = format_size(size);
    if size >= GB {
        format!("{BOLD_RED}{s}{RESET}")
    } else if size >= 500 * MB {
        format!("{BOLD_ORANGE}{s}{RESET}")
    } else if size >= 100 * MB {
        format!("{BOLD_YELLOW}{s}{RESET}")
    } else {
        format!("{GREEN}{s}{RESET}")
    }
}

/// Color for sub-target sizes (dimmer)
fn colored_sub_size(size: u64) -> String {
    let s = format_size(size);
    if size >= GB {
        format!("{RED}{s}{RESET}")
    } else if size >= 500 * MB {
        format!("{ORANGE}{s}{RESET}")
    } else if size >= 100 * MB {
        format!("{YELLOW}{s}{RESET}")
    } else {
        format!("{DIM}{s}{RESET}")
    }
}

pub fn run(path: Option<String>, delete_all: bool, delete: bool, sort_by_name: bool) {
    let scan_root = path
        .map(PathBuf::from)
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")));

    println!("{BOLD_CYAN}🦀 Cargo Target Scanner{RESET}");
    println!("{DIM}Scanning: {}{RESET}", scan_root.display());

    let mut projects = scan_with_progress(&scan_root);

    if projects.is_empty() {
        println!("{YELLOW}No Rust projects with target directories found.{RESET}");
        return;
    }

    if sort_by_name {
        projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    } else {
        projects.sort_by(|a, b| b.target_size.cmp(&a.target_size));
    }

    let total_size: u64 = projects.iter().map(|p| p.target_size).sum();

    println!();
    println!(
        "{BOLD}{:<4} {:<30} {:>12}  Path{RESET}",
        "#", "Project", "Size"
    );
    println!("{DIM}{}{RESET}", "─".repeat(90));

    for (i, p) in projects.iter().enumerate() {
        let size_str = colored_size(p.target_size);
        println!(
            "{BOLD}{:<4}{RESET} {CYAN}{:<30}{RESET} {:>24}  {DIM}{}{RESET}",
            i + 1,
            truncate(&p.name, 28),
            size_str,
            p.target_path.display()
        );
        for t in &p.build_targets {
            let t_size = colored_sub_size(t.size);
            println!("     {DIM}└──{RESET} {:<25} {:>24}", t.name, t_size);
        }
    }

    println!("{DIM}{}{RESET}", "─".repeat(90));
    println!(
        "  Found {BOLD}{}{RESET} projects, total target size: {}",
        projects.len(),
        colored_size(total_size)
    );
    println!();

    if !delete_all && !delete {
        println!(
            "{CYAN}Scan only. Use --delete for interactive deletion, --delete-all to remove all.{RESET}"
        );
        return;
    }

    if delete_all {
        println!("{BOLD_RED}Deleting ALL target directories...{RESET}");
        let mut deleted = 0u64;
        let mut count = 0usize;
        let mut errors = Vec::new();
        for p in &projects {
            print!("  {DIM}Deleting{RESET} {CYAN}{}{RESET} ... ", p.name);
            io::stdout().flush().ok();
            match std::fs::remove_dir_all(&p.target_path) {
                Ok(()) => {
                    println!("{GREEN}OK{RESET} ({})", colored_size(p.target_size));
                    deleted += p.target_size;
                    count += 1;
                }
                Err(e) => {
                    println!("{RED}FAILED: {}{RESET}", e);
                    errors.push(format!("{}: {}", p.name, e));
                }
            }
        }
        println!();
        println!(
            "  {BOLD_GREEN}✅ Deleted {} targets, freed {}{RESET}",
            count,
            colored_size(deleted)
        );
        if !errors.is_empty() {
            println!("{RED}{} errors: {}{RESET}", errors.len(), errors.join("; "));
        }
        return;
    }

    // Interactive mode
    print!(
        "{BOLD_MAGENTA}>{RESET} Enter project numbers to delete ({DIM}e.g. 1,3,5 or 1-5 or 'all'{RESET}), or {DIM}'q'{RESET} to quit: "
    );
    io::stdout().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("{RED}Failed to read input.{RESET}");
        return;
    }
    let input = input.trim();

    if input.eq_ignore_ascii_case("q") || input.is_empty() {
        println!("{DIM}Aborted.{RESET}");
        return;
    }

    let indices = parse_selection(input, projects.len());
    if indices.is_empty() {
        println!("{YELLOW}No valid selection.{RESET}");
        return;
    }

    let selected_size: u64 = indices.iter().map(|&i| projects[i].target_size).sum();
    print!(
        "\n  Will delete {BOLD}{}{RESET} targets ({}). Continue? [{BOLD_GREEN}y{RESET}/{RED}N{RESET}]: ",
        indices.len(),
        colored_size(selected_size)
    );
    io::stdout().flush().ok();

    let mut confirm = String::new();
    if io::stdin().read_line(&mut confirm).is_err() {
        println!("{RED}Failed to read input.{RESET}");
        return;
    }

    if !confirm.trim().eq_ignore_ascii_case("y") {
        println!("{DIM}Aborted.{RESET}");
        return;
    }

    let mut deleted = 0u64;
    let mut count = 0usize;
    let mut errors = Vec::new();
    for &i in &indices {
        let p = &projects[i];
        print!("  {DIM}Deleting{RESET} {CYAN}{}{RESET} ... ", p.name);
        io::stdout().flush().ok();
        match std::fs::remove_dir_all(&p.target_path) {
            Ok(()) => {
                println!("{GREEN}OK{RESET} ({})", colored_size(p.target_size));
                deleted += p.target_size;
                count += 1;
            }
            Err(e) => {
                println!("{RED}FAILED: {}{RESET}", e);
                errors.push(format!("{}: {}", p.name, e));
            }
        }
    }
    println!();
    println!(
        "  {BOLD_GREEN}✅ Deleted {} targets, freed {}{RESET}",
        count,
        colored_size(deleted)
    );
    if !errors.is_empty() {
        println!("{RED}{} errors: {}{RESET}", errors.len(), errors.join("; "));
    }
}

fn parse_selection(input: &str, max: usize) -> Vec<usize> {
    if input.eq_ignore_ascii_case("all") {
        return (0..max).collect();
    }

    let mut result = Vec::new();
    for part in input.split(',') {
        let part = part.trim();
        if let Some((start, end)) = part.split_once('-') {
            if let (Ok(s), Ok(e)) = (start.trim().parse::<usize>(), end.trim().parse::<usize>()) {
                for n in s..=e {
                    if n >= 1 && n <= max {
                        result.push(n - 1);
                    }
                }
            }
        } else if let Ok(n) = part.parse::<usize>()
            && n >= 1
            && n <= max
        {
            result.push(n - 1);
        }
    }
    result.sort();
    result.dedup();
    result
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len - 1])
    }
}

fn scan_with_progress(scan_root: &Path) -> Vec<crate::model::ProjectInfo> {
    let results = Arc::new(Mutex::new(Vec::<crate::model::ProjectInfo>::new()));
    let done_flag = Arc::new(AtomicBool::new(false));
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let skip_dirs = scanner::default_skip_dirs();

    let results_clone = results.clone();
    let done_clone = done_flag.clone();
    let cancel_clone = cancel_flag.clone();
    let root = scan_root.to_path_buf();

    std::thread::spawn(move || {
        scanner::scan_directory_collect(
            &root,
            &results_clone,
            &done_clone,
            &cancel_clone,
            &skip_dirs,
        );
    });

    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let mut spinner_idx = 0usize;
    let start = Instant::now();

    loop {
        let count = results.lock().map(|r| r.len()).unwrap_or(0);
        let total_size: u64 = results
            .lock()
            .map(|r| r.iter().map(|p| p.target_size).sum())
            .unwrap_or(0);
        let elapsed = start.elapsed().as_secs_f64();
        let ch = spinner_chars[spinner_idx % spinner_chars.len()];

        print!(
            "\r  {CYAN}{ch}{RESET} Scanning... found {BOLD}{count}{RESET} projects ({}) {DIM}[{elapsed:.1}s]{RESET}    ",
            colored_size(total_size)
        );
        io::stdout().flush().ok();

        spinner_idx += 1;

        if done_flag.load(Ordering::SeqCst) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(80));
    }

    let final_results = match Arc::try_unwrap(results) {
        Ok(mutex) => mutex.into_inner().unwrap(),
        Err(arc) => arc.lock().unwrap().clone(),
    };

    let elapsed = start.elapsed().as_secs_f64();
    println!(
        "\r  {BOLD_GREEN}✅ Scan complete:{RESET} {BOLD}{}{RESET} projects ({}) in {DIM}{:.1}s{RESET}          ",
        final_results.len(),
        colored_size(final_results.iter().map(|p| p.target_size).sum()),
        elapsed
    );

    final_results
}
