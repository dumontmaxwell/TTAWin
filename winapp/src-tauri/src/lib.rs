use tauri::{Emitter, Manager};
mod shortcuts;
mod streams;
mod api_response;
mod task;
mod service_client;
mod service_bridge;

use api_response::ApiResponse;
use shortcuts::HotkeyManager;
use streams::AudioStream;
use service_client::{init_service_client, get_service_client, is_service_available};
use service_bridge::{init_service_bridge, get_service_bridge};

// ===== UNIFIED SERVICE BRIDGE COMMANDS =====

/// Unified content analysis (uses service if available, falls back to direct)
#[tauri::command]
async fn unified_analyze_content(
    content_type: String,
    content: String,
    session_id: String,
) -> ApiResponse<serde_json::Value> {
    match get_service_bridge()
        .analyze_content(&content_type, &content, &session_id)
        .await
    {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Content analysis failed: {}", e)),
    }
}

/// Unified OCR processing (uses service if available, falls back to direct)
#[tauri::command]
async fn unified_process_ocr(image_path: String, session_id: String) -> ApiResponse<serde_json::Value> {
    match get_service_bridge().process_ocr(&image_path, &session_id).await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("OCR processing failed: {}", e)),
    }
}

/// Unified payment processing (uses service if available, falls back to direct)
#[tauri::command]
async fn unified_process_payment(
    amount: u64,
    currency: String,
    description: String,
    customer_email: String,
    payment_method: String,
    crypto_details: Option<std::collections::HashMap<String, String>>,
) -> ApiResponse<serde_json::Value> {
    match get_service_bridge()
        .process_payment(
            amount,
            &currency,
            &description,
            &customer_email,
            &payment_method,
            crypto_details,
        )
        .await
    {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Payment processing failed: {}", e)),
    }
}

/// Unified configuration retrieval (uses service if available, falls back to direct)
#[tauri::command]
async fn unified_get_config() -> ApiResponse<serde_json::Value> {
    match get_service_bridge().get_configuration().await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Configuration retrieval failed: {}", e)),
    }
}

/// Unified configuration update (uses service if available, falls back to direct)
#[tauri::command]
async fn unified_update_config(config: serde_json::Value) -> ApiResponse<serde_json::Value> {
    let config_map = serde_json::from_value::<std::collections::HashMap<String, serde_json::Value>>(config)
        .map_err(|e| format!("Invalid config format: {}", e))?;
    
    match get_service_bridge().update_configuration(config_map).await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Configuration update failed: {}", e)),
    }
}

/// Unified audio stream start (uses service if available, falls back to direct)
#[tauri::command]
async fn unified_start_audio_stream(
    session_id: String,
    buffer_size: Option<u32>,
    sample_rate: Option<u32>,
) -> ApiResponse<serde_json::Value> {
    match get_service_bridge()
        .start_audio_stream(&session_id, buffer_size, sample_rate)
        .await
    {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Audio stream start failed: {}", e)),
    }
}

/// Unified audio stream stop (uses service if available, falls back to direct)
#[tauri::command]
async fn unified_stop_audio_stream(session_id: String) -> ApiResponse<serde_json::Value> {
    match get_service_bridge().stop_audio_stream(&session_id).await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Audio stream stop failed: {}", e)),
    }
}

// ===== SERVICE CLIENT COMMANDS =====

/// Check if the Windows service is available
#[tauri::command]
async fn check_service_health() -> ApiResponse<serde_json::Value> {
    match get_service_client().health_check().await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Service health check failed: {}", e)),
    }
}

/// Get system status from the service
#[tauri::command]
async fn get_service_status() -> ApiResponse<serde_json::Value> {
    match get_service_client().get_system_status().await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Failed to get service status: {}", e)),
    }
}

/// Get available AI models from the service
#[tauri::command]
async fn get_ai_models() -> ApiResponse<Vec<String>> {
    match get_service_client().get_learning_models().await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Failed to get AI models: {}", e)),
    }
}

/// Analyze content using the service
#[tauri::command]
async fn analyze_content(
    content_type: String,
    content: String,
    session_id: String,
) -> ApiResponse<serde_json::Value> {
    match get_service_client()
        .analyze_content(&content_type, &content, &session_id)
        .await
    {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Content analysis failed: {}", e)),
    }
}

