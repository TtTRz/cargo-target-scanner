#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Language {
    Zh,
    En,
}

impl Language {
    pub fn toggle(self) -> Self {
        match self {
            Language::Zh => Language::En,
            Language::En => Language::Zh,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Language::Zh => "中文",
            Language::En => "EN",
        }
    }
}

pub struct I18n;

impl I18n {
    // App title
    pub fn app_title(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "🦀 Cargo Target Scanner",
            Language::En => "🦀 Cargo Target Scanner",
        }
    }

    // Top panel
    pub fn scanning_found(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "扫描中... 已找到",
            Language::En => "Scanning... found",
        }
    }

    pub fn projects_unit(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "个项目",
            Language::En => " projects",
        }
    }

    pub fn total_projects(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "共",
            Language::En => "",
        }
    }

    pub fn scan_dir(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "扫描目录:",
            Language::En => "Scan path:",
        }
    }

    pub fn choose_dir(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "📂",
            Language::En => "📂",
        }
    }

    pub fn choose_dir_title(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "选择扫描目录",
            Language::En => "Choose scan directory",
        }
    }

    pub fn start_scan(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "🔍",
            Language::En => "🔍",
        }
    }

    pub fn scanning(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "⏳ 扫描中...",
            Language::En => "⏳ Scanning...",
        }
    }

    // Toolbar
    pub fn search_placeholder(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "搜索项目名或路径...",
            Language::En => "Search project name or path...",
        }
    }

    pub fn sort_label(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "排序:",
            Language::En => "Sort:",
        }
    }

    pub fn sort_by_size(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "按大小",
            Language::En => "By Size",
        }
    }

    pub fn sort_by_name(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "按名称",
            Language::En => "By Name",
        }
    }

    // Project list
    pub fn empty_state(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "点击「开始扫描」来查找 Rust 项目的 target 目录",
            Language::En => "Click \"Start Scan\" to find Rust project target directories",
        }
    }

    pub fn build_targets_label(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "编译目标:",
            Language::En => "Build targets:",
        }
    }

    // Bottom panel
    pub fn total_usage(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "总占用:",
            Language::En => "Total:",
        }
    }

    pub fn pending_delete(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "待删除:",
            Language::En => "To delete:",
        }
    }

    pub fn confirm_delete_prefix(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "确定删除",
            Language::En => "Confirm delete",
        }
    }

    pub fn confirm_delete_suffix(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "吗？",
            Language::En => "?",
        }
    }

    pub fn btn_confirm_delete(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "✅ 确认删除",
            Language::En => "✅ Confirm",
        }
    }

    pub fn btn_cancel(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "❌ 取消",
            Language::En => "❌ Cancel",
        }
    }

    pub fn btn_select_all(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "全选",
            Language::En => "Select All",
        }
    }

    pub fn btn_delete_selected(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "🗑",
            Language::En => "🗑",
        }
    }

    // Loading
    pub fn deleting_loading(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "正在删除中，请稍候...",
            Language::En => "Deleting, please wait...",
        }
    }

    // Delete description helpers
    pub fn target_dirs_unit(lang: Language, count: usize) -> String {
        match lang {
            Language::Zh => format!("{} 个整个 target 目录", count),
            Language::En => {
                if count == 1 {
                    "1 whole target dir".to_string()
                } else {
                    format!("{} whole target dirs", count)
                }
            }
        }
    }

    pub fn build_targets_unit(lang: Language, count: usize) -> String {
        match lang {
            Language::Zh => format!("{} 个编译目标", count),
            Language::En => {
                if count == 1 {
                    "1 build target".to_string()
                } else {
                    format!("{} build targets", count)
                }
            }
        }
    }

    // Delete result messages
    pub fn delete_success(lang: Language, count: usize, size: &str) -> String {
        match lang {
            Language::Zh => format!("删除成功！已删除 {} 项，释放 {}", count, size),
            Language::En => format!("Deleted {} items, freed {}", count, size),
        }
    }

    pub fn delete_partial(lang: Language, count: usize, fail_count: usize, errors: &str) -> String {
        match lang {
            Language::Zh => format!("已删除 {} 项，{} 个失败: {}", count, fail_count, errors),
            Language::En => format!("Deleted {} items, {} failed: {}", count, fail_count, errors),
        }
    }

    // Scanning progress overlay
    pub fn scanning_title(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "正在扫描文件系统...",
            Language::En => "Scanning file system...",
        }
    }

    pub fn scanning_projects_found(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "个项目",
            Language::En => "projects",
        }
    }

    pub fn scanning_size_found(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "target 占用",
            Language::En => "target usage",
        }
    }

    pub fn scanning_elapsed(lang: Language) -> &'static str {
        match lang {
            Language::Zh => "耗时",
            Language::En => "elapsed",
        }
    }
}
