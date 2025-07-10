use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

pub struct AutoKillConfig {
    pub timeout_secs: u64,
}

impl Default for AutoKillConfig {
    fn default() -> Self {
        Self { timeout_secs: 10 }
    }
}

pub fn init(config: Option<AutoKillConfig>) -> impl Fn(&AppHandle) + Send + Sync + 'static {
    let config = config.unwrap_or_default();
    move |app: &AppHandle| {
        let app_handle = app.clone();
        let timeout = config.timeout_secs;
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));
            // Attempt to close all windows gracefully
            let _ = app_handle.exit(0);
        });
    }
} 