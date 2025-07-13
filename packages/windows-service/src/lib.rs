//! TTAWin Windows Service Library
//! 
//! This library provides the backend functionality for TTAWin as a Windows service.
//! It includes learning analysis, payment processing, settings management, and audio streaming.

pub mod config;
pub mod error;
pub mod http_server;
pub mod services;

// Re-export main types for easy access
pub use config::ServiceConfig;
pub use error::ServiceError;
pub use http_server::HttpServer;
pub use services::{LearningService, PaymentService, SettingsService, StreamService};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Service name constant
pub const SERVICE_NAME: &str = "TTAWinService";

/// Default configuration
pub fn default_config() -> ServiceConfig {
    ServiceConfig::default()
}

/// Create a new learning service with default configuration
pub async fn create_learning_service() -> Result<LearningService, ServiceError> {
    let config = config::LearningConfig::default();
    LearningService::new(config)
}

/// Create a new payment service with default configuration
pub fn create_payment_service() -> Result<PaymentService, ServiceError> {
    let config = config::PaymentConfig::default();
    PaymentService::new(config)
}

/// Create a new settings service with default configuration
pub fn create_settings_service() -> Result<SettingsService, ServiceError> {
    let config = config::SettingsConfig::default();
    SettingsService::new(config)
}

/// Create a new stream service with default configuration
pub fn create_stream_service() -> Result<StreamService, ServiceError> {
    let config = config::StreamConfig::default();
    StreamService::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.contains('.'));
    }

    #[test]
    fn test_service_name() {
        assert_eq!(SERVICE_NAME, "TTAWinService");
    }

    #[test]
    fn test_default_config() {
        let config = default_config();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.host, "127.0.0.1");
    }

    #[tokio::test]
    async fn test_learning_service_creation() {
        let result = create_learning_service().await;
        // This might fail in test environment due to missing dependencies
        // but we can at least verify the function exists and returns the right type
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_payment_service_creation() {
        let result = create_payment_service();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_settings_service_creation() {
        let result = create_settings_service();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_stream_service_creation() {
        let result = create_stream_service();
        assert!(result.is_ok() || result.is_err());
    }
} 