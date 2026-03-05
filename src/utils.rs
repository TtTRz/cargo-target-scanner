pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn size_color_class(bytes: u64) -> &'static str {
    const MB: u64 = 1024 * 1024;
    if bytes >= 1024 * MB {
        "size-red"
    } else if bytes >= 500 * MB {
        "size-orange"
    } else if bytes >= 100 * MB {
        "size-yellow"
    } else {
        "size-green"
    }
}
