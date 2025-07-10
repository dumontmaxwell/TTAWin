use serde::{Deserialize, Serialize};
use tauri::Emitter;
use win_sys::OverlayState;
mod shortcuts;
use shortcuts::HotkeyManager;


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
    // For now, return true as mic is available
    // TODO: Integrate with mic-recorder plugin for actual state
    ApiResponse::success(true)
}

#[tauri::command]
async fn toggle_mic(current: bool) -> ApiResponse<bool> {
    // TODO: Integrate with mic-recorder plugin to start/stop recording
    ApiResponse::success(!current)
}

#[tauri::command]
async fn start_audio_stream() -> ApiResponse<()> {
    // TODO: Use mic-recorder plugin to start audio streaming
    // This will be integrated with your transcription system
    ApiResponse::success(())
}

#[tauri::command]
async fn stop_audio_stream() -> ApiResponse<()> {
    // TODO: Use mic-recorder plugin to stop audio streaming
    ApiResponse::success(())
}

/// Test hotkey trigger (for debugging)
#[tauri::command]
async fn test_hotkey(action: String, window: tauri::Window) -> ApiResponse<()> {
    if let Err(e) = window.emit("hotkey-triggered", action) {
        ApiResponse::error(format!("Failed to emit hotkey event: {}", e))
    } else {
        ApiResponse::success(())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_mic_recorder::init())
        .setup(|app| {
            let app_handle = app.handle();
            
            // Initialize hotkey manager
            let hotkey_manager = HotkeyManager::new(app_handle.clone());
            
            // Register hotkeys and start listener
            tokio::spawn(async move {
                if let Err(e) = hotkey_manager.register_hotkeys().await {
                    eprintln!("Failed to register hotkeys: {}", e);
                }
                
                if let Err(e) = hotkey_manager.start_hotkey_listener().await {
                    eprintln!("Failed to start hotkey listener: {}", e);
                }
            });
            
            Ok(())
        })
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
            toggle_mic,
            start_audio_stream,
            stop_audio_stream,
            test_hotkey
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
