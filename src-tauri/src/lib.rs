use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};
use walkdir::WalkDir;

// Global cancellation flag for scan operations
static SCAN_CANCELLED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderInfo {
    path: String,
    name: String,
    size: u64,
    file_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathValidation {
    valid: bool,
    reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityCheck {
    estimated_directories: u64,
    is_large: bool,
    is_blocked: bool,
    recommendation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    folders: Vec<FolderInfo>,
    total_size: u64,
    scan_path: String,
}

// Folders that typically consume lots of space in dev projects
const HEAVY_FOLDERS: &[&str] = &[
    "node_modules",
    ".git",
    "target",
    "build",
    "dist",
    ".next",
    ".nuxt",
    "vendor",
    "__pycache__",
    ".venv",
    "venv",
    ".cargo",
    "Pods",
    "DerivedData",
    ".gradle",
    "obj",
    "bin",
    ".dart_tool",
    ".pub-cache",
];

// Maximum depth to scan - safety limit
const MAX_SCAN_DEPTH: usize = 15;

// Maximum directories to scan before bailing out
const MAX_DIRS_LIMIT: u64 = 50_000;

// Paths that are too dangerous/large to scan
const DANGEROUS_PATHS: &[&str] = &[
    "/",
    "/Users",
    "/home",
    "/System",
    "/Applications",
    "/Library",
    "/usr",
    "/var",
    "/bin",
    "/sbin",
    "/etc",
    "/opt",
    "/private",
    "C:\\",
    "C:\\Windows",
    "C:\\Program Files",
    "C:\\Program Files (x86)",
    "C:\\Users",
];

/// Check if a path is dangerous (too close to system root)
fn is_dangerous_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Check against known dangerous paths
    for dangerous in DANGEROUS_PATHS {
        if path_str.eq_ignore_ascii_case(dangerous) {
            return true;
        }
    }

    // Check path component count - require at least 3 components
    // e.g., /Users/name/projects is OK (3+ components)
    //       /Users/name is NOT OK (only 2 components)
    let components: Vec<_> = path.components().collect();

    #[cfg(target_os = "windows")]
    {
        // On Windows, C:\ counts as one component, so we need 3+ after that
        if components.len() < 3 {
            return true;
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On Unix, / is the root, so paths like /Users/name need 3+ components
        if components.len() < 3 {
            return true;
        }
    }

    false
}

/// Calculate folder size with cancellation support
/// Returns Ok((size, file_count)) or Err(()) if cancelled
fn calculate_folder_size(path: &Path) -> Result<(u64, u64), ()> {
    let mut total_size: u64 = 0;
    let mut file_count: u64 = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        // Check cancellation every 500 files
        if file_count % 500 == 0 && SCAN_CANCELLED.load(Ordering::SeqCst) {
            return Err(());
        }

        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
                file_count += 1;
            }
        }
    }

    Ok((total_size, file_count))
}

fn find_heavy_folders(base_path: &Path) -> Result<Vec<FolderInfo>, String> {
    // Reset cancellation flag at start of scan
    SCAN_CANCELLED.store(false, Ordering::SeqCst);

    let mut results: Vec<FolderInfo> = Vec::new();
    let mut dirs_scanned: u64 = 0;

    for entry in WalkDir::new(base_path)
        .max_depth(MAX_SCAN_DEPTH)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        // Check cancellation every 100 directories
        if dirs_scanned % 100 == 0 && SCAN_CANCELLED.load(Ordering::SeqCst) {
            return Err("Scan cancelled".to_string());
        }

        // Hard limit protection
        dirs_scanned += 1;
        if dirs_scanned > MAX_DIRS_LIMIT {
            return Err(
                "Folder too large to scan safely. Please select a smaller folder.".to_string(),
            );
        }

        let folder_name = entry.file_name().to_string_lossy().to_string();

        if HEAVY_FOLDERS.contains(&folder_name.as_str()) {
            // Calculate size with cancellation support
            let (size, file_count) = match calculate_folder_size(entry.path()) {
                Ok(result) => result,
                Err(()) => return Err("Scan cancelled".to_string()),
            };

            // Only include folders > 1MB
            if size > 1_000_000 {
                results.push(FolderInfo {
                    path: entry.path().to_string_lossy().to_string(),
                    name: format!(
                        "{}/{}",
                        entry
                            .path()
                            .parent()
                            .and_then(|p| p.file_name())
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default(),
                        folder_name
                    ),
                    size,
                    file_count,
                });
            }
        }
    }

    // Sort by size descending
    results.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(results)
}

