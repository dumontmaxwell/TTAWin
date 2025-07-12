use std::sync::Arc;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS, MOD_CONTROL, MOD_SHIFT,
};
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, WM_HOTKEY};

#[derive(Serialize, Clone)]
pub struct HotkeyEvent {
    pub action: String,
    pub timestamp: u64,
    pub source: String,
}

pub struct HotkeyManager {
    app_handle: AppHandle,
    registered_hotkeys: Arc<Mutex<Vec<i32>>>,
}

impl HotkeyManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            registered_hotkeys: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn register_hotkeys(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut hotkeys = self.registered_hotkeys.lock().await;

        // Register hotkeys with unique IDs
        let hotkey_configs = vec![
            (1i32, MOD_CONTROL.0 | MOD_SHIFT.0, 'H' as u32, "toggle_overlay"),
            (2i32, MOD_CONTROL.0 | MOD_SHIFT.0, 'M' as u32, "toggle_mic"),
            (3i32, MOD_CONTROL.0 | MOD_SHIFT.0, 'N' as u32, "switch_monitor"),
            (4i32, MOD_CONTROL.0 | MOD_SHIFT.0, 'S' as u32, "open_settings"),
            (5i32, MOD_CONTROL.0 | MOD_SHIFT.0, 'Q' as u32, "quit"),
        ];

        for (id, modifiers, key, action) in hotkey_configs {
            unsafe {
                let success = RegisterHotKey(
                    windows::Win32::Foundation::HWND(0),
                    id,
                    HOT_KEY_MODIFIERS(modifiers),
                    key,
                );

                if success.is_ok() {
                    hotkeys.push(id);
                    println!("Registered hotkey {} for action: {}", id, action);
                } else {
                    eprintln!("Failed to register hotkey {} for action: {}", id, action);
                }
            }
        }

        Ok(())
    }

    pub async fn unregister_hotkeys(&self) -> Result<(), Box<dyn std::error::Error>> {
        let hotkeys = self.registered_hotkeys.lock().await;

        for &id in hotkeys.iter() {
            unsafe {
                let _ = UnregisterHotKey(windows::Win32::Foundation::HWND(0), id);
            }
        }

        Ok(())
    }

    pub async fn start_hotkey_listener(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app_handle = self.app_handle.clone();
        let registered_hotkeys = self.registered_hotkeys.clone();

        tokio::spawn(async move {
            let mut msg = MSG::default();

            loop {
                unsafe {
                    let result = GetMessageW(&mut msg, windows::Win32::Foundation::HWND(0), 0, 0);
                    
                    if result.0 == -1 {
                        // Error occurred
                        break;
                    } else if result.0 == 0 {
                        // WM_QUIT received
                        break;
                    }

                    if msg.message == WM_HOTKEY {
                        let hotkey_id = msg.wParam.0 as i32;
                        let hotkeys = registered_hotkeys.lock().await;
                        
                        if hotkeys.contains(&hotkey_id) {
                            let action = match hotkey_id {
                                1 => "toggle_overlay",
                                2 => "toggle_mic",
                                3 => "switch_monitor", 
                                4 => "open_settings",
                                5 => "quit",
                                _ => "unknown",
                            };

                            println!("Hotkey triggered: {} (ID: {})", action, hotkey_id);
                            
                            // Create detailed event payload
                            let event = HotkeyEvent {
                                action: action.to_string(),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis() as u64,
                                source: "hotkey".to_string(),
                            };
                            
                            // Emit event to frontend
                            if let Err(e) = app_handle.emit("hotkey-triggered", event) {
                                eprintln!("Failed to emit hotkey event: {}", e);
                            }
                        }
                    }

                    DispatchMessageW(&msg);
                }
            }
        });

        Ok(())
    }
}