/// Process OCR using the service
#[tauri::command]
async fn process_ocr(image_path: String, session_id: String) -> ApiResponse<serde_json::Value> {
    match get_service_client().process_ocr(&image_path, &session_id).await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("OCR processing failed: {}", e)),
    }
}

/// Get payment methods from the service
#[tauri::command]
async fn get_payment_methods() -> ApiResponse<Vec<String>> {
    match get_service_client().get_payment_methods().await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Failed to get payment methods: {}", e)),
    }
}

/// Process Stripe payment through the service
#[tauri::command]
async fn process_stripe_payment(
    amount: u64,
    currency: String,
    description: String,
    customer_email: String,
) -> ApiResponse<serde_json::Value> {
    match get_service_client()
        .process_stripe_payment(amount, &currency, &description, &customer_email)
        .await
    {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Stripe payment failed: {}", e)),
    }
}

/// Process crypto payment through the service
#[tauri::command]
async fn process_crypto_payment(
    amount: u64,
    currency: String,
    description: String,
    customer_email: String,
    crypto_currency: String,
    network: String,
    wallet_address: String,
) -> ApiResponse<serde_json::Value> {
    match get_service_client()
        .process_crypto_payment(
            amount,
            &currency,
            &description,
            &customer_email,
            &crypto_currency,
            &network,
            &wallet_address,
        )
        .await
    {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Crypto payment failed: {}", e)),
    }
}

/// Get configuration from the service
#[tauri::command]
async fn get_service_config() -> ApiResponse<serde_json::Value> {
    match get_service_client().get_configuration().await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Failed to get configuration: {}", e)),
    }
}

/// Update configuration through the service
#[tauri::command]
async fn update_service_config(config: serde_json::Value) -> ApiResponse<serde_json::Value> {
    let config_map = serde_json::from_value::<std::collections::HashMap<String, serde_json::Value>>(config)
        .map_err(|e| format!("Invalid config format: {}", e))?;
    
    match get_service_client().update_configuration(config_map).await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Failed to update configuration: {}", e)),
    }
}

/// Start audio stream through the service
#[tauri::command]
async fn start_service_audio_stream(
    session_id: String,
    buffer_size: Option<u32>,
    sample_rate: Option<u32>,
) -> ApiResponse<serde_json::Value> {
    match get_service_client()
        .start_audio_stream(&session_id, buffer_size, sample_rate)
        .await
    {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Failed to start audio stream: {}", e)),
    }
}

/// Stop audio stream through the service
#[tauri::command]
async fn stop_service_audio_stream(session_id: String) -> ApiResponse<serde_json::Value> {
    match get_service_client().stop_audio_stream(&session_id).await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Failed to stop audio stream: {}", e)),
    }
}

/// Get stream status from the service
#[tauri::command]
async fn get_stream_status() -> ApiResponse<serde_json::Value> {
    match get_service_client().get_stream_status().await {
        Ok(response) => response,
        Err(e) => ApiResponse::error(format!("Failed to get stream status: {}", e)),
    }
}

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
    task::terminate_app(app_handle).await;
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
        
        // Set initial transparency (fully transparent)
        if let Err(e) = SetLayeredWindowAttributes(hwnd, COLORREF(0), 0, LWA_ALPHA) {
            eprintln!("Failed to set layered window attributes: {}", e);
        }
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
    // Initialize the service client and bridge
    init_service_client();
    init_service_bridge(true); // Prefer service, fallback to direct
    
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
            // Unified service bridge commands (preferred)
            unified_analyze_content,
            unified_process_ocr,
            unified_process_payment,
            unified_get_config,
            unified_update_config,
            unified_start_audio_stream,
            unified_stop_audio_stream,
            
            // Service client commands (direct service access)
            check_service_health,
            get_service_status,
            get_ai_models,
            analyze_content,
            process_ocr,
            get_payment_methods,
            process_stripe_payment,
            process_crypto_payment,
            get_service_config,
            update_service_config,
            start_service_audio_stream,
            stop_service_audio_stream,
            get_stream_status,
            
            // Existing commands
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
