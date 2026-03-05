use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::i18n::{I18n, Language};
use crate::model::{ProjectInfo, SortBy, SortOrder};
use crate::scanner;
use crate::utils::format_size;

pub struct AppState {
    pub projects: Vec<ProjectInfo>,
    pub scanning: bool,
    pub deleting: bool,
    pub scan_root: String,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub search_filter: String,
    pub status_message: Option<String>,
    pub show_delete_confirm: bool,
    pub toast: Option<Toast>,
    pub language: Language,
    pub scan_found_count: usize,
    pub scan_found_size: u64,
    pub scan_elapsed_secs: f64,
    shared_results: Option<Arc<Mutex<Vec<ProjectInfo>>>>,
    scan_done: Option<Arc<AtomicBool>>,
    scan_start_time: Option<std::time::Instant>,
}

#[derive(Clone, Debug)]
pub struct Toast {
    pub message: String,
    pub is_error: bool,
}

impl AppState {
    pub fn new() -> Self {
        let home = dirs::home_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "/".to_string());
        Self {
            projects: Vec::new(),
            scanning: false,
            deleting: false,
            scan_root: home,
            sort_by: SortBy::Size,
            sort_order: SortOrder::Desc,
            search_filter: String::new(),
            status_message: None,
            show_delete_confirm: false,
            toast: None,
            language: Language::Zh,
            scan_found_count: 0,
            scan_found_size: 0,
            scan_elapsed_secs: 0.0,
            shared_results: None,
            scan_done: None,
            scan_start_time: None,
        }
    }

    pub fn start_scan(&mut self) {
        self.projects.clear();
        self.scanning = true;
        self.status_message = None;
        self.show_delete_confirm = false;
        self.scan_found_count = 0;
        self.scan_found_size = 0;
        self.scan_elapsed_secs = 0.0;
        self.scan_start_time = Some(std::time::Instant::now());

        let results = Arc::new(Mutex::new(Vec::<ProjectInfo>::new()));
        let done_flag = Arc::new(AtomicBool::new(false));

        self.shared_results = Some(results.clone());
        self.scan_done = Some(done_flag.clone());

        let root = self.scan_root.clone();

        thread::spawn(move || {
            let skip_dirs = scanner::default_skip_dirs();
            let root_path = std::path::PathBuf::from(&root);
            scanner::scan_directory_collect(&root_path, &results, &done_flag, &skip_dirs);
        });
    }

    pub fn poll_results(&mut self) -> bool {
        let mut changed = false;

        if let Some(results) = &self.shared_results {
            if let Ok(mut list) = results.lock() {
                if !list.is_empty() {
                    self.projects.append(&mut *list);
                    changed = true;
                }
            }
        }

        if let Some(done) = &self.scan_done {
            if done.load(Ordering::SeqCst) {
                if let Some(results) = &self.shared_results {
                    if let Ok(mut list) = results.lock() {
                        if !list.is_empty() {
                            self.projects.append(&mut *list);
                        }
                    }
                }
                self.scanning = false;
                self.shared_results = None;
                self.scan_done = None;
                self.scan_start_time = None;
                changed = true;
            }
        }

        if changed {
            self.sort_projects();
        }

        // Update scan progress stats
        self.scan_found_count = self.projects.len();
        self.scan_found_size = self.projects.iter().map(|p| p.target_size).sum();
        if let Some(start) = self.scan_start_time {
            self.scan_elapsed_secs = start.elapsed().as_secs_f64();
        }

        changed
    }

    pub fn sort_projects(&mut self) {
        match (self.sort_by, self.sort_order) {
            (SortBy::Name, SortOrder::Asc) => {
                self.projects
                    .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }
            (SortBy::Name, SortOrder::Desc) => {
                self.projects
                    .sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
            }
            (SortBy::Size, SortOrder::Asc) => {
                self.projects
                    .sort_by(|a, b| a.target_size.cmp(&b.target_size));
            }
            (SortBy::Size, SortOrder::Desc) => {
                self.projects
                    .sort_by(|a, b| b.target_size.cmp(&a.target_size));
            }
        }
    }

    pub fn total_size(&self) -> u64 {
        self.projects.iter().map(|p| p.target_size).sum()
    }

    pub fn selected_count(&self) -> usize {
        self.projects.iter().filter(|p| p.selected).count()
    }

    /// Count how many individual build targets are selected across all projects
    pub fn selected_targets_count(&self) -> usize {
        self.projects
            .iter()
            .flat_map(|p| &p.build_targets)
            .filter(|t| t.selected)
            .count()
    }

    /// Whether there are any selected items (whole projects or individual targets)
    pub fn has_any_selection(&self) -> bool {
        self.selected_count() > 0 || self.selected_targets_count() > 0
    }

    /// Total deletion size (whole selected projects + individual selected targets from non-selected projects)
    pub fn total_delete_size(&self) -> u64 {
        let whole_project_size: u64 = self
            .projects
            .iter()
            .filter(|p| p.selected)
            .map(|p| p.target_size)
            .sum();
        let individual_target_size: u64 = self
            .projects
            .iter()
            .filter(|p| !p.selected)
            .flat_map(|p| &p.build_targets)
            .filter(|t| t.selected)
            .map(|t| t.size)
            .sum();
        whole_project_size + individual_target_size
    }

    /// Total deletion count description
    pub fn delete_description(&self) -> String {
        let whole = self.selected_count();
        let targets = self
            .projects
            .iter()
            .filter(|p| !p.selected)
            .flat_map(|p| &p.build_targets)
            .filter(|t| t.selected)
            .count();

        let lang = self.language;
        let mut parts = Vec::new();
        if whole > 0 {
            parts.push(I18n::target_dirs_unit(lang, whole));
        }
        if targets > 0 {
            parts.push(I18n::build_targets_unit(lang, targets));
        }
        parts.join(" + ")
    }

    pub fn toggle_select_all(&mut self) {
        let all_selected = self.projects.iter().all(|p| p.selected);
        for p in &mut self.projects {
            p.selected = !all_selected;
            // When selecting whole project, deselect individual targets
            if p.selected {
                for t in &mut p.build_targets {
                    t.selected = false;
                }
            }
        }
    }

    pub fn toggle_build_target(&mut self, project_idx: usize, target_idx: usize) {
        if let Some(project) = self.projects.get_mut(project_idx) {
            if let Some(target) = project.build_targets.get_mut(target_idx) {
                target.selected = !target.selected;
            }
            // If all targets selected, select the whole project instead
            let all_targets_selected = project.build_targets.iter().all(|t| t.selected);
            if all_targets_selected && !project.build_targets.is_empty() {
                project.selected = true;
                for t in &mut project.build_targets {
                    t.selected = false;
                }
            }
        }
    }

    /// Collect delete tasks: returns (whole_project_indices, individual_targets: Vec<(proj_idx, target_idx, path, name, size)>)
    pub fn collect_delete_tasks(&self) -> DeleteTasks {
        let mut whole_projects = Vec::new();
        let mut individual_targets = Vec::new();

        for (i, project) in self.projects.iter().enumerate() {
            if project.selected {
                whole_projects.push(DeleteWholeProject {
                    name: project.name.clone(),
                    path: project.target_path.clone(),
                    size: project.target_size,
                });
            } else {
                for (ti, target) in project.build_targets.iter().enumerate() {
                    if target.selected {
                        individual_targets.push(DeleteSingleTarget {
                            project_index: i,
                            target_index: ti,
                            project_name: project.name.clone(),
                            target_name: target.name.clone(),
                            path: target.path.clone(),
                            size: target.size,
                        });
                    }
                }
            }
        }

        DeleteTasks {
            whole_projects,
            individual_targets,
        }
    }

    /// Apply delete results back to state
    pub fn apply_delete_results(&mut self, result: DeleteResult) {
        // Remove successfully deleted individual targets (reverse order for index stability)
        let mut targets_by_project: std::collections::HashMap<usize, Vec<usize>> =
            std::collections::HashMap::new();
        for ti in &result.deleted_target_indices {
            targets_by_project
                .entry(ti.0)
                .or_default()
                .push(ti.1);
        }
        for (proj_idx, mut target_indices) in targets_by_project {
            target_indices.sort_unstable_by(|a, b| b.cmp(a));
            if let Some(project) = self.projects.get_mut(proj_idx) {
                for ti in target_indices {
                    if ti < project.build_targets.len() {
                        project.build_targets.remove(ti);
                    }
                }
                project.target_size = scanner::calc_dir_size(&project.target_path);
            }
        }

        // Remove whole-deleted projects
        self.projects.retain(|p| {
            if p.selected {
                p.target_path.exists()
                    && p.target_path
                        .read_dir()
                        .is_ok_and(|mut d| d.next().is_some())
            } else {
                true
            }
        });

        self.deleting = false;
        self.show_delete_confirm = false;

        let has_errors = !result.errors.is_empty();
        let lang = self.language;
        let message = if result.errors.is_empty() {
            I18n::delete_success(lang, result.deleted_count, &format_size(result.deleted_size))
        } else {
            I18n::delete_partial(
                lang,
                result.deleted_count,
                result.errors.len(),
                &result.errors.join("; "),
            )
        };

        self.toast = Some(Toast {
            message,
            is_error: has_errors,
        });
    }

    pub fn filtered_indices(&self) -> Vec<usize> {
        let filter = self.search_filter.to_lowercase();
        self.projects
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                if filter.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&filter)
                        || p.path.display().to_string().to_lowercase().contains(&filter)
                }
            })
            .map(|(i, _)| i)
            .collect()
    }
}

