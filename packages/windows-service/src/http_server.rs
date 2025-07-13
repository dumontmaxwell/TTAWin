use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::oneshot;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::config::ServerConfig;
use crate::error::ServiceError;
use crate::services::{LearningService, PaymentService, SettingsService, StreamService};

pub struct HttpServer {
    config: ServerConfig,
    learning_service: Arc<LearningService>,
    payment_service: Arc<PaymentService>,
    settings_service: Arc<SettingsService>,
    stream_service: Arc<StreamService>,
}

impl HttpServer {
    pub fn new(
        config: ServerConfig,
        learning_service: Arc<LearningService>,
        payment_service: Arc<PaymentService>,
        settings_service: Arc<SettingsService>,
        stream_service: Arc<StreamService>,
    ) -> Result<Self, ServiceError> {
        Ok(Self {
            config,
            learning_service,
            payment_service,
            settings_service,
            stream_service,
        })
    }

    pub async fn run(self, shutdown_rx: oneshot::Receiver<()>) -> Result<(), ServiceError> {
        let app = self.create_router();

        let addr = format!("{}:{}", self.config.host, self.config.port)
            .parse()
            .map_err(|e| ServiceError::Internal(format!("Invalid address: {}", e)))?;

        tracing::info!("Starting HTTP server on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
                tracing::info!("Shutting down HTTP server");
            })
            .await
            .map_err(ServiceError::HttpServer)?;

