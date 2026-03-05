use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct BuildTarget {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub selected: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectInfo {
    pub name: String,
    pub path: PathBuf,
    pub target_path: PathBuf,
    pub target_size: u64,
    pub build_targets: Vec<BuildTarget>,
    pub selected: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortBy {
    Name,
    Size,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}