// Delete task data structures
pub struct DeleteTasks {
    pub whole_projects: Vec<DeleteWholeProject>,
    pub individual_targets: Vec<DeleteSingleTarget>,
}

pub struct DeleteWholeProject {
    pub name: String,
    pub path: std::path::PathBuf,
    pub size: u64,
}

pub struct DeleteSingleTarget {
    pub project_index: usize,
    pub target_index: usize,
    pub project_name: String,
    pub target_name: String,
    pub path: std::path::PathBuf,
    pub size: u64,
}

pub struct DeleteResult {
    pub deleted_count: usize,
    pub deleted_size: u64,
    pub deleted_target_indices: Vec<(usize, usize)>,
    pub errors: Vec<String>,
}

/// Execute deletion in a blocking context (call from a spawned thread)
pub fn execute_delete(tasks: DeleteTasks) -> DeleteResult {
    let mut deleted_size: u64 = 0;
    let mut deleted_count: usize = 0;
    let mut errors: Vec<String> = Vec::new();
    let mut deleted_target_indices: Vec<(usize, usize)> = Vec::new();

    // Delete whole target directories
    for wp in &tasks.whole_projects {
        match fs::remove_dir_all(&wp.path) {
            Ok(()) => {
                deleted_size += wp.size;
                deleted_count += 1;
            }
            Err(e) => {
                errors.push(format!("{}: {}", wp.name, e));
            }
        }
    }

    // Delete individual build targets
    for st in &tasks.individual_targets {
        match fs::remove_dir_all(&st.path) {
            Ok(()) => {
                deleted_size += st.size;
                deleted_count += 1;
                deleted_target_indices.push((st.project_index, st.target_index));
            }
            Err(e) => {
                errors.push(format!("{}/{}: {}", st.project_name, st.target_name, e));
            }
        }
    }

    DeleteResult {
        deleted_count,
        deleted_size,
        deleted_target_indices,
        errors,
    }
}
