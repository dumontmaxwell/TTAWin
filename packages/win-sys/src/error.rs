//! Error types for win-sys

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OverlayError {
    #[error("Failed to create overlay window")]
    WindowCreationFailed,
    
    #[error("Overlay is not initialized")]
    NotInitialized,
    
    #[error("Insufficient permissions for overlay operations")]
    InsufficientPermissions,
    
    #[error("Windows API error: {0}")]
    WindowsApiError(#[from] windows::core::Error),
    
    #[error("Invalid window handle")]
    InvalidWindowHandle,
    
    #[error("Overlay already enabled")]
    AlreadyEnabled,
    
    #[error("Overlay already disabled")]
    AlreadyDisabled,
}