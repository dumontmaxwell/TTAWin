use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Windows service error: {0}")]
    WindowsService(#[from] windows_service::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] anyhow::Error),

    #[error("HTTP server error: {0}")]
    HttpServer(#[from] axum::Error),

    #[error("Learning service error: {0}")]
    Learning(#[from] learning::Error),

    #[error("Payment service error: {0}")]
    Payment(#[from] payments::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ServiceError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::InvalidRequest(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable(message.into())
    }
}

// Custom error types for specific services
#[derive(Error, Debug)]
pub enum LearningError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("OCR error: {0}")]
    OCRError(String),

    #[error("Audio processing error: {0}")]
    AudioError(String),

    #[error("LLM error: {0}")]
    LLMError(String),
}

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Stripe error: {0}")]
    StripeError(String),

    #[error("Crypto error: {0}")]
    CryptoError(String),

    #[error("Invalid payment method: {0}")]
    InvalidPaymentMethod(String),

    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
}

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("Sync error: {0}")]
    SyncError(String),

    #[error("Backup error: {0}")]
    BackupError(String),
}

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("Audio device error: {0}")]
    AudioDeviceError(String),

    #[error("Stream initialization error: {0}")]
    InitializationError(String),

    #[error("Buffer overflow: {0}")]
    BufferOverflow(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),
} 