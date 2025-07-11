use serde::{Deserialize, Serialize};
use tauri::Emitter;
mod shortcuts;
mod auto_kill;

use shortcuts::HotkeyManager;
use auto_kill::{AutoKillConfig, init as init_auto_kill};

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
    let event = shortcuts::HotkeyEvent {
        action: action.clone(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
        source: "test".to_string(),
    };
    
    if let Err(e) = window.emit("hotkey-triggered", event) {
        ApiResponse::error(format!("Failed to emit hotkey event: {}", e))
    } else {
        ApiResponse::success(())
    }
}

/// Trigger action from frontend (for button clicks)
#[tauri::command]
async fn trigger_action(action: String, window: tauri::Window) -> ApiResponse<()> {
    let event = shortcuts::HotkeyEvent {
        action: action.clone(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
        source: "button".to_string(),
    };
    
    if let Err(e) = window.emit("hotkey-triggered", event) {
        ApiResponse::error(format!("Failed to emit action event: {}", e))
    } else {
        ApiResponse::success(())
    }
}

/// Open settings window
#[tauri::command]
async fn open_settings_window(window: tauri::Window) -> ApiResponse<()> {
    // For now, just log the action - we'll implement proper window management later
    println!("Opening settings window from window: {}", window.label());
    ApiResponse::success(())
}

/// Close settings window
#[tauri::command]
async fn close_settings_window(window: tauri::Window) -> ApiResponse<()> {
    // For now, just log the action - we'll implement proper window management later
    println!("Closing settings window from window: {}", window.label());
    ApiResponse::success(())
}

/// Quit the application properly
#[tauri::command]
async fn quit_app(_window: tauri::Window) -> ApiResponse<()> {
    std::process::exit(0);
}

#[cfg(target_os = "windows")]
fn set_click_through_impl(window: &tauri::Window, enabled: bool) -> Result<(), String> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_TRANSPARENT};
    use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

    let hwnd = match window.raw_window_handle() {
        RawWindowHandle::Win32(handle) => HWND(handle.hwnd as isize),
        _ => return Err("Not a Win32 window".to_string()),
    };
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        let new_style = if enabled {
            ex_style | WS_EX_TRANSPARENT.0 as i32
        } else {
            ex_style & !(WS_EX_TRANSPARENT.0 as i32)
        };
        SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn set_click_through_impl(_window: &tauri::Window, _enabled: bool) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
fn set_click_through(window: tauri::Window, enabled: bool) -> Result<(), String> {
    set_click_through_impl(&window, enabled)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_mic_recorder::init())
        .setup(|app| {
            // Start auto-kill timer with 5 second timeout
            let auto_kill_config = AutoKillConfig { timeout_secs: 5 };
            let auto_kill_fn = init_auto_kill(Some(auto_kill_config));
            auto_kill_fn(&app.handle());
            
            // Initialize hotkey manager
            let app_handle = app.handle();
            let hotkey_manager = HotkeyManager::new(app_handle.clone());
            
            // Register hotkeys and start listener using Tauri's runtime
            tauri::async_runtime::spawn(async move {
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
            get_monitors,
            switch_monitor,
            get_mic_state,
            toggle_mic,
            start_audio_stream,
            stop_audio_stream,
            test_hotkey,
            trigger_action,
            open_settings_window,
            close_settings_window,
            quit_app,
            set_click_through
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