        Ok(())
    }

    fn create_router(self) -> Router {
        // CORS configuration
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        Router::new()
            // Health check
            .route("/health", get(Self::health_check))
            
            // Learning endpoints
            .route("/learning/analyze", post(Self::analyze_content))
            .route("/learning/ocr", post(Self::extract_text))
            .route("/learning/audio", post(Self::transcribe_audio))
            .route("/learning/summary", post(Self::generate_summary))
            .route("/learning/insights", post(Self::generate_insights))
            .route("/learning/session/:session_id", get(Self::get_session))
            .route("/learning/session/:session_id", delete(Self::clear_session))
            
            // Payment endpoints
            .route("/payments/process", post(Self::process_payment))
            .route("/payments/status/:payment_id", get(Self::get_payment_status))
            .route("/payments/refund/:payment_id", post(Self::refund_payment))
            .route("/payments/methods", get(Self::get_payment_methods))
            .route("/payments/currencies", get(Self::get_supported_currencies))
            .route("/payments/wallet/:currency", get(Self::get_wallet_info))
            .route("/payments/wallet/:currency", post(Self::create_wallet))
            
            // Settings endpoints
            .route("/settings", get(Self::get_settings))
            .route("/settings", put(Self::update_settings))
            .route("/settings/backup", post(Self::create_backup))
            .route("/settings/restore", post(Self::restore_backup))
            .route("/settings/sync", post(Self::sync_settings))
            .route("/settings/reset", post(Self::reset_settings))
            
            // Stream endpoints
            .route("/stream/start", post(Self::start_stream))
            .route("/stream/stop", post(Self::stop_stream))
            .route("/stream/status", get(Self::get_stream_status))
            .route("/stream/audio", get(Self::get_audio_stream))
            .route("/stream/transcribe", post(Self::stream_transcribe))
            
            // File management endpoints
            .route("/files/upload", post(Self::upload_file))
            .route("/files/:file_id", get(Self::get_file))
            .route("/files/:file_id", delete(Self::delete_file))
            .route("/files/list", get(Self::list_files))
            
            // System endpoints
            .route("/system/status", get(Self::get_system_status))
            .route("/system/logs", get(Self::get_logs))
            .route("/system/restart", post(Self::restart_service))
            .route("/system/shutdown", post(Self::shutdown_service))
            
            .layer(cors)
            .layer(TraceLayer::new_for_http())
            .with_state(Arc::new(self))
    }

    // Health check endpoint
    async fn health_check() -> impl IntoResponse {
        Json(serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": env!("CARGO_PKG_VERSION")
        }))
    }

    // Learning endpoints
    async fn analyze_content(
        State(state): State<Arc<Self>>,
        Json(payload): Json<AnalyzeRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Analyzing content: {:?}", payload.content_type);
        
        // TODO: Implement actual analysis logic
        let result = state.learning_service.analyze_content(&payload).await?;
        
        Ok(Json(result))
    }

    async fn extract_text(
        State(state): State<Arc<Self>>,
        Json(payload): Json<OcrRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Extracting text from image");
        
        // TODO: Implement OCR logic
        let result = state.learning_service.extract_text(&payload).await?;
        
        Ok(Json(result))
    }

    async fn transcribe_audio(
        State(state): State<Arc<Self>>,
        Json(payload): Json<AudioRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Transcribing audio");
        
        // TODO: Implement audio transcription logic
        let result = state.learning_service.transcribe_audio(&payload).await?;
        
        Ok(Json(result))
    }

    async fn generate_summary(
        State(state): State<Arc<Self>>,
        Json(payload): Json<SummaryRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Generating summary");
        
        // TODO: Implement summary generation logic
        let result = state.learning_service.generate_summary(&payload).await?;
        
        Ok(Json(result))
    }

    async fn generate_insights(
        State(state): State<Arc<Self>>,
        Json(payload): Json<InsightsRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Generating insights");
        
        // TODO: Implement insights generation logic
        let result = state.learning_service.generate_insights(&payload).await?;
        
        Ok(Json(result))
    }

    async fn get_session(
        State(state): State<Arc<Self>>,
        Path(session_id): Path<String>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting session: {}", session_id);
        
        // TODO: Implement session retrieval logic
        let result = state.learning_service.get_session(&session_id).await?;
        
        Ok(Json(result))
    }

    async fn clear_session(
        State(state): State<Arc<Self>>,
        Path(session_id): Path<String>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Clearing session: {}", session_id);
        
        // TODO: Implement session clearing logic
        state.learning_service.clear_session(&session_id).await?;
        
        Ok(StatusCode::NO_CONTENT)
    }

    // Payment endpoints
    async fn process_payment(
        State(state): State<Arc<Self>>,
        Json(payload): Json<PaymentRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Processing payment: {:?}", payload.payment_method);
        
        // TODO: Implement payment processing logic
        let result = state.payment_service.process_payment(&payload).await?;
        
        Ok(Json(result))
    }

    async fn get_payment_status(
        State(state): State<Arc<Self>>,
        Path(payment_id): Path<String>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting payment status: {}", payment_id);
        
        // TODO: Implement payment status retrieval logic
        let result = state.payment_service.get_payment_status(&payment_id).await?;
        
        Ok(Json(result))
    }

    async fn refund_payment(
        State(state): State<Arc<Self>>,
        Path(payment_id): Path<String>,
        Json(payload): Json<RefundRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Processing refund: {}", payment_id);
        
        // TODO: Implement refund logic
        let result = state.payment_service.refund_payment(&payment_id, &payload).await?;
        
        Ok(Json(result))
    }

    async fn get_payment_methods(
        State(state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting supported payment methods");
        
        // TODO: Implement payment methods retrieval logic
        let result = state.payment_service.get_supported_methods().await?;
        
        Ok(Json(result))
    }

    async fn get_supported_currencies(
        State(state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting supported currencies");
        
        // TODO: Implement currencies retrieval logic
        let result = state.payment_service.get_supported_currencies().await?;
        
        Ok(Json(result))
    }

    async fn get_wallet_info(
        State(state): State<Arc<Self>>,
        Path(currency): Path<String>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting wallet info for: {}", currency);
        
        // TODO: Implement wallet info retrieval logic
        let result = state.payment_service.get_wallet_info(&currency).await?;
        
        Ok(Json(result))
    }

    async fn create_wallet(
        State(state): State<Arc<Self>>,
        Path(currency): Path<String>,
        Json(payload): Json<CreateWalletRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Creating wallet for: {}", currency);
        
        // TODO: Implement wallet creation logic
        let result = state.payment_service.create_wallet(&currency, &payload).await?;
        
        Ok(Json(result))
    }

    // Settings endpoints
    async fn get_settings(
        State(state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting settings");
        
        // TODO: Implement settings retrieval logic
        let result = state.settings_service.get_settings().await?;
        
        Ok(Json(result))
    }

    async fn update_settings(
        State(state): State<Arc<Self>>,
        Json(payload): Json<UpdateSettingsRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Updating settings");
        
        // TODO: Implement settings update logic
        let result = state.settings_service.update_settings(&payload).await?;
        
        Ok(Json(result))
    }

    async fn create_backup(
        State(state): State<Arc<Self>>,
        Json(payload): Json<BackupRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Creating backup");
        
        // TODO: Implement backup creation logic
        let result = state.settings_service.create_backup(&payload).await?;
        
        Ok(Json(result))
    }

    async fn restore_backup(
        State(state): State<Arc<Self>>,
        Json(payload): Json<RestoreRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Restoring backup");
        
        // TODO: Implement backup restoration logic
        let result = state.settings_service.restore_backup(&payload).await?;
        
        Ok(Json(result))
    }

    async fn sync_settings(
        State(state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Syncing settings");
        
        // TODO: Implement settings sync logic
        let result = state.settings_service.sync_settings().await?;
        
        Ok(Json(result))
    }

    async fn reset_settings(
        State(state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Resetting settings");
        
        // TODO: Implement settings reset logic
        state.settings_service.reset_settings().await?;
        
        Ok(StatusCode::NO_CONTENT)
    }

    // Stream endpoints
    async fn start_stream(
        State(state): State<Arc<Self>>,
        Json(payload): Json<StartStreamRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Starting stream");
        
        // TODO: Implement stream start logic
        let result = state.stream_service.start_stream(&payload).await?;
        
        Ok(Json(result))
    }

    async fn stop_stream(
        State(state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Stopping stream");
        
        // TODO: Implement stream stop logic
        state.stream_service.stop_stream().await?;
        
        Ok(StatusCode::NO_CONTENT)
    }

    async fn get_stream_status(
        State(state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting stream status");
        
        // TODO: Implement stream status retrieval logic
        let result = state.stream_service.get_status().await?;
        
        Ok(Json(result))
    }

    async fn get_audio_stream(
        State(state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting audio stream");
        
        // TODO: Implement audio stream retrieval logic
        let result = state.stream_service.get_audio_stream().await?;
        
        Ok(Json(result))
    }

    async fn stream_transcribe(
        State(state): State<Arc<Self>>,
        Json(payload): Json<StreamTranscribeRequest>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Stream transcribing");
        
        // TODO: Implement stream transcription logic
        let result = state.stream_service.transcribe_stream(&payload).await?;
        
        Ok(Json(result))
    }

    // File management endpoints
    async fn upload_file(
        State(state): State<Arc<Self>>,
        headers: HeaderMap,
        body: axum::body::Bytes,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Uploading file");
        
        // TODO: Implement file upload logic
        let result = state.settings_service.upload_file(headers, body).await?;
        
        Ok(Json(result))
    }

    async fn get_file(
        State(state): State<Arc<Self>>,
        Path(file_id): Path<String>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting file: {}", file_id);
        
        // TODO: Implement file retrieval logic
        let result = state.settings_service.get_file(&file_id).await?;
        
        Ok(Json(result))
    }

    async fn delete_file(
        State(state): State<Arc<Self>>,
        Path(file_id): Path<String>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Deleting file: {}", file_id);
        
        // TODO: Implement file deletion logic
        state.settings_service.delete_file(&file_id).await?;
        
        Ok(StatusCode::NO_CONTENT)
    }

    async fn list_files(
        State(state): State<Arc<Self>>,
        Query(query): Query<ListFilesQuery>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Listing files");
        
        // TODO: Implement file listing logic
        let result = state.settings_service.list_files(&query).await?;
        
        Ok(Json(result))
    }

    // System endpoints
    async fn get_system_status(
        State(_state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting system status");
        
        // TODO: Implement system status retrieval logic
        let status = SystemStatus {
            service: "running".to_string(),
            uptime: chrono::Utc::now().timestamp(),
            memory_usage: 1024 * 1024 * 100, // 100MB
            cpu_usage: 5.0,
            active_connections: 0,
        };
        
        Ok(Json(status))
    }

    async fn get_logs(
        State(_state): State<Arc<Self>>,
        Query(query): Query<LogsQuery>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Getting logs");
        
        // TODO: Implement log retrieval logic
        let logs = vec![
            LogEntry {
                timestamp: chrono::Utc::now().to_rfc3339(),
                level: "INFO".to_string(),
                message: "Service started successfully".to_string(),
            }
        ];
        
        Ok(Json(logs))
    }

    async fn restart_service(
        State(_state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Restarting service");
        
        // TODO: Implement service restart logic
        // This would typically involve sending a signal to the main process
        
        Ok(Json(serde_json::json!({
            "message": "Service restart initiated",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    }

    async fn shutdown_service(
        State(_state): State<Arc<Self>>,
    ) -> Result<impl IntoResponse, ServiceError> {
        tracing::info!("Shutting down service");
        
        // TODO: Implement service shutdown logic
        // This would typically involve sending a signal to the main process
        
        Ok(Json(serde_json::json!({
            "message": "Service shutdown initiated",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    }
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub content: String,
    pub content_type: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OcrRequest {
    pub image_path: String,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AudioRequest {
    pub audio_path: String,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SummaryRequest {
    pub content: String,
    pub max_length: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct InsightsRequest {
    pub content: String,
    pub insight_types: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub amount: u64,
    pub currency: String,
    pub payment_method: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct RefundRequest {
    pub amount: Option<u64>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub currency: String,
    pub network: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct BackupRequest {
    pub include_data: bool,
    pub include_config: bool,
}

#[derive(Debug, Deserialize)]
pub struct RestoreRequest {
    pub backup_path: String,
    pub restore_config: bool,
}

#[derive(Debug, Deserialize)]
pub struct StartStreamRequest {
    pub stream_type: String,
    pub config: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct StreamTranscribeRequest {
    pub audio_data: Vec<u8>,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct ListFilesQuery {
    pub path: Option<String>,
    pub recursive: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct LogsQuery {
    pub level: Option<String>,
    pub limit: Option<usize>,
    pub since: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SystemStatus {
    pub service: String,
    pub uptime: i64,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub active_connections: u32,
}

#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
} 