use dioxus::prelude::*;

use crate::i18n::{I18n, Language};
use crate::utils::format_size;

#[component]
pub fn TopPanel(
    scan_root: String,
    scanning: bool,
    project_count: usize,
    language: Language,
    on_scan_root_change: EventHandler<String>,
    on_start_scan: EventHandler<()>,
    on_toggle_language: EventHandler<()>,
) -> Element {
    let on_scan_root_change2 = on_scan_root_change.clone();
    let lang = language;
    rsx! {
        div { class: "top-panel",
            div { class: "header-row",
                span { class: "title", "{I18n::app_title(lang)}" }
                div { class: "header-right",
                    span { class: "scan-info",
                        if scanning {
                            span { class: "spinner" }
                            "{I18n::scanning_found(lang)} {project_count} {I18n::projects_unit(lang)}"
                        } else {
                            "{I18n::total_projects(lang)} {project_count} {I18n::projects_unit(lang)}"
                        }
                    }
                    button {
                        class: "btn btn-lang",
                        onclick: move |_| on_toggle_language.call(()),
                        "🌐 {language.label()}"
                    }
                }
            }
            div { class: "controls-row",
                span { "{I18n::scan_dir(lang)}" }
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
                        let title = I18n::choose_dir_title(lang).to_string();
                        spawn(async move {
                            if let Some(folder) = rfd::AsyncFileDialog::new()
                                .set_title(&title)
                                .pick_folder()
                                .await
                            {
                                cb.call(folder.path().display().to_string());
                            }
                        });
                    },
                    "{I18n::choose_dir(lang)}"
                }
                button {
                    class: "btn btn-primary",
                    disabled: scanning,
                    onclick: move |_| on_start_scan.call(()),
                    if scanning { "{I18n::scanning(lang)}" } else { "{I18n::start_scan(lang)}" }
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
    language: Language,
    on_select_all: EventHandler<()>,
    on_delete_click: EventHandler<()>,
    on_confirm_delete: EventHandler<()>,
    on_cancel_delete: EventHandler<()>,
) -> Element {
    let lang = language;
    rsx! {
        div { class: "bottom-panel",
            if let Some(msg) = &status_message {
                div { class: "status-msg", "{msg}" }
            }
            div { class: "bottom-row",
                div { class: "stats",
                    span { class: "total-size", "{I18n::total_usage(lang)} {format_size(total_size)}" }
                    if has_selection {
                        span { class: "selected-info",
                            "{I18n::pending_delete(lang)} {delete_desc} ({format_size(delete_size)})"
                        }
                    }
                }
                div { class: "actions",
                    if show_confirm {
                        span { class: "confirm-text",
                            "{I18n::confirm_delete_prefix(lang)} {delete_desc} {I18n::confirm_delete_suffix(lang)}"
                        }
                        button {
                            class: "btn btn-danger",
                            disabled: deleting,
                            onclick: move |_| on_confirm_delete.call(()),
                            "{I18n::btn_confirm_delete(lang)}"
                        }
                        button {
                            class: "btn btn-secondary",
                            disabled: deleting,
                            onclick: move |_| on_cancel_delete.call(()),
                            "{I18n::btn_cancel(lang)}"
                        }
                    } else {
                        button {
                            class: "btn btn-secondary",
                            disabled: deleting,
                            onclick: move |_| on_select_all.call(()),
                            "{I18n::btn_select_all(lang)}"
                        }
                        button {
                            class: "btn btn-danger",
                            disabled: !has_selection || deleting,
                            onclick: move |_| on_delete_click.call(()),
                            "{I18n::btn_delete_selected(lang)}"
                        }
                    }
                }
            }
        }
    }
}
