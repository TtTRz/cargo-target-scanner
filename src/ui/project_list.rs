use dioxus::prelude::*;

use crate::model::{ProjectInfo, SortBy, SortOrder};
use crate::utils::{format_size, size_color_class};

#[component]
pub fn ProjectList(
    projects: Vec<ProjectInfo>,
    filtered_indices: Vec<usize>,
    scanning: bool,
    search_filter: String,
    sort_by: SortBy,
    sort_order: SortOrder,
    on_search_change: EventHandler<String>,
    on_sort_change: EventHandler<SortBy>,
    on_toggle_select: EventHandler<usize>,
    on_toggle_target: EventHandler<(usize, usize)>,
) -> Element {
    let arrow = match sort_order {
        SortOrder::Asc => "↑",
        SortOrder::Desc => "↓",
    };

    rsx! {
        div { class: "toolbar",
            span { "🔎" }
            input {
                class: "search-input",
                placeholder: "搜索项目名或路径...",
                value: "{search_filter}",
                oninput: move |e| on_search_change.call(e.value()),
            }
            span { class: "sort-label", "排序:" }
            button {
                class: if sort_by == SortBy::Size { "sort-btn active" } else { "sort-btn" },
                onclick: move |_| on_sort_change.call(SortBy::Size),
                "按大小"
            }
            button {
                class: if sort_by == SortBy::Name { "sort-btn active" } else { "sort-btn" },
                onclick: move |_| on_sort_change.call(SortBy::Name),
                "按名称"
            }
            span { class: "sort-label", "{arrow}" }
        }
        div { class: "project-list",
            if filtered_indices.is_empty() && !scanning {
                div { class: "empty-state",
                    "点击「开始扫描」来查找 Rust 项目的 target 目录"
                }
            }
            for idx in filtered_indices.iter() {
                {
                    let i = *idx;
                    let project = &projects[i];
                    let name = project.name.clone();
                    let path_str = project.path.display().to_string();
                    let size = project.target_size;
                    let size_str = format_size(size);
                    let color_class = size_color_class(size);
                    let targets = project.build_targets.clone();
                    let selected = project.selected;
                    let card_class = if selected { "project-card selected" } else { "project-card" };

                    rsx! {
                        div {
                            key: "{i}",
                            class: "{card_class}",
                            div {
                                class: "project-main-row",
                                onclick: move |_| on_toggle_select.call(i),
                                input {
                                    r#type: "checkbox",
                                    checked: selected,
                                    onclick: move |e| e.stop_propagation(),
                                    onchange: move |_| on_toggle_select.call(i),
                                }
                                div { class: "project-info",
                                    div { class: "project-header",
                                        span { class: "project-name", "{name}" }
                                        span { class: "project-size {color_class}", "{size_str}" }
                                    }
                                    div { class: "project-path", "{path_str}" }
                                }
                            }
                            if !targets.is_empty() {
                                div { class: "build-targets",
                                    span { class: "targets-label", "编译目标:" }
                                    for (ti, t) in targets.iter().enumerate() {
                                        {
                                            let t_name = t.name.clone();
                                            let t_size = format_size(t.size);
                                            let t_selected = t.selected;
                                            let t_class = if t.selected || selected {
                                                "target-item selected"
                                            } else {
                                                "target-item"
                                            };
                                            let proj_idx = i;
                                            rsx! {
                                                label {
                                                    key: "{ti}",
                                                    class: "{t_class}",
                                                    onclick: move |e| e.stop_propagation(),
                                                    input {
                                                        r#type: "checkbox",
                                                        checked: if selected { true } else { t_selected },
                                                        disabled: selected,
                                                        onchange: move |_| {
                                                            on_toggle_target.call((proj_idx, ti));
                                                        },
                                                    }
                                                    span { class: "target-name", "{t_name}" }
                                                    span { class: "target-size", "{t_size}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
