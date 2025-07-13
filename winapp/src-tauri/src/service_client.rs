use crate::api_response::ApiResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::{timeout, Duration};

/// Service client configuration
#[derive(Debug, Clone)]
pub struct ServiceClientConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
}

impl Default for ServiceClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080".to_string(),
            timeout_seconds: 30,
            retry_attempts: 3,
        }
    }
}

/// Service client for communicating with the Windows service
pub struct ServiceClient {
    config: ServiceClientConfig,
    client: reqwest::Client,
}

impl ServiceClient {
    /// Create a new service client
    pub fn new(config: ServiceClientConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    /// Create a service client with default configuration
    pub fn new_default() -> Self {
        Self::new(ServiceClientConfig::default())
    }

    /// Make an HTTP request to the service
    async fn make_request<T, R>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<T>,
    ) -> Result<R, String>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.config.base_url, endpoint);
        let mut request = self.client.request(method, &url);

        if let Some(body_data) = body {
            request = request.json(&body_data);
        }

        let response = timeout(
            Duration::from_secs(self.config.timeout_seconds),
            request.send(),
        )
        .await
        .map_err(|_| "Request timeout".to_string())?
        .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Service returned error: {}",
                response.status()
            ));
        }

        let result: R = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(result)
    }

    // ===== HEALTH & SYSTEM ENDPOINTS =====

    /// Check service health
    pub async fn health_check(&self) -> Result<ApiResponse<serde_json::Value>, String> {
        self.make_request(reqwest::Method::GET, "/health", None::<()>)
            .await
    }

    /// Get system status
    pub async fn get_system_status(&self) -> Result<ApiResponse<serde_json::Value>, String> {
        self.make_request(reqwest::Method::GET, "/system/status", None::<()>)
            .await
    }

    /// Get system logs
    pub async fn get_system_logs(&self) -> Result<ApiResponse<Vec<String>>, String> {
        self.make_request(reqwest::Method::GET, "/system/logs", None::<()>)
            .await
    }

    // ===== LEARNING SERVICE ENDPOINTS =====

    /// Get available AI models
    pub async fn get_learning_models(&self) -> Result<ApiResponse<Vec<String>>, String> {
        self.make_request(reqwest::Method::GET, "/learning/models", None::<()>)
            .await
    }

    /// Get learning sessions
    pub async fn get_learning_sessions(&self) -> Result<ApiResponse<Vec<serde_json::Value>>, String> {
        self.make_request(reqwest::Method::GET, "/learning/sessions", None::<()>)
            .await
    }

    /// Analyze content
    pub async fn analyze_content(
        &self,
        content_type: &str,
        content: &str,
        session_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        #[derive(Serialize)]
        struct AnalysisRequest {
            content_type: String,
            content: String,
            session_id: String,
        }

        let request = AnalysisRequest {
            content_type: content_type.to_string(),
            content: content.to_string(),
            session_id: session_id.to_string(),
        };

        self.make_request(reqwest::Method::POST, "/learning/analyze", Some(request))
            .await
    }

    /// Process OCR
    pub async fn process_ocr(
        &self,
        image_path: &str,
        session_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        #[derive(Serialize)]
        struct OcrRequest {
            image_path: String,
            session_id: String,
        }

        let request = OcrRequest {
            image_path: image_path.to_string(),
            session_id: session_id.to_string(),
        };

        self.make_request(reqwest::Method::POST, "/learning/ocr", Some(request))
            .await
    }

    /// Transcribe audio
    pub async fn transcribe_audio(
        &self,
        audio_path: &str,
        session_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        #[derive(Serialize)]
        struct TranscriptionRequest {
            audio_path: String,
            session_id: String,
        }

        let request = TranscriptionRequest {
            audio_path: audio_path.to_string(),
            session_id: session_id.to_string(),
        };

        self.make_request(reqwest::Method::POST, "/learning/transcribe", Some(request))
            .await
    }

    // ===== PAYMENT SERVICE ENDPOINTS =====

    /// Get supported payment methods
    pub async fn get_payment_methods(&self) -> Result<ApiResponse<Vec<String>>, String> {
        self.make_request(reqwest::Method::GET, "/payments/methods", None::<()>)
            .await
    }

    /// Get supported currencies
    pub async fn get_supported_currencies(&self) -> Result<ApiResponse<Vec<String>>, String> {
        self.make_request(reqwest::Method::GET, "/payments/currencies", None::<()>)
            .await
    }

    /// Process Stripe payment
    pub async fn process_stripe_payment(
        &self,
        amount: u64,
        currency: &str,
        description: &str,
        customer_email: &str,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        #[derive(Serialize)]
        struct StripePaymentRequest {
            amount: u64,
            currency: String,
            description: String,
            customer_email: String,
            payment_method: String,
        }

        let request = StripePaymentRequest {
            amount,
            currency: currency.to_string(),
            description: description.to_string(),
            customer_email: customer_email.to_string(),
            payment_method: "stripe".to_string(),
        };

        self.make_request(reqwest::Method::POST, "/payments/process", Some(request))
            .await
    }

    /// Process crypto payment
    pub async fn process_crypto_payment(
        &self,
        amount: u64,
        currency: &str,
        description: &str,
        customer_email: &str,
        crypto_currency: &str,
        network: &str,
        wallet_address: &str,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        #[derive(Serialize)]
        struct CryptoPaymentRequest {
            amount: u64,
            currency: String,
            description: String,
            customer_email: String,
            payment_method: String,
            crypto_details: CryptoDetails,
        }

        #[derive(Serialize)]
        struct CryptoDetails {
            currency: String,
            network: String,
            wallet_address: String,
        }

        let request = CryptoPaymentRequest {
            amount,
            currency: currency.to_string(),
            description: description.to_string(),
            customer_email: customer_email.to_string(),
            payment_method: "crypto".to_string(),
            crypto_details: CryptoDetails {
                currency: crypto_currency.to_string(),
                network: network.to_string(),
                wallet_address: wallet_address.to_string(),
            },
        };

        self.make_request(reqwest::Method::POST, "/payments/process", Some(request))
            .await
    }

    /// Check payment status
    pub async fn check_payment_status(
        &self,
        payment_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        self.make_request(
            reqwest::Method::GET,
            &format!("/payments/status/{}", payment_id),
            None::<()>,
        )
        .await
    }

    // ===== SETTINGS SERVICE ENDPOINTS =====

    /// Get configuration
    pub async fn get_configuration(&self) -> Result<ApiResponse<serde_json::Value>, String> {
        self.make_request(reqwest::Method::GET, "/settings/config", None::<()>)
            .await
    }

    /// Update configuration
    pub async fn update_configuration(
        &self,
        config: HashMap<String, serde_json::Value>,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        self.make_request(reqwest::Method::PUT, "/settings/config", Some(config))
            .await
    }

    /// Get backups
    pub async fn get_backups(&self) -> Result<ApiResponse<Vec<serde_json::Value>>, String> {
        self.make_request(reqwest::Method::GET, "/settings/backup", None::<()>)
            .await
    }

    /// Create backup
    pub async fn create_backup(&self) -> Result<ApiResponse<serde_json::Value>, String> {
        self.make_request(reqwest::Method::POST, "/settings/backup", None::<()>)
            .await
    }

    /// Restore backup
    pub async fn restore_backup(&self, backup_id: &str) -> Result<ApiResponse<serde_json::Value>, String> {
        #[derive(Serialize)]
        struct RestoreRequest {
            backup_id: String,
        }

        let request = RestoreRequest {
            backup_id: backup_id.to_string(),
        };

        self.make_request(reqwest::Method::POST, "/settings/backup/restore", Some(request))
            .await
    }

    // ===== STREAM SERVICE ENDPOINTS =====

    /// Get stream status
    pub async fn get_stream_status(&self) -> Result<ApiResponse<serde_json::Value>, String> {
        self.make_request(reqwest::Method::GET, "/stream/status", None::<()>)
            .await
    }

    /// Get stream sessions
    pub async fn get_stream_sessions(&self) -> Result<ApiResponse<Vec<serde_json::Value>>, String> {
        self.make_request(reqwest::Method::GET, "/stream/sessions", None::<()>)
            .await
    }

    /// Start audio stream
    pub async fn start_audio_stream(
        &self,
        session_id: &str,
        buffer_size: Option<u32>,
        sample_rate: Option<u32>,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        #[derive(Serialize)]
        struct StreamStartRequest {
            session_id: String,
            buffer_size: Option<u32>,
            sample_rate: Option<u32>,
        }

        let request = StreamStartRequest {
            session_id: session_id.to_string(),
            buffer_size,
            sample_rate,
        };

        self.make_request(reqwest::Method::POST, "/stream/start", Some(request))
            .await
    }

    /// Stop audio stream
    pub async fn stop_audio_stream(&self, session_id: &str) -> Result<ApiResponse<serde_json::Value>, String> {
        #[derive(Serialize)]
        struct StreamStopRequest {
            session_id: String,
        }

        let request = StreamStopRequest {
            session_id: session_id.to_string(),
        };

        self.make_request(reqwest::Method::POST, "/stream/stop", Some(request))
            .await
    }

    /// Get stream transcription
    pub async fn get_stream_transcription(
        &self,
        session_id: &str,
    ) -> Result<ApiResponse<serde_json::Value>, String> {
        self.make_request(
            reqwest::Method::GET,
            &format!("/stream/transcription/{}", session_id),
            None::<()>,
        )
        .await
    }
}

/// Global service client instance
pub static mut SERVICE_CLIENT: Option<ServiceClient> = None;

/// Initialize the global service client
pub fn init_service_client() {
    unsafe {
        SERVICE_CLIENT = Some(ServiceClient::new_default());
    }
}

/// Get the global service client
pub fn get_service_client() -> &'static ServiceClient {
    unsafe {
        SERVICE_CLIENT.as_ref().expect("Service client not initialized")
    }
}

/// Check if the service is available
pub async fn is_service_available() -> bool {
    match get_service_client().health_check().await {
        Ok(_) => true,
        Err(_) => false,
    }
} 