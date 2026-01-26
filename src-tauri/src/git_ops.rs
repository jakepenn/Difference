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
    pub is_cosmetic: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDiff {
    pub path: String,
    pub hunks: Vec<DiffHunk>,
    pub is_binary: bool,
    pub is_cosmetic: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<DiffLine>,
    pub is_cosmetic: bool,
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

/// Check if a line is a comment based on common patterns
fn is_comment_line(line: &str) -> bool {
    let trimmed = line.trim();

    // Empty lines are considered cosmetic
    if trimmed.is_empty() {
        return true;
    }

    // Single-line comments
    if trimmed.starts_with("//") ||      // C, C++, JS, TS, Rust, Go, Java
       trimmed.starts_with("#") ||       // Python, Ruby, Shell, YAML
       trimmed.starts_with("--") ||      // SQL, Lua, Haskell
       trimmed.starts_with(";") ||       // Lisp, Assembly, INI
       trimmed.starts_with("*") ||       // JSDoc, block comment continuation
       trimmed.starts_with("/*") ||      // C-style block comment start
       trimmed.starts_with("*/") ||      // C-style block comment end
       trimmed.starts_with("'") ||       // VB
       trimmed.starts_with("\"\"\"") ||  // Python docstring
       trimmed.starts_with("'''") ||     // Python docstring
       trimmed.starts_with("<!--") ||    // HTML/XML
       trimmed.starts_with("-->") ||     // HTML/XML
       trimmed.starts_with("rem ") ||    // Batch
       trimmed.to_lowercase().starts_with("rem ") {
        return true;
    }

    false
}

/// Check if two lines differ only in whitespace
fn is_whitespace_only_change(old: &str, new: &str) -> bool {
    // Compare without any whitespace
    let old_no_ws: String = old.chars().filter(|c| !c.is_whitespace()).collect();
    let new_no_ws: String = new.chars().filter(|c| !c.is_whitespace()).collect();
    old_no_ws == new_no_ws
}

/// Check if two lines differ only in indentation (leading whitespace)
fn is_indentation_only_change(old: &str, new: &str) -> bool {
    old.trim_start() == new.trim_start() && old != new
}

/// Check if two lines differ only in trailing whitespace
fn is_trailing_whitespace_change(old: &str, new: &str) -> bool {
    old.trim_end() == new.trim_end() && old != new
}

/// Check if two lines differ only in case
fn is_case_only_change(old: &str, new: &str) -> bool {
    old.to_lowercase() == new.to_lowercase() && old != new
}

/// Normalize content by removing all whitespace for comparison
fn normalize_content(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Check if multiple lines represent a formatting-only change (like line wrapping)
/// This handles cases where content is reformatted across different line counts
fn is_formatting_only_change(old_lines: &[&str], new_lines: &[&str]) -> bool {
    let old_combined = normalize_content(&old_lines.join(""));
    let new_combined = normalize_content(&new_lines.join(""));
    old_combined == new_combined
}

/// Analyze a hunk to determine if it's cosmetic
fn analyze_hunk_cosmetic(lines: &[DiffLine]) -> bool {
    let additions: Vec<&str> = lines
        .iter()
        .filter(|l| l.line_type == "add")
        .map(|l| l.content.as_str())
        .collect();

    let deletions: Vec<&str> = lines
        .iter()
        .filter(|l| l.line_type == "delete")
        .map(|l| l.content.as_str())
        .collect();

    // If only additions or only deletions, check if all are comments/whitespace/empty
    if additions.is_empty() && !deletions.is_empty() {
        return deletions.iter().all(|l| is_comment_line(l));
    }

    if deletions.is_empty() && !additions.is_empty() {
        return additions.iter().all(|l| is_comment_line(l));
    }

    // Check if it's a formatting-only change (content reformatted across lines)
    // This handles Tailwind class reordering, line wrapping, etc.
    if is_formatting_only_change(&deletions, &additions) {
        return true;
    }

    // If same number of additions and deletions, check pairwise
    if additions.len() == deletions.len() {
        for (old, new) in deletions.iter().zip(additions.iter()) {
            // Both are comments
            if is_comment_line(old) && is_comment_line(new) {
                continue;
            }
            // Whitespace-only change (any whitespace differs)
            if is_whitespace_only_change(old, new) {
                continue;
            }
            // Indentation-only change
            if is_indentation_only_change(old, new) {
                continue;
            }
            // Trailing whitespace change
            if is_trailing_whitespace_change(old, new) {
                continue;
            }
            // Case-only change
            if is_case_only_change(old, new) {
                continue;
            }
            // This is a real change
            return false;
        }
        return true;
    }

    // Different number of adds/deletes - check if all are comments
    let all_comments = additions.iter().all(|l| is_comment_line(l))
        && deletions.iter().all(|l| is_comment_line(l));

    all_comments
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

    // Store file info and their diff lines for cosmetic analysis
    let files: RefCell<HashMap<String, ChangedFile>> = RefCell::new(HashMap::new());
    let file_lines: RefCell<HashMap<String, Vec<(char, String)>>> = RefCell::new(HashMap::new());

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
                    path: path.clone(),
                    status: status.to_string(),
                    additions: 0,
                    deletions: 0,
                    is_cosmetic: false,
                },
            );
            file_lines.borrow_mut().insert(path, Vec::new());
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

            let origin = line.origin();
            let content = String::from_utf8_lossy(line.content()).to_string();

            if let Some(file) = files.borrow_mut().get_mut(&path) {
                match origin {
                    '+' => file.additions += 1,
                    '-' => file.deletions += 1,
                    _ => {}
                }
            }

            if origin == '+' || origin == '-' {
                if let Some(lines) = file_lines.borrow_mut().get_mut(&path) {
                    lines.push((origin, content));
                }
            }
            true
        }),
    )
    .map_err(|e| e.message().to_string())?;

    let mut files = files.into_inner();
    let file_lines = file_lines.into_inner();

    // Analyze each file for cosmetic changes
    for (path, lines) in file_lines.iter() {
        if let Some(file) = files.get_mut(path) {
            // Convert to DiffLine format for analysis
            let diff_lines: Vec<DiffLine> = lines
                .iter()
                .map(|(origin, content)| DiffLine {
                    content: content.clone(),
                    line_type: if *origin == '+' { "add".to_string() } else { "delete".to_string() },
                    old_lineno: None,
                    new_lineno: None,
                })
                .collect();

            file.is_cosmetic = !diff_lines.is_empty() && analyze_hunk_cosmetic(&diff_lines);
        }
    }

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
                    is_cosmetic: false,
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

    // Helper to extract hunks from a diff
    fn extract_hunks(diff: &git2::Diff) -> Result<(Vec<DiffHunk>, bool), String> {
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
                    is_cosmetic: false,
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

        Ok((hunks.into_inner(), is_binary.into_inner()))
    }

    let mut diff_opts = DiffOptions::new();
    diff_opts.pathspec(file_path);
    diff_opts.context_lines(3);

    // First try: diff from merge base to HEAD (committed changes)
    let committed_diff = repo
        .diff_tree_to_tree(Some(&merge_base_tree), Some(&head_tree), Some(&mut diff_opts))
        .map_err(|e| e.message().to_string())?;

    let (mut hunks, mut is_binary) = extract_hunks(&committed_diff)?;

    // If no committed changes found, try working directory changes (uncommitted)
    if hunks.is_empty() {
        let mut diff_opts_workdir = DiffOptions::new();
        diff_opts_workdir.pathspec(file_path);
        diff_opts_workdir.context_lines(3);
        diff_opts_workdir.include_untracked(true);

        let workdir_diff = repo
            .diff_tree_to_workdir_with_index(Some(&merge_base_tree), Some(&mut diff_opts_workdir))
            .map_err(|e| e.message().to_string())?;

        let (workdir_hunks, workdir_is_binary) = extract_hunks(&workdir_diff)?;
        hunks = workdir_hunks;
        is_binary = workdir_is_binary;
    }

    // If still no hunks, file might be untracked - read it directly
    if hunks.is_empty() {
        let workdir = repo.workdir().ok_or("No working directory")?;
        let full_path = workdir.join(file_path);

        if full_path.exists() {
            // Check if it's a binary file
            let content = std::fs::read(&full_path).map_err(|e| e.to_string())?;
            let is_binary_file = content.iter().take(8000).any(|&b| b == 0);

            if is_binary_file {
                is_binary = true;
            } else {
                // Create synthetic diff showing all lines as additions
                let text = String::from_utf8_lossy(&content);
                let lines: Vec<DiffLine> = text
                    .lines()
                    .enumerate()
                    .map(|(i, line)| DiffLine {
                        content: format!("{}\n", line),
                        line_type: "add".to_string(),
                        old_lineno: None,
                        new_lineno: Some((i + 1) as u32),
                    })
                    .collect();

                if !lines.is_empty() {
                    hunks.push(DiffHunk {
                        old_start: 0,
                        old_lines: 0,
                        new_start: 1,
                        new_lines: lines.len() as u32,
                        lines,
                        is_cosmetic: false,
                    });
                }
            }
        }
    }

    // Analyze each hunk for cosmetic changes
    for hunk in hunks.iter_mut() {
        hunk.is_cosmetic = analyze_hunk_cosmetic(&hunk.lines);
    }

    // File is cosmetic if all hunks are cosmetic
    let all_cosmetic = !hunks.is_empty() && hunks.iter().all(|h| h.is_cosmetic);

    Ok(FileDiff {
        path: file_path.to_string(),
        hunks,
        is_binary,
        is_cosmetic: all_cosmetic,
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
