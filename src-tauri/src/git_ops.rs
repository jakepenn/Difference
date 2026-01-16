use git2::{Delta, DiffOptions, Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangedFile {
    pub path: String,
    pub status: String,
    pub additions: i32,
    pub deletions: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDiff {
    pub path: String,
    pub hunks: Vec<DiffHunk>,
    pub is_binary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffLine {
    pub content: String,
    pub line_type: String,
    pub old_lineno: Option<u32>,
    pub new_lineno: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoInfo {
    pub path: String,
    pub current_branch: String,
    pub branches: Vec<BranchInfo>,
    pub default_base: String,
}

fn delta_to_status(delta: Delta) -> &'static str {
    match delta {
        Delta::Added => "added",
        Delta::Deleted => "deleted",
        Delta::Modified => "modified",
        Delta::Renamed => "renamed",
        Delta::Copied => "copied",
        Delta::Typechange => "typechange",
        _ => "unknown",
    }
}

pub fn get_repo_info(repo_path: &str) -> Result<RepoInfo, String> {
    let repo = Repository::discover(repo_path).map_err(|e| e.message().to_string())?;

    let head = repo.head().map_err(|e| e.message().to_string())?;
    let current_branch = head
        .shorthand()
        .unwrap_or("HEAD")
        .to_string();

    let mut branches = Vec::new();
    let branch_iter = repo
        .branches(None)
        .map_err(|e| e.message().to_string())?;

    for branch_result in branch_iter {
        let (branch, branch_type) = branch_result.map_err(|e| e.message().to_string())?;
        if let Some(name) = branch.name().map_err(|e| e.message().to_string())? {
            let is_remote = branch_type == git2::BranchType::Remote;
            let is_current = branch.is_head();
            branches.push(BranchInfo {
                name: name.to_string(),
                is_current,
                is_remote,
            });
        }
    }

    // Determine default base branch
    let default_base = if branches.iter().any(|b| b.name == "main" && !b.is_remote) {
        "main".to_string()
    } else if branches.iter().any(|b| b.name == "master" && !b.is_remote) {
        "master".to_string()
    } else {
        branches
            .iter()
            .find(|b| !b.is_remote && !b.is_current)
            .map(|b| b.name.clone())
            .unwrap_or_else(|| current_branch.clone())
    };

    Ok(RepoInfo {
        path: repo
            .workdir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default(),
        current_branch,
        branches,
        default_base,
    })
}

pub fn get_changed_files(repo_path: &str, base_branch: &str) -> Result<Vec<ChangedFile>, String> {
    let repo = Repository::discover(repo_path).map_err(|e| e.message().to_string())?;

    let head_commit = repo
        .head()
        .map_err(|e| e.message().to_string())?
        .peel_to_commit()
        .map_err(|e| e.message().to_string())?;

    // Find merge base between current HEAD and base branch
    let base_ref = repo
        .find_branch(base_branch, git2::BranchType::Local)
        .or_else(|_| repo.find_branch(&format!("origin/{}", base_branch), git2::BranchType::Remote))
        .map_err(|e| format!("Could not find branch '{}': {}", base_branch, e.message()))?;

    let base_commit = base_ref
        .get()
        .peel_to_commit()
        .map_err(|e| e.message().to_string())?;

    let merge_base_oid = repo
        .merge_base(head_commit.id(), base_commit.id())
        .map_err(|e| e.message().to_string())?;

    let merge_base_commit = repo
        .find_commit(merge_base_oid)
        .map_err(|e| e.message().to_string())?;

    let merge_base_tree = merge_base_commit
        .tree()
        .map_err(|e| e.message().to_string())?;

    let head_tree = head_commit.tree().map_err(|e| e.message().to_string())?;

    let mut diff_opts = DiffOptions::new();
    diff_opts.include_untracked(true);

    // Get diff from merge base to HEAD (committed changes)
    let diff = repo
        .diff_tree_to_tree(Some(&merge_base_tree), Some(&head_tree), Some(&mut diff_opts))
        .map_err(|e| e.message().to_string())?;

    let files: RefCell<HashMap<String, ChangedFile>> = RefCell::new(HashMap::new());

    diff.foreach(
        &mut |delta, _| {
            let path = delta
                .new_file()
                .path()
                .or_else(|| delta.old_file().path())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            let status = delta_to_status(delta.status());

            files.borrow_mut().insert(
                path.clone(),
                ChangedFile {
                    path,
                    status: status.to_string(),
                    additions: 0,
                    deletions: 0,
                },
            );
            true
        },
        None,
        None,
        Some(&mut |delta, _hunk, line| {
            let path = delta
                .new_file()
                .path()
                .or_else(|| delta.old_file().path())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            if let Some(file) = files.borrow_mut().get_mut(&path) {
                match line.origin() {
                    '+' => file.additions += 1,
                    '-' => file.deletions += 1,
                    _ => {}
                }
            }
            true
        }),
    )
    .map_err(|e| e.message().to_string())?;

    let mut files = files.into_inner();

    // Also check working directory for uncommitted changes
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    status_opts.recurse_untracked_dirs(true);

    let statuses = repo
        .statuses(Some(&mut status_opts))
        .map_err(|e| e.message().to_string())?;

    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("").to_string();
        let status = entry.status();

        if !files.contains_key(&path) {
            let status_str = if status.is_wt_new() || status.is_index_new() {
                "added"
            } else if status.is_wt_deleted() || status.is_index_deleted() {
                "deleted"
            } else if status.is_wt_modified() || status.is_index_modified() {
                "modified"
            } else {
                continue;
            };

            files.insert(
                path.clone(),
                ChangedFile {
                    path,
                    status: status_str.to_string(),
                    additions: 0,
                    deletions: 0,
                },
            );
        }
    }

    let mut result: Vec<ChangedFile> = files.into_values().collect();
    result.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(result)
}

pub fn get_file_diff(repo_path: &str, file_path: &str, base_branch: &str) -> Result<FileDiff, String> {
    let repo = Repository::discover(repo_path).map_err(|e| e.message().to_string())?;

    let head_commit = repo
        .head()
        .map_err(|e| e.message().to_string())?
        .peel_to_commit()
        .map_err(|e| e.message().to_string())?;

    let base_ref = repo
        .find_branch(base_branch, git2::BranchType::Local)
        .or_else(|_| repo.find_branch(&format!("origin/{}", base_branch), git2::BranchType::Remote))
        .map_err(|e| format!("Could not find branch '{}': {}", base_branch, e.message()))?;

    let base_commit = base_ref
        .get()
        .peel_to_commit()
        .map_err(|e| e.message().to_string())?;

    let merge_base_oid = repo
        .merge_base(head_commit.id(), base_commit.id())
        .map_err(|e| e.message().to_string())?;

    let merge_base_commit = repo
        .find_commit(merge_base_oid)
        .map_err(|e| e.message().to_string())?;

    let merge_base_tree = merge_base_commit
        .tree()
        .map_err(|e| e.message().to_string())?;

    let head_tree = head_commit.tree().map_err(|e| e.message().to_string())?;

    let mut diff_opts = DiffOptions::new();
    diff_opts.pathspec(file_path);
    diff_opts.context_lines(3);

    let diff = repo
        .diff_tree_to_tree(Some(&merge_base_tree), Some(&head_tree), Some(&mut diff_opts))
        .map_err(|e| e.message().to_string())?;

    let hunks: RefCell<Vec<DiffHunk>> = RefCell::new(Vec::new());
    let is_binary = RefCell::new(false);

    diff.foreach(
        &mut |delta, _| {
            *is_binary.borrow_mut() = delta.new_file().is_binary() || delta.old_file().is_binary();
            true
        },
        Some(&mut |_, _| true),
        Some(&mut |_delta, hunk| {
            hunks.borrow_mut().push(DiffHunk {
                old_start: hunk.old_start(),
                old_lines: hunk.old_lines(),
                new_start: hunk.new_start(),
                new_lines: hunk.new_lines(),
                lines: Vec::new(),
            });
            true
        }),
        Some(&mut |_delta, _hunk, line| {
            if let Some(current_hunk) = hunks.borrow_mut().last_mut() {
                let line_type = match line.origin() {
                    '+' => "add",
                    '-' => "delete",
                    ' ' => "context",
                    _ => "context",
                };

                let content = String::from_utf8_lossy(line.content()).to_string();

                current_hunk.lines.push(DiffLine {
                    content,
                    line_type: line_type.to_string(),
                    old_lineno: line.old_lineno(),
                    new_lineno: line.new_lineno(),
                });
            }
            true
        }),
    )
    .map_err(|e| e.message().to_string())?;

    Ok(FileDiff {
        path: file_path.to_string(),
        hunks: hunks.into_inner(),
        is_binary: is_binary.into_inner(),
    })
}

pub fn open_in_editor(repo_path: &str, file_path: &str) -> Result<(), String> {
    let full_path = Path::new(repo_path).join(file_path);

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-t")
            .arg(&full_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &full_path.to_string_lossy()])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&full_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
