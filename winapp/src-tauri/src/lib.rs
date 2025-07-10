use serde::{Deserialize, Serialize};
use win_sys::OverlayState;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Initialize the overlay system
#[tauri::command]
async fn init_overlay() -> ApiResponse<()> {
    match win_sys::init_overlay() {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(format!("Failed to initialize overlay: {}", e)),
    }
}

/// Enable the overlay
#[tauri::command]
async fn enable_overlay() -> ApiResponse<OverlayState> {
    match win_sys::enable_overlay() {
        Ok(state) => ApiResponse::success(state),
        Err(e) => ApiResponse::error(format!("Failed to enable overlay: {}", e)),
    }
}

/// Disable the overlay
#[tauri::command]
async fn disable_overlay() -> ApiResponse<OverlayState> {
    match win_sys::disable_overlay() {
        Ok(state) => ApiResponse::success(state),
        Err(e) => ApiResponse::error(format!("Failed to disable overlay: {}", e)),
    }
}

/// Toggle the overlay on/off
#[tauri::command]
async fn toggle_overlay() -> ApiResponse<OverlayState> {
    match win_sys::toggle_overlay() {
        Ok(state) => ApiResponse::success(state),
        Err(e) => ApiResponse::error(format!("Failed to toggle overlay: {}", e)),
    }
}

/// Get current overlay state
#[tauri::command]
async fn get_overlay_state() -> ApiResponse<OverlayState> {
    let state = win_sys::get_overlay_state();
    ApiResponse::success(state)
}

/// Check overlay permissions
#[tauri::command]
async fn check_overlay_permissions() -> ApiResponse<bool> {
    match win_sys::check_overlay_permissions() {
        Ok(has_permissions) => ApiResponse::success(has_permissions),
        Err(e) => ApiResponse::error(format!("Failed to check permissions: {}", e)),
    }
}

/// Cleanup overlay resources
#[tauri::command]
async fn cleanup_overlay(with_exit: bool) -> ApiResponse<()> {
    match win_sys::cleanup_overlay() {
        Ok(_) => ApiResponse::success(()),
        Err(e) => ApiResponse::error(format!("Failed to cleanup overlay: {}", e)),
    };

    if with_exit {
        std::process::exit(0);
    }
    ApiResponse::success(())
}

#[tauri::command]
async fn get_monitors(window: tauri::Window) -> ApiResponse<usize> {
    let monitors = window.available_monitors().unwrap().len();
    ApiResponse::success(monitors)
}

#[tauri::command]
async fn switch_monitor(window: tauri::Window, current_index: usize) -> ApiResponse<usize> {
    let monitors = window.available_monitors().unwrap();
    let next_index = (current_index + 1) % monitors.len();
    ApiResponse::success(next_index)
}

#[tauri::command]
async fn get_mic_state() -> ApiResponse<bool> {
    // Stub: Always enabled
    ApiResponse::success(true)
}

#[tauri::command]
async fn toggle_mic(current: bool) -> ApiResponse<bool> {
    // Stub: Just invert
    ApiResponse::success(!current)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_mic_recorder::init())
        .invoke_handler(tauri::generate_handler![
            init_overlay,
            enable_overlay,
            disable_overlay,
            toggle_overlay,
            get_overlay_state,
            check_overlay_permissions,
            cleanup_overlay,
            get_monitors,
            switch_monitor,
            get_mic_state,
            toggle_mic
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
