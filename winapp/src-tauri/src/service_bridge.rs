use crate::api_response::ApiResponse;
use crate::service_client::get_service_client;
use serde_json::Value;
use std::collections::HashMap;

/// Service bridge that provides unified interface for both direct and service-based operations
pub struct ServiceBridge {
    use_service: bool,
}

impl ServiceBridge {
    /// Create a new service bridge
    pub fn new(use_service: bool) -> Self {
        Self { use_service }
    }

    /// Create a service bridge that prefers the Windows service
    pub fn new_with_service() -> Self {
        Self { use_service: true }
    }

    /// Create a service bridge that uses direct package calls
    pub fn new_direct() -> Self {
        Self { use_service: false }
    }

    /// Check if the service is available and should be used
    async fn should_use_service(&self) -> bool {
        if !self.use_service {
            return false;
        }

        // Try to check service health
        match get_service_client().health_check().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    // ===== LEARNING OPERATIONS =====

    /// Analyze content using either service or direct package
    pub async fn analyze_content(
        &self,
        content_type: &str,
        content: &str,
        session_id: &str,
    ) -> Result<ApiResponse<Value>, String> {
        if self.should_use_service().await {
            // Use service
            match get_service_client()
                .analyze_content(content_type, content, session_id)
                .await
            {
                Ok(response) => Ok(response),
                Err(e) => Err(format!("Service analysis failed: {}", e)),
            }
        } else {
            // Use direct package (fallback)
            self.analyze_content_direct(content_type, content, session_id)
                .await
        }
    }

    /// Direct package analysis (fallback)
    async fn analyze_content_direct(
        &self,
        content_type: &str,
        content: &str,
        session_id: &str,
    ) -> Result<ApiResponse<Value>, String> {
        // This would use the learning package directly
        // For now, return a mock response
        let result = serde_json::json!({
            "analysis": {
                "content_type": content_type,
                "session_id": session_id,
                "summary": format!("Analysis of {} content", content_type),
                "insights": vec!["Insight 1", "Insight 2"],
                "confidence": 0.85
            }
        });

        Ok(ApiResponse::success(result))
    }

    /// Process OCR using either service or direct package
    pub async fn process_ocr(
        &self,
        image_path: &str,
        session_id: &str,
    ) -> Result<ApiResponse<Value>, String> {
        if self.should_use_service().await {
            // Use service
            match get_service_client().process_ocr(image_path, session_id).await {
                Ok(response) => Ok(response),
                Err(e) => Err(format!("Service OCR failed: {}", e)),
            }
        } else {
            // Use direct package (fallback)
            self.process_ocr_direct(image_path, session_id).await
        }
    }

    /// Direct package OCR (fallback)
    async fn process_ocr_direct(
        &self,
        image_path: &str,
        session_id: &str,
    ) -> Result<ApiResponse<Value>, String> {
        // This would use the learning package directly
        // For now, return a mock response
        let result = serde_json::json!({
            "ocr": {
                "image_path": image_path,
                "session_id": session_id,
                "text": "Sample OCR text from image",
                "confidence": 0.92,
                "words": vec!["Sample", "OCR", "text", "from", "image"]
            }
        });

        Ok(ApiResponse::success(result))
    }

    // ===== PAYMENT OPERATIONS =====

    /// Process payment using either service or direct package
    pub async fn process_payment(
        &self,
        amount: u64,
        currency: &str,
        description: &str,
        customer_email: &str,
        payment_method: &str,
        crypto_details: Option<HashMap<String, String>>,
    ) -> Result<ApiResponse<Value>, String> {
        if self.should_use_service().await {
            // Use service
            match payment_method {
                "stripe" => {
                    get_service_client()
                        .process_stripe_payment(amount, currency, description, customer_email)
                        .await
                        .map_err(|e| format!("Service Stripe payment failed: {}", e))
                }
                "crypto" => {
                    if let Some(details) = crypto_details {
                        let crypto_currency = details.get("currency").unwrap_or(&"bitcoin".to_string());
                        let network = details.get("network").unwrap_or(&"bitcoin".to_string());
                        let wallet_address = details.get("wallet_address").unwrap_or(&"".to_string());

                        get_service_client()
                            .process_crypto_payment(
                                amount,
                                currency,
                                description,
                                customer_email,
                                crypto_currency,
                                network,
                                wallet_address,
                            )
                            .await
                            .map_err(|e| format!("Service crypto payment failed: {}", e))
                    } else {
                        Err("Crypto details required for crypto payment".to_string())
                    }
                }
                _ => Err(format!("Unsupported payment method: {}", payment_method)),
            }
        } else {
            // Use direct package (fallback)
            self.process_payment_direct(amount, currency, description, customer_email, payment_method, crypto_details)
                .await
        }
    }

    /// Direct package payment (fallback)
    async fn process_payment_direct(
        &self,
        amount: u64,
        currency: &str,
        description: &str,
        customer_email: &str,
        payment_method: &str,
        crypto_details: Option<HashMap<String, String>>,
    ) -> Result<ApiResponse<Value>, String> {
        // This would use the payments package directly
        // For now, return a mock response
        let result = serde_json::json!({
            "payment": {
                "amount": amount,
                "currency": currency,
                "description": description,
                "customer_email": customer_email,
                "payment_method": payment_method,
                "status": "success",
                "transaction_id": "mock_txn_123456",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });

        Ok(ApiResponse::success(result))
    }

    // ===== SETTINGS OPERATIONS =====

    /// Get configuration using either service or direct package
    pub async fn get_configuration(&self) -> Result<ApiResponse<Value>, String> {
        if self.should_use_service().await {
            // Use service
            match get_service_client().get_configuration().await {
                Ok(response) => Ok(response),
                Err(e) => Err(format!("Service config retrieval failed: {}", e)),
            }
        } else {
            // Use direct package (fallback)
            self.get_configuration_direct().await
        }
    }

    /// Direct package configuration (fallback)
    async fn get_configuration_direct(&self) -> Result<ApiResponse<Value>, String> {
        // This would load configuration from local files
        // For now, return a mock response
        let result = serde_json::json!({
            "config": {
                "server": {
                    "host": "127.0.0.1",
                    "port": 8080
                },
                "learning": {
                    "model_path": "data/models",
                    "log_level": "info"
                },
                "payments": {
                    "stripe_enabled": true,
                    "crypto_enabled": true
                }
            }
        });

        Ok(ApiResponse::success(result))
    }

    /// Update configuration using either service or direct package
    pub async fn update_configuration(
        &self,
        config: HashMap<String, Value>,
    ) -> Result<ApiResponse<Value>, String> {
        if self.should_use_service().await {
            // Use service
            match get_service_client().update_configuration(config).await {
                Ok(response) => Ok(response),
                Err(e) => Err(format!("Service config update failed: {}", e)),
            }
        } else {
            // Use direct package (fallback)
            self.update_configuration_direct(config).await
        }
    }

    /// Direct package configuration update (fallback)
    async fn update_configuration_direct(
        &self,
        config: HashMap<String, Value>,
    ) -> Result<ApiResponse<Value>, String> {
        // This would save configuration to local files
        // For now, return a mock response
        let result = serde_json::json!({
            "config_updated": true,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "changes": config
        });

        Ok(ApiResponse::success(result))
    }

    // ===== STREAM OPERATIONS =====

    /// Start audio stream using either service or direct package
    pub async fn start_audio_stream(
        &self,
        session_id: &str,
        buffer_size: Option<u32>,
        sample_rate: Option<u32>,
    ) -> Result<ApiResponse<Value>, String> {
        if self.should_use_service().await {
            // Use service
            match get_service_client()
                .start_audio_stream(session_id, buffer_size, sample_rate)
                .await
            {
                Ok(response) => Ok(response),
                Err(e) => Err(format!("Service stream start failed: {}", e)),
            }
        } else {
            // Use direct package (fallback)
            self.start_audio_stream_direct(session_id, buffer_size, sample_rate)
                .await
        }
    }

    /// Direct package stream start (fallback)
    async fn start_audio_stream_direct(
        &self,
        session_id: &str,
        buffer_size: Option<u32>,
        sample_rate: Option<u32>,
    ) -> Result<ApiResponse<Value>, String> {
        // This would use the streams package directly
        // For now, return a mock response
        let result = serde_json::json!({
            "stream": {
                "session_id": session_id,
                "status": "started",
                "buffer_size": buffer_size.unwrap_or(2048),
                "sample_rate": sample_rate.unwrap_or(16000),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });

        Ok(ApiResponse::success(result))
    }

    /// Stop audio stream using either service or direct package
    pub async fn stop_audio_stream(&self, session_id: &str) -> Result<ApiResponse<Value>, String> {
        if self.should_use_service().await {
            // Use service
            match get_service_client().stop_audio_stream(session_id).await {
                Ok(response) => Ok(response),
                Err(e) => Err(format!("Service stream stop failed: {}", e)),
            }
        } else {
            // Use direct package (fallback)
            self.stop_audio_stream_direct(session_id).await
        }
    }

    /// Direct package stream stop (fallback)
    async fn stop_audio_stream_direct(&self, session_id: &str) -> Result<ApiResponse<Value>, String> {
        // This would use the streams package directly
        // For now, return a mock response
        let result = serde_json::json!({
            "stream": {
                "session_id": session_id,
                "status": "stopped",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });

        Ok(ApiResponse::success(result))
    }
}

/// Global service bridge instance
pub static mut SERVICE_BRIDGE: Option<ServiceBridge> = None;

/// Initialize the global service bridge
pub fn init_service_bridge(use_service: bool) {
    unsafe {
        SERVICE_BRIDGE = Some(ServiceBridge::new(use_service));
    }
}

/// Get the global service bridge
pub fn get_service_bridge() -> &'static ServiceBridge {
    unsafe {
        SERVICE_BRIDGE.as_ref().expect("Service bridge not initialized")
    }
} 