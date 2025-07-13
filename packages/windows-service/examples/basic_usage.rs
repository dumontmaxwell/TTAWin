use windows_service::{
    config::ServiceConfig,
    error::ServiceError,
    services::{LearningService, PaymentService, SettingsService, StreamService},
    VERSION,
};

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    println!("🚀 TTAWin Windows Service Demo");
    println!("=============================\n");

    // Load configuration
    println!("📋 Loading configuration...");
    let config = ServiceConfig::load()?;
    println!("✅ Configuration loaded successfully!");
    println!("   Server: {}:{}", config.server.host, config.server.port);
    println!("   Learning: {} concurrent analyses", config.learning.max_concurrent_analyses);
    println!("   Payments: {} currencies supported", config.payments.supported_currencies.len());
    println!("   Stream: {}", if config.stream.enabled { "enabled" } else { "disabled" });
    println!();

    // Initialize services
    println!("🔧 Initializing services...");
    
    let learning_service = LearningService::new(config.learning.clone())?;
    println!("   ✅ Learning service initialized");
    
    let payment_service = PaymentService::new(config.payments.clone())?;
    println!("   ✅ Payment service initialized");
    
    let settings_service = SettingsService::new(config.settings.clone())?;
    println!("   ✅ Settings service initialized");
    
    let stream_service = StreamService::new(config.stream.clone())?;
    println!("   ✅ Stream service initialized");
    println!();

    // Demo: Learning service
    println!("🧠 Learning Service Demo");
    println!("----------------------");
    
    let analyze_request = windows_service::http_server::AnalyzeRequest {
        content: "The user is working on a complex coding project and making good progress.".to_string(),
        content_type: "text".to_string(),
        session_id: Some("demo-session-123".to_string()),
    };
    
    match learning_service.analyze_content(&analyze_request).await {
        Ok(result) => {
            println!("   ✅ Content analysis completed");
            println!("   📊 Result: {}", serde_json::to_string_pretty(&result).unwrap());
        }
        Err(e) => {
            println!("   ❌ Analysis failed: {}", e);
        }
    }
    println!();

    // Demo: Payment service
    println!("💳 Payment Service Demo");
    println!("---------------------");
    
    let payment_request = windows_service::http_server::PaymentRequest {
        amount: 1000, // $10.00
        currency: "usd".to_string(),
        payment_method: "stripe".to_string(),
        description: Some("Demo payment".to_string()),
        metadata: None,
    };
    
    match payment_service.process_payment(&payment_request).await {
        Ok(result) => {
            println!("   ✅ Payment processed");
            println!("   📊 Result: {}", serde_json::to_string_pretty(&result).unwrap());
        }
        Err(e) => {
            println!("   ❌ Payment failed: {}", e);
        }
    }
    println!();

    // Demo: Settings service
    println!("⚙️  Settings Service Demo");
    println!("-----------------------");
    
    match settings_service.get_settings().await {
        Ok(result) => {
            println!("   ✅ Settings retrieved");
            println!("   📊 Result: {}", serde_json::to_string_pretty(&result).unwrap());
        }
        Err(e) => {
            println!("   ❌ Settings retrieval failed: {}", e);
        }
    }
    println!();

    // Demo: Stream service
    println!("🎤 Stream Service Demo");
    println!("--------------------");
    
    let start_stream_request = windows_service::http_server::StartStreamRequest {
        stream_type: "audio".to_string(),
        config: None,
    };
    
    match stream_service.start_stream(&start_stream_request).await {
        Ok(result) => {
            println!("   ✅ Stream started");
            println!("   📊 Result: {}", serde_json::to_string_pretty(&result).unwrap());
            
            // Get stream status
            match stream_service.get_status().await {
                Ok(status) => {
                    println!("   📊 Status: {}", serde_json::to_string_pretty(&status).unwrap());
                }
                Err(e) => {
                    println!("   ❌ Status check failed: {}", e);
                }
            }
            
            // Stop stream
            if let Err(e) = stream_service.stop_stream().await {
                println!("   ❌ Stream stop failed: {}", e);
            } else {
                println!("   ✅ Stream stopped");
            }
        }
        Err(e) => {
            println!("   ❌ Stream start failed: {}", e);
        }
    }
    println!();

    // Cleanup
    println!("🧹 Cleaning up services...");
    learning_service.dispose().await?;
    payment_service.dispose().await?;
    settings_service.dispose().await?;
    stream_service.dispose().await?;
    println!("   ✅ All services disposed successfully");
    println!();

    println!("🎉 Demo completed successfully!");
    println!("📚 The Windows service provides:");
    println!("   • Learning analysis with OCR and audio transcription");
    println!("   • Payment processing with Stripe and crypto support");
    println!("   • Settings management with backup and sync");
    println!("   • Audio streaming with real-time processing");
    println!("   • REST API endpoints for all functionality");
    println!("   • Windows service integration for reliability");

    Ok(())
} 