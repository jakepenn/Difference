mod git_ops;

use git_ops::{ChangedFile, FileDiff, RepoInfo};

#[tauri::command]
fn get_repo_info(repo_path: String) -> Result<RepoInfo, String> {
    git_ops::get_repo_info(&repo_path)
}

#[tauri::command]
fn get_changed_files(repo_path: String, base_branch: String) -> Result<Vec<ChangedFile>, String> {
    git_ops::get_changed_files(&repo_path, &base_branch)
}

#[tauri::command]
fn get_file_diff(repo_path: String, file_path: String, base_branch: String) -> Result<FileDiff, String> {
    git_ops::get_file_diff(&repo_path, &file_path, &base_branch)
}

#[tauri::command]
fn open_in_editor(repo_path: String, file_path: String) -> Result<(), String> {
    git_ops::open_in_editor(&repo_path, &file_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_repo_info,
            get_changed_files,
            get_file_diff,
            open_in_editor,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