#[tauri::command]
fn validate_path(path: String) -> PathValidation {
    let path = Path::new(&path);

    if !path.exists() {
        return PathValidation {
            valid: false,
            reason: Some("Path does not exist".to_string()),
        };
    }

    if !path.is_dir() {
        return PathValidation {
            valid: false,
            reason: Some("Path is not a directory".to_string()),
        };
    }

    if is_dangerous_path(path) {
        return PathValidation {
            valid: false,
            reason: Some(
                "This folder is too close to the system root. Please select a folder at least 3 levels deep (e.g., /Users/name/projects)."
                    .to_string(),
            ),
        };
    }

    PathValidation {
        valid: true,
        reason: None,
    }
}

#[tauri::command]
async fn check_folder_complexity(path: String) -> Result<ComplexityCheck, String> {
    let path_buf = std::path::PathBuf::from(&path);

    if !path_buf.exists() {
        return Err("Path does not exist".to_string());
    }

    // Move the blocking scan to a background thread
    let result = tauri::async_runtime::spawn_blocking(move || {
        // Quick shallow scan to estimate complexity
        // Max depth of 4, sample up to 5000 directories
        let max_sample = 5000u64;
        let mut dir_count = 0u64;

        for _ in WalkDir::new(&path_buf)
            .max_depth(4)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
        {
            dir_count += 1;
            if dir_count >= max_sample {
                break;
            }
        }

        // Estimate total based on sample
        let estimated = if dir_count >= max_sample {
            // If we hit the sample limit, extrapolate (this is a rough estimate)
            dir_count * 10
        } else {
            dir_count
        };

        // Thresholds for warnings and blocking
        let is_blocked = estimated > 100_000;
        let is_large = estimated > 10_000;

        let recommendation = if is_blocked {
            "This folder is too large to scan safely. Please select a more specific subfolder."
                .to_string()
        } else if estimated > 50_000 {
            "This folder appears to be very large and may take several minutes to scan. Consider selecting a more specific subfolder.".to_string()
        } else if is_large {
            "This folder is fairly large. The scan may take a minute or more.".to_string()
        } else {
            "Folder size looks reasonable for scanning.".to_string()
        };

        ComplexityCheck {
            estimated_directories: estimated,
            is_large,
            is_blocked,
            recommendation,
        }
    })
    .await
    .map_err(|e| format!("Complexity check failed: {}", e))?;

    Ok(result)
}

#[tauri::command]
fn cancel_scan() {
    SCAN_CANCELLED.store(true, Ordering::SeqCst);
}

#[tauri::command]
async fn scan_folders(path: String) -> Result<ScanResult, String> {
    let path_buf = std::path::PathBuf::from(&path);

    // Validation stays synchronous (fast)
    if !path_buf.exists() {
        return Err("Path does not exist".to_string());
    }

    if !path_buf.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    // Additional safety check - should have been caught by validate_path but double-check
    if is_dangerous_path(&path_buf) {
        return Err(
            "Cannot scan system-level folders. Please select a more specific folder.".to_string(),
        );
    }

    let scan_path = path_buf.to_string_lossy().to_string();

    // Move heavy work to blocking thread pool
    let folders = tauri::async_runtime::spawn_blocking(move || find_heavy_folders(&path_buf))
        .await
        .map_err(|e| format!("Scan task failed: {}", e))??;

    let total_size: u64 = folders.iter().map(|f| f.size).sum();

    Ok(ScanResult {
        folders,
        total_size,
        scan_path,
    })
}

#[tauri::command]
fn delete_folder(path: String) -> Result<(), String> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err("Path does not exist".to_string());
    }

    // Move to trash instead of permanent delete for safety
    trash::delete(path).map_err(|e| format!("Failed to delete: {}", e))?;

    Ok(())
}

#[tauri::command]
fn open_in_finder(path: String) -> Result<(), String> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err("Path does not exist".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R") // Reveal in Finder (selects the item)
            .arg(path)
            .spawn()
            .map_err(|e| format!("Failed to open Finder: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg("/select,")
            .arg(path)
            .spawn()
            .map_err(|e| format!("Failed to open Explorer: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        // Linux doesn't have a standard "reveal" option, open the parent directory
        let dir_to_open = path.parent().unwrap_or(path);
        std::process::Command::new("xdg-open")
            .arg(dir_to_open)
            .spawn()
            .map_err(|e| format!("Failed to open file manager: {}", e))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Create tray menu
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let scan = MenuItem::with_id(app, "scan", "Scan Now", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &scan, &quit])?;

            // Build tray icon
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "scan" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = window.emit("trigger-scan", ());
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan_folders,
            delete_folder,
            open_in_finder,
            validate_path,
            check_folder_complexity,
            cancel_scan,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { ref api, .. } = event {
                api.prevent_exit();
            }

            // Handle macOS dock icon click
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen { .. } = event {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        });
}
