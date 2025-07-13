use std::ffi::OsString;
use std::sync::Arc;
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
    },
    service_control_handler::{self, ServiceControlHandler},
    service_dispatcher,
};
use windows_service::service::ServiceStartType;

use windows_service::Result as WindowsServiceResult;

mod config;
mod http_server;
mod services;
mod error;

use crate::config::ServiceConfig;
use crate::error::ServiceError;
use crate::http_server::HttpServer;
use crate::services::{LearningService, PaymentService, SettingsService, StreamService};

const SERVICE_NAME: &str = "TTAWinService";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

define_windows_service!(ffi_service_main, service_main);

#[derive(Debug, Clone)]
enum RunMode {
    Service,
    Debug,
    Development,
}

fn main() -> Result<(), ServiceError> {
    let args: Vec<String> = std::env::args().collect();
    let mode = determine_run_mode(&args);

    // Initialize logging based on mode
    match mode {
        RunMode::Service => {
            tracing_subscriber::fmt()
                .with_env_filter("windows_service=debug,info")
                .init();
        }
        RunMode::Debug => {
            tracing_subscriber::fmt()
                .with_env_filter("debug")
                .with_ansi(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .init();
        }
        RunMode::Development => {
            tracing_subscriber::fmt()
                .with_env_filter("debug")
                .with_ansi(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
                .init();
        }
    }

    tracing::info!("Starting TTAWin in {:?} mode", mode);

    match mode {
        RunMode::Service => {
            // Register the service
            service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
        }
        RunMode::Debug | RunMode::Development => {
            // Run directly without Windows service infrastructure
            if let Err(e) = run_standalone(mode) {
                tracing::error!("Standalone mode failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn determine_run_mode(args: &[String]) -> RunMode {
    if args.len() > 1 {
        match args[1].as_str() {
            "--debug" | "-d" => RunMode::Debug,
            "--dev" | "--development" => RunMode::Development,
            "--service" | "-s" => RunMode::Service,
            _ => {
                println!("Usage: {} [--debug|--dev|--service]", args[0]);
                println!("  --debug, -d     Run in debug mode with detailed logging");
                println!("  --dev, --development  Run in development mode with hot reload");
                println!("  --service, -s   Run as Windows service (default)");
                std::process::exit(1);
            }
        }
    } else {
        RunMode::Service
    }
}

fn service_main(arguments: Vec<OsString>) {
    tracing::info!("Service main function called with arguments: {:?}", arguments);

    if let Err(e) = run_service() {
        tracing::error!("Service failed: {}", e);
    }
}

fn run_service() -> Result<(), ServiceError> {
    run_standalone(RunMode::Service)
}

fn run_standalone(mode: RunMode) -> Result<(), ServiceError> {
    // Create a channel to receive shutdown signal
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

    // Load configuration
    let config = ServiceConfig::load()?;
    tracing::info!("Service configuration loaded: {:?}", config);

    // Create service instances
    let learning_service = Arc::new(LearningService::new(config.learning.clone())?);
    let payment_service = Arc::new(PaymentService::new(config.payments.clone())?);
    let settings_service = Arc::new(SettingsService::new(config.settings.clone())?);
    let stream_service = Arc::new(StreamService::new(config.stream.clone())?);

    // Create HTTP server
    let http_server = HttpServer::new(
        config.server.clone(),
        learning_service,
        payment_service,
        settings_service,
        stream_service,
    )?;

    // Set up shutdown handling based on mode
    let shutdown_tx_clone = shutdown_tx.clone();
    match mode {
        RunMode::Service => {
            // Create service control handler for Windows service
            let event_handler = move |control_event| -> ServiceControlHandlerResult {
                match control_event {
                    ServiceControl::Stop => {
                        tracing::info!("Received stop signal");
                        let _ = shutdown_tx_clone.send(());
                        ServiceControlHandlerResult::NoError
                    }
                    _ => ServiceControlHandlerResult::NotImplemented,
                }
            };

            let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

            // Update service status to running
            status_handle.set_service_status(ServiceStatus {
                service_type: SERVICE_TYPE,
                current_state: ServiceState::Running,
                controls_accepted: ServiceControlAccept::STOP,
                exit_code: ServiceExitCode::Win32(0),
                checkpoint: 0,
                wait_hint: std::time::Duration::default(),
                process_id: None,
            })?;
        }
        RunMode::Debug | RunMode::Development => {
            // Set up signal handling for standalone mode
            let shutdown_tx_signal = shutdown_tx_clone;
            ctrlc::set_handler(move || {
                tracing::info!("Received Ctrl+C, shutting down...");
                let _ = shutdown_tx_signal.send(());
            }).expect("Error setting Ctrl-C handler");

            tracing::info!("🚀 TTAWin Service running in {:?} mode", mode);
            tracing::info!("📡 HTTP Server: http://{}:{}", config.server.host, config.server.port);
            tracing::info!("🔍 Health check: http://{}:{}/health", config.server.host, config.server.port);
            tracing::info!("⏹️  Press Ctrl+C to stop the service");
            
            if mode == RunMode::Development {
                tracing::info!("🔄 Development mode: Configuration changes will be auto-reloaded");
            }
        }
    }

    tracing::info!("Service is now running");

    // Start HTTP server
    let server_handle = tokio::spawn(async move {
        if let Err(e) = http_server.run(shutdown_rx).await {
            tracing::error!("HTTP server error: {}", e);
        }
    });

    // Wait for shutdown signal
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        let _ = server_handle.await;
    });

    // Update service status to stopped (only for service mode)
    if let RunMode::Service = mode {
        // This would be handled by the service control handler in real service mode
        tracing::info!("Service stopped successfully");
    } else {
        tracing::info!("Standalone service stopped successfully");
    }

    Ok(())
}

type ServiceControlHandlerResult = windows_service::Result<()>; 