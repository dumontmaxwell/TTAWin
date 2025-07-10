//! Windows-specific system utilities for overlay management

use anyhow::Result;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, GetSystemMetrics, RegisterClassExW,
        SetWindowLongW, SetWindowPos, ShowWindow, UnregisterClassW,
        GWL_EXSTYLE, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SW_HIDE, SW_SHOW,
        WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
        WM_DESTROY, WM_PAINT, WNDCLASSEXW, WNDCLASS_STYLES, WINDOW_STYLE,
        GetWindowLongW,
    },
    Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT},
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{SM_CXSCREEN, SM_CYSCREEN},
};

use crate::error::OverlayError;

/// Global overlay state
static OVERLAY_ENABLED: AtomicBool = AtomicBool::new(false);
// Use isize instead of HWND to avoid Send trait issues with raw pointers
static OVERLAY_WINDOW: Mutex<Option<isize>> = Mutex::new(None);
static WINDOW_CLASS_REGISTERED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OverlayState {
    pub enabled: bool,
    pub window_handle: Option<u64>,
    pub topmost: bool,
    pub transparent: bool,
}

/// Window procedure for the overlay window
unsafe extern "system" fn overlay_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            unsafe {
                let _hdc = BeginPaint(hwnd, &mut ps);
                // We don't need to paint anything for a transparent overlay
                let _ = EndPaint(hwnd, &ps);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            // Clean up when window is destroyed
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

/// Register the overlay window class
unsafe fn register_overlay_window_class() -> Result<()> {
    if WINDOW_CLASS_REGISTERED.load(Ordering::Relaxed) {
        return Ok(());
    }

    let class_name = windows::core::w!("TTAWinOverlayClass");
    
    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: WNDCLASS_STYLES(0),
        lpfnWndProc: Some(overlay_window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: unsafe { GetModuleHandleW(None)?.into() },
        hIcon: windows::Win32::UI::WindowsAndMessaging::HICON::default(),
        hCursor: windows::Win32::UI::WindowsAndMessaging::HCURSOR::default(),
        hbrBackground: windows::Win32::Graphics::Gdi::HBRUSH::default(),
        lpszMenuName: windows::core::PCWSTR::null(),
        lpszClassName: class_name,
        hIconSm: windows::Win32::UI::WindowsAndMessaging::HICON::default(),
    };

    unsafe { RegisterClassExW(&wc) };
    WINDOW_CLASS_REGISTERED.store(true, Ordering::Relaxed);
    Ok(())
}

/// Initialize the overlay system
pub fn init_overlay() -> Result<()> {
    // Register the window class
    unsafe {
        register_overlay_window_class()?;
    }
    Ok(())
}

/// Toggle overlay on/off
pub fn toggle_overlay() -> Result<OverlayState> {
    let current_state = OVERLAY_ENABLED.load(Ordering::Relaxed);
    
    if current_state {
        disable_overlay()
    } else {
        enable_overlay()
    }
}

/// Enable overlay
pub fn enable_overlay() -> Result<OverlayState> {
    let mut window_guard = OVERLAY_WINDOW.lock().unwrap();
    
    // Create or show overlay window
    let hwnd = create_overlay_window()?;
    *window_guard = Some(hwnd.0 as isize);
    
    // Set window properties for overlay
    unsafe {
        // Make window topmost, transparent, and tool window
        let ex_style = WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_TOPMOST | WS_EX_TRANSPARENT;
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style.0 as i32);
        
        // Position window on top - fix: pass HWND_TOPMOST directly
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE,
        )?;
        
        // Show the window
        let _ = ShowWindow(hwnd, SW_SHOW);
    }
    
    OVERLAY_ENABLED.store(true, Ordering::Relaxed);
    
    Ok(OverlayState {
        enabled: true,
        window_handle: Some(hwnd.0 as u64),
        topmost: true,
        transparent: true,
    })
}

