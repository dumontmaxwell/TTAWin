use tauri::AppHandle;

/// Gracefully terminate the application using Tauri's best practice
pub async fn terminate_app(app_handle: AppHandle) {
    eprintln!("Quitting TTAWin - using Tauri AppHandle::exit(0)");
    app_handle.exit(0);
}

/// Get current process information
pub fn get_process_info() -> (u32, String) {
    let pid = std::process::id();
    let exe_path = std::env::current_exe()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    (pid, exe_path)
}

/// Check if TTAWin processes are still running
pub fn is_ttawin_running() -> bool {
    let output = std::process::Command::new("tasklist")
        .args(&["/FI", "IMAGENAME eq TTAWin*"])
        .output();
    
    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Check if any TTAWin processes are listed
            output_str.contains("TTAWin") || output_str.contains("ttawin")
        }
        Err(_) => false
    }
}

/// Kill processes by name (case insensitive)
pub fn kill_processes_by_name(process_name: &str) -> bool {
    let result = std::process::Command::new("taskkill")
        .args(&["/F", "/IM", &format!("{}.exe", process_name)])
        .output();
    
    match result {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            eprintln!("Killed processes for {}: {}", process_name, output_str);
            true
        }
        Err(e) => {
            eprintln!("Failed to kill processes for {}: {}", process_name, e);
            false
        }
    }
} 