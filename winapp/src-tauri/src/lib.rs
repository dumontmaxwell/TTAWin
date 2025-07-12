use tauri::{Emitter, Manager};
mod shortcuts;
mod streams;
mod api_response;

use api_response::ApiResponse;
use shortcuts::HotkeyManager;
use streams::AudioStream;

// Windows-specific imports for overlay functionality
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowLongPtrW, GetWindowLongPtrW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT,
    SetLayeredWindowAttributes, LWA_ALPHA, LWA_COLORKEY, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
    SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SWP_HIDEWINDOW,
    ShowWindow, SW_SHOW, SW_HIDE
};
use windows::Win32::Foundation::{HWND, RECT, POINT, COLORREF};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_SHIFT;

#[tauri::command]
async fn get_monitors(window: tauri::Window) -> ApiResponse<Vec<String>> {
    let monitors = window.available_monitors().unwrap();
    let monitor_names: Vec<String> = monitors.iter()
        .enumerate()
        .map(|(i, _)| format!("Monitor {}", i + 1))
        .collect();
    ApiResponse::success(monitor_names)
}

#[tauri::command]
async fn switch_monitor(window: tauri::Window, current_index: usize) -> ApiResponse<usize> {
    let monitors = window.available_monitors().unwrap();
    let next_index = (current_index + 1) % monitors.len();
    ApiResponse::success(next_index)
}

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

#[tauri::command]
async fn quit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

/// Windows-specific click-through implementation with security
#[tauri::command]
fn set_click_through(window: tauri::Window, enabled: bool) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(window.hwnd().unwrap().0 as isize);
        
        if enabled {
            // Enable click-through: make window transparent to mouse events
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
            SetWindowLongPtrW(
                hwnd,
                GWL_EXSTYLE,
                (ex_style | WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0) as isize,
            );
            
            // Set window to be transparent but still visible
            SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);
        } else {
            // Disable click-through: make window interactive (for security)
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
            SetWindowLongPtrW(
                hwnd,
                GWL_EXSTYLE,
                (ex_style & !WS_EX_TRANSPARENT.0) as isize,
            );
        }
    }
    Ok(())
}

/// Set overlay to "hidden" state - only top-right controls visible, middle click-through
#[tauri::command]
fn set_overlay_hidden(window: tauri::Window) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(window.hwnd().unwrap().0 as isize);
        
        // Show window but make it mostly transparent
        ShowWindow(hwnd, SW_SHOW);
        
        // Set to topmost
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
        );
        
        // Enable layered window for transparency
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongPtrW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style | WS_EX_LAYERED.0) as isize,
        );
        
        // Set transparency - very low alpha so only controls are visible
        SetLayeredWindowAttributes(hwnd, COLORREF(0), 30, LWA_ALPHA);
        
        // Enable click-through for middle area (will be handled by CSS)
        let ex_style_click = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongPtrW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style_click | WS_EX_TRANSPARENT.0) as isize,
        );
    }
    Ok(())
}

/// Set overlay to "visible" state - full overlay, no click-through for security
#[tauri::command]
fn set_overlay_visible(window: tauri::Window) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(window.hwnd().unwrap().0 as isize);
        
        // Show window
        ShowWindow(hwnd, SW_SHOW);
        
        // Set to topmost
        if let Err(e) = SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
        ) {
            eprintln!("Failed to set window position: {}", e);
            return Err(e.to_string());
        }
        
        // Enable layered window for transparency
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongPtrW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style | WS_EX_LAYERED.0) as isize,
        );
        
        // Set transparency to allow background visibility but maintain security
        if let Err(e) = SetLayeredWindowAttributes(hwnd, COLORREF(0), 180, LWA_ALPHA) {
            eprintln!("Failed to set layered window attributes: {}", e);
            return Err(e.to_string());
        }
        
        // DISABLE click-through for security - prevent clicks from reaching background
        let ex_style_click = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongPtrW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style_click & !WS_EX_TRANSPARENT.0) as isize,
        );
    }
    Ok(())
}

/// Toggle overlay visibility with proper state management
#[tauri::command]
fn toggle_overlay(window: tauri::Window) -> Result<ApiResponse<bool>, String> {
    unsafe {
        let hwnd = HWND(window.hwnd().unwrap().0 as isize);
        let is_visible = ShowWindow(hwnd, SW_SHOW).as_bool();
        
        if is_visible {
            // Switch to hidden state
            set_overlay_hidden(window)?;
            Ok(ApiResponse::success(false))
        } else {
            // Switch to visible state
            set_overlay_visible(window)?;
            Ok(ApiResponse::success(true))
        }
    }
}

/// Set overlay to full screen and properly configured
#[tauri::command]
fn setup_overlay(window: tauri::WebviewWindow) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(window.hwnd().unwrap().0 as isize);
        
        // Get screen dimensions
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        
        // Set window to full screen
        if let Err(e) = SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0, 0, screen_width, screen_height,
            SWP_SHOWWINDOW,
        ) {
            eprintln!("Failed to set window position: {}", e);
            return Err(e.to_string());
        }
        
        // Enable layered window for transparency
        let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongPtrW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style | WS_EX_LAYERED.0) as isize,
        );
        
        // Set initial transparency - show controls but allow click-through
        if let Err(e) = SetLayeredWindowAttributes(hwnd, COLORREF(0), 30, LWA_ALPHA) {
            eprintln!("Failed to set layered window attributes: {}", e);
        }
        
        // Show the window
        ShowWindow(hwnd, SW_SHOW);
    }
    Ok(())
}

/// Show overlay with proper transparency
#[tauri::command]
fn show_overlay(window: tauri::Window) -> Result<(), String> {
    set_overlay_visible(window)
}

/// Hide overlay
#[tauri::command]
fn hide_overlay(window: tauri::Window) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(window.hwnd().unwrap().0 as isize);
        ShowWindow(hwnd, SW_HIDE);
    }
    Ok(())
}

#[tauri::command]
async fn start_audio_stream(audio_stream: tauri::State<'_, AudioStream>) -> Result<ApiResponse<()>, String> {
    Ok(audio_stream.start_stream().await)
}

#[tauri::command]
async fn stop_audio_stream(audio_stream: tauri::State<'_, AudioStream>) -> Result<ApiResponse<()>, String> {
    Ok(audio_stream.stop_stream().await)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_mic_recorder::init())
        .setup(|app| {
            eprintln!("Setting up overlay");
            let app_handle = app.handle();
            let hotkey_manager = HotkeyManager::new(app_handle.clone());

            if let Some(window) = app.get_webview_window("main") {
                let _ = setup_overlay(window);
            }
            
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
            test_hotkey,
            trigger_action,
            quit_app,
            set_click_through,
            set_overlay_hidden,
            set_overlay_visible,
            toggle_overlay,
            setup_overlay,
            show_overlay,
            hide_overlay,
            start_audio_stream,
            stop_audio_stream,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