/// Disable overlay
pub fn disable_overlay() -> Result<OverlayState> {
    let mut window_guard = OVERLAY_WINDOW.lock().unwrap();
    
    if let Some(window_handle) = *window_guard {
        unsafe {
            // Convert back to HWND for API calls
            let hwnd = HWND(window_handle);
            // Hide the overlay window
            let _ = ShowWindow(hwnd, SW_HIDE);
        }
        *window_guard = None;
    }
    
    OVERLAY_ENABLED.store(false, Ordering::Relaxed);
    
    Ok(OverlayState {
        enabled: false,
        window_handle: None,
        topmost: false,
        transparent: false,
    })
}

/// Get current overlay state
pub fn get_overlay_state() -> OverlayState {
    let window_guard = OVERLAY_WINDOW.lock().unwrap();
    let enabled = OVERLAY_ENABLED.load(Ordering::Relaxed);
    
    OverlayState {
        enabled,
        window_handle: window_guard.map(|h| h as u64),
        topmost: enabled,
        transparent: enabled,
    }
}

/// Create the overlay window
fn create_overlay_window() -> Result<HWND> {
    unsafe {
        // Get screen dimensions
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        
        let class_name = windows::core::w!("TTAWinOverlayClass");
        let window_title = windows::core::w!("TTAWin Overlay");
        
        // Create the overlay window
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_TOPMOST | WS_EX_TRANSPARENT,
            class_name,
            window_title,
            WINDOW_STYLE(0), // No window style needed for overlay
            0, 0, // Position
            screen_width, screen_height, // Size - full screen
            None, // Parent
            None, // Menu
            GetModuleHandleW(None)?,
            None, // Additional data
        );
        
        if hwnd.0 == 0 {
            Err(OverlayError::WindowCreationFailed.into())
        } else {
            Ok(hwnd)
        }
    }
}

/// Check if current process has necessary privileges for overlay
pub fn check_overlay_permissions() -> Result<bool> {
    // Check if we have the necessary permissions to create overlays
    // This might involve checking for admin rights or specific Windows permissions
    Ok(true) // Simplified for now
}

/// Cleanup overlay resources
pub fn cleanup_overlay() -> Result<()> {
    disable_overlay()?;
    
    // Unregister the window class
    unsafe {
        if WINDOW_CLASS_REGISTERED.load(Ordering::Relaxed) {
            let class_name = windows::core::w!("TTAWinOverlayClass");
            let _ = unsafe { UnregisterClassW(class_name, GetModuleHandleW(None)?) };
            WINDOW_CLASS_REGISTERED.store(false, Ordering::Relaxed);
        }
    }
    
    Ok(())
}

/// Set window click-through behavior
pub fn set_window_click_through(enabled: bool) -> Result<()> {
    let window_guard = OVERLAY_WINDOW.lock().unwrap();
    
    if let Some(window_handle) = *window_guard {
        unsafe {
            let hwnd = HWND(window_handle);
            let current_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            
            let new_style = if enabled {
                // Add WS_EX_TRANSPARENT to make window click-through
                current_style | WS_EX_TRANSPARENT.0 as i32
            } else {
                // Remove WS_EX_TRANSPARENT to make window clickable
                current_style & !(WS_EX_TRANSPARENT.0 as i32)
            };
            
            SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);
        }
    }
    
    Ok(())
}

/// Get current window click-through state
pub fn get_window_click_through() -> Result<bool> {
    let window_guard = OVERLAY_WINDOW.lock().unwrap();
    
    if let Some(window_handle) = *window_guard {
        unsafe {
            let hwnd = HWND(window_handle);
            let current_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            
            // Check if WS_EX_TRANSPARENT is set
            Ok((current_style & WS_EX_TRANSPARENT.0 as i32) != 0)
        }
    } else {
        // No overlay window exists, default to click-through enabled
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlay_state() {
        let state = get_overlay_state();
        assert!(!state.enabled);
    }

    #[test]
    fn test_permissions() {
        assert!(check_overlay_permissions().is_ok());
    }
} 