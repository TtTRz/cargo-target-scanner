mod app;
mod model;
mod scanner;
mod ui;
mod utils;

use dioxus::prelude::*;

use crate::app::{AppState, execute_delete};
use crate::model::{SortBy, SortOrder};
use crate::ui::{BottomPanel, ProjectList, TopPanel, STYLESHEET};

fn main() {
    LaunchBuilder::desktop().launch(App);
}

#[component]
fn App() -> Element {
    let mut state = use_signal(|| AppState::new());

    let s = state.read();
    let projects = s.projects.clone();
    let filtered = s.filtered_indices();
    let scanning = s.scanning;
    let deleting = s.deleting;
    let scan_root = s.scan_root.clone();
    let search_filter = s.search_filter.clone();
    let sort_by = s.sort_by;
    let sort_order = s.sort_order;
    let total_size = s.total_size();
    let has_selection = s.has_any_selection();
    let delete_size = s.total_delete_size();
    let delete_desc = s.delete_description();
    let status_message = s.status_message.clone();
    let show_confirm = s.show_delete_confirm;
    let project_count = s.projects.len();
    let toast = s.toast.clone();
    drop(s);

    rsx! {
        style { "{STYLESHEET}" }
        div { class: "app",
            TopPanel {
                scan_root: scan_root,
                scanning: scanning,
                project_count: project_count,
                on_scan_root_change: move |val: String| {
                    state.write().scan_root = val;
                },
                on_start_scan: move |_| {
                    state.write().start_scan();
                    spawn(async move {
                        loop {
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                            state.write().poll_results();
                            if !state.read().scanning {
                                break;
                            }
                        }
                    });
                },
            }
            ProjectList {
                projects: projects,
                filtered_indices: filtered,
                scanning: scanning,
                search_filter: search_filter,
                sort_by: sort_by,
                sort_order: sort_order,
                on_search_change: move |val: String| {
                    state.write().search_filter = val;
                },
                on_sort_change: move |new_sort: SortBy| {
                    let mut s = state.write();
                    if s.sort_by == new_sort {
                        s.sort_order = match s.sort_order {
                            SortOrder::Asc => SortOrder::Desc,
                            SortOrder::Desc => SortOrder::Asc,
                        };
                    } else {
                        s.sort_by = new_sort;
                        s.sort_order = if new_sort == SortBy::Size {
                            SortOrder::Desc
                        } else {
                            SortOrder::Asc
                        };
                    }
                    s.sort_projects();
                },
                on_toggle_select: move |idx: usize| {
                    let mut s = state.write();
                    if let Some(p) = s.projects.get_mut(idx) {
                        p.selected = !p.selected;
                        if p.selected {
                            for t in &mut p.build_targets {
                                t.selected = false;
                            }
                        }
                    }
                },
                on_toggle_target: move |(proj_idx, target_idx): (usize, usize)| {
                    state.write().toggle_build_target(proj_idx, target_idx);
                },
            }
            BottomPanel {
                total_size: total_size,
                has_selection: has_selection,
                delete_size: delete_size,
                delete_desc: delete_desc,
                status_message: status_message,
                show_confirm: show_confirm,
                deleting: deleting,
                on_select_all: move |_| {
                    state.write().toggle_select_all();
                },
                on_delete_click: move |_| {
                    state.write().show_delete_confirm = true;
                },
                on_confirm_delete: move |_| {
                    state.write().deleting = true;
                    state.write().show_delete_confirm = false;
                    let tasks = state.read().collect_delete_tasks();
                    spawn(async move {
                        let result = tokio::task::spawn_blocking(move || {
                            execute_delete(tasks)
                        }).await.unwrap();
                        state.write().apply_delete_results(result);
                        // Auto-dismiss toast after 3 seconds
                        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                        state.write().toast = None;
                    });
                },
                on_cancel_delete: move |_| {
                    state.write().show_delete_confirm = false;
                },
            }
            // Loading overlay during deletion
            if deleting {
                div { class: "loading-overlay",
                    div { class: "loading-content",
                        div { class: "loading-spinner" }
                        div { class: "loading-text", "正在删除中，请稍候..." }
                    }
                }
            }
            // Toast notification
            if let Some(t) = &toast {
                div {
                    class: if t.is_error { "toast toast-error" } else { "toast toast-success" },
                    onclick: move |_| { state.write().toast = None; },
                    if t.is_error { "❌ " } else { "✅ " }
                    "{t.message}"
                }
            }
        }
    }
}
