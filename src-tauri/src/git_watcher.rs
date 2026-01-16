use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub struct WatcherState {
    stop_tx: Option<mpsc::Sender<()>>,
    watched_path: Option<String>,
}

impl WatcherState {
    pub fn new() -> Self {
        Self {
            stop_tx: None,
            watched_path: None,
        }
    }
}

pub type WatcherHandle = Arc<Mutex<WatcherState>>;

pub fn create_watcher_handle() -> WatcherHandle {
    Arc::new(Mutex::new(WatcherState::new()))
}

#[derive(Clone, serde::Serialize)]
pub struct GitChangeEvent {
    pub change_type: String, // "branch", "index", "refs"
}

pub fn start_watching(
    app: AppHandle,
    watcher_handle: WatcherHandle,
    repo_path: String,
) -> Result<(), String> {
    // Stop any existing watcher
    stop_watching(watcher_handle.clone())?;

    let git_dir = PathBuf::from(&repo_path).join(".git");
    if !git_dir.exists() {
        return Err("Not a git repository".to_string());
    }

    let (stop_tx, stop_rx) = mpsc::channel::<()>();

    // Store the stop channel
    {
        let mut state = watcher_handle.lock().map_err(|e| e.to_string())?;
        state.stop_tx = Some(stop_tx);
        state.watched_path = Some(repo_path.clone());
    }

    // Spawn watcher thread
    thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = match new_debouncer(Duration::from_millis(500), tx) {
            Ok(d) => d,
            Err(e) => {
                log::error!("Failed to create debouncer: {}", e);
                return;
            }
        };

        // Watch the .git directory
        if let Err(e) = debouncer.watcher().watch(&git_dir, RecursiveMode::Recursive) {
            log::error!("Failed to watch .git directory: {}", e);
            return;
        }

        loop {
            if stop_rx.try_recv().is_ok() {
                break;
            }

            // Check for file events (with timeout)
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(Ok(events)) => {
                    let mut should_emit = false;
                    let mut change_type = "unknown";

                    for event in events {
                        let path_str = event.path.to_string_lossy();

                        if path_str.ends_with("HEAD") || path_str.contains("/HEAD") {
                            change_type = "branch";
                            should_emit = true;
                        } else if path_str.ends_with("/index") || path_str.ends_with("/index.lock") {
                            change_type = "index";
                            should_emit = true;
                        } else if path_str.contains("/refs/") {
                            change_type = "refs";
                            should_emit = true;
                        } else if path_str.contains("FETCH_HEAD")
                            || path_str.contains("ORIG_HEAD")
                            || path_str.contains("/logs/")
                        {
                            change_type = "refs";
                            should_emit = true;
                        }
                    }

                    if should_emit {
                        let _ = app.emit("git-changed", GitChangeEvent {
                            change_type: change_type.to_string(),
                        });
                    }
                }
                Ok(Err(_)) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
    });

    Ok(())
}

pub fn stop_watching(watcher_handle: WatcherHandle) -> Result<(), String> {
    let mut state = watcher_handle.lock().map_err(|e| e.to_string())?;

    if let Some(tx) = state.stop_tx.take() {
        let _ = tx.send(());
    }
    state.watched_path = None;

    Ok(())
}
