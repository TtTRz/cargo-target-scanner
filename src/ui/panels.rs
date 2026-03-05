use dioxus::prelude::*;

use crate::utils::format_size;

#[component]
pub fn TopPanel(
    scan_root: String,
    scanning: bool,
    project_count: usize,
    on_scan_root_change: EventHandler<String>,
    on_start_scan: EventHandler<()>,
) -> Element {
    let on_scan_root_change2 = on_scan_root_change.clone();
    rsx! {
        div { class: "top-panel",
            div { class: "header-row",
                span { class: "title", "🦀 Cargo Target Scanner" }
                span { class: "scan-info",
                    if scanning {
                        span { class: "spinner" }
                        "扫描中... 已找到 {project_count} 个项目"
                    } else {
                        "共 {project_count} 个项目"
                    }
                }
            }
            div { class: "controls-row",
                span { "扫描目录:" }
                input {
                    class: "scan-root-input",
                    value: "{scan_root}",
                    oninput: move |e| on_scan_root_change.call(e.value()),
                }
                button {
                    class: "btn btn-secondary",
                    disabled: scanning,
                    onclick: move |_| {
                        let cb = on_scan_root_change2.clone();
                        spawn(async move {
                            if let Some(folder) = rfd::AsyncFileDialog::new()
                                .set_title("选择扫描目录")
                                .pick_folder()
                                .await
                            {
                                cb.call(folder.path().display().to_string());
                            }
                        });
                    },
                    "📂 选择目录"
                }
                button {
                    class: "btn btn-primary",
                    disabled: scanning,
                    onclick: move |_| on_start_scan.call(()),
                    if scanning { "⏳ 扫描中..." } else { "🔍 开始扫描" }
                }
            }
        }
    }
}

#[component]
pub fn BottomPanel(
    total_size: u64,
    has_selection: bool,
    delete_size: u64,
    delete_desc: String,
    status_message: Option<String>,
    show_confirm: bool,
    deleting: bool,
    on_select_all: EventHandler<()>,
    on_delete_click: EventHandler<()>,
    on_confirm_delete: EventHandler<()>,
    on_cancel_delete: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "bottom-panel",
            if let Some(msg) = &status_message {
                div { class: "status-msg", "{msg}" }
            }
            div { class: "bottom-row",
                div { class: "stats",
                    span { class: "total-size", "总占用: {format_size(total_size)}" }
                    if has_selection {
                        span { class: "selected-info",
                            "待删除: {delete_desc} ({format_size(delete_size)})"
                        }
                    }
                }
                div { class: "actions",
                    if show_confirm {
                        span { class: "confirm-text",
                            "确定删除 {delete_desc} 吗？"
                        }
                        button {
                            class: "btn btn-danger",
                            disabled: deleting,
                            onclick: move |_| on_confirm_delete.call(()),
                            "✅ 确认删除"
                        }
                        button {
                            class: "btn btn-secondary",
                            disabled: deleting,
                            onclick: move |_| on_cancel_delete.call(()),
                            "❌ 取消"
                        }
                    } else {
                        button {
                            class: "btn btn-secondary",
                            disabled: deleting,
                            onclick: move |_| on_select_all.call(()),
                            "全选"
                        }
                        button {
                            class: "btn btn-danger",
                            disabled: !has_selection || deleting,
                            onclick: move |_| on_delete_click.call(()),
                            "🗑 删除选中"
                        }
                    }
                }
            }
        }
    }
}
