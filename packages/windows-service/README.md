# TTAWin Windows Service

A Windows service that provides the backend functionality for TTAWin, including learning analysis, payment processing, settings management, and audio streaming. This service runs independently of the frontend and provides REST API endpoints for all core functionality.

## 🏗️ Architecture

The Windows service is designed as a modular backend that:

- **Runs as a Windows Service** - Managed by the Windows Service Control Manager
- **Provides REST API Endpoints** - Accessible via HTTP on localhost
- **Integrates with Learning Package** - OCR, audio transcription, and LLM analysis
- **Integrates with Payments Package** - Stripe and cryptocurrency payments
- **Manages Local Settings** - Configuration, backups, and file management
- **Handles Audio Streaming** - Real-time audio processing and transcription

### Service Structure

```
packages/windows-service/
├── src/
│   ├── main.rs           # Service entry point and Windows service registration
│   ├── config.rs         # Configuration management
│   ├── error.rs          # Error handling and custom error types
│   ├── http_server.rs    # REST API server with all endpoints
│   └── services.rs       # Service implementations (Learning, Payment, Settings, Stream)
├── Cargo.toml           # Dependencies and package configuration
└── README.md           # This file
```

## 🚀 Features

### Learning Service (`/learning/*`)
- **Content Analysis** - Text, screenshot, and audio analysis
- **OCR Processing** - Text extraction from images
- **Audio Transcription** - Speech-to-text conversion
- **Summary Generation** - Content summarization
- **Insights Generation** - AI-powered insights and recommendations
- **Session Management** - Conversation and analysis history

### Payment Service (`/payments/*`)
- **Stripe Integration** - Traditional payment processing
- **Cryptocurrency Support** - Bitcoin, Ethereum, USDC, USDT, DAI
- **Payment Status Tracking** - Real-time payment monitoring
- **Refund Processing** - Payment refunds and cancellations
- **Wallet Management** - Crypto wallet creation and management
- **Multi-currency Support** - USD, EUR, BTC, ETH, and more

### Settings Service (`/settings/*`)
- **Configuration Management** - Service settings and preferences
- **Backup & Restore** - Data backup and restoration
- **Settings Synchronization** - Cross-device settings sync
- **File Management** - Upload, download, and file organization
- **Data Persistence** - Local storage and caching

### Stream Service (`/stream/*`)
- **Audio Streaming** - Real-time audio capture and processing
- **Stream Management** - Start, stop, and status monitoring
- **Audio Transcription** - Real-time speech-to-text
- **Buffer Management** - Audio buffer handling and optimization

### System Management (`/system/*`)
- **Health Monitoring** - Service health and status
- **Log Management** - Log retrieval and filtering
- **Service Control** - Restart and shutdown capabilities

## 🐛 Debugging and Development

The Windows service supports multiple run modes for debugging and development:

### Run Modes

- **Service Mode** (default): Runs as a Windows service
- **Debug Mode**: Runs standalone with detailed logging
- **Development Mode**: Runs standalone with hot reload capabilities

### Quick Debug Commands

```bash
# Navigate to the windows-service directory
cd packages/windows-service

# Quick debug run (build + run in debug mode)
debug.bat debug

# Development mode with hot reload
debug.bat dev

# Kill running instances
debug.bat kill

# Show logs
debug.bat logs

# Run tests
debug.bat test
```

### Advanced Debugging with PowerShell

```powershell
# Run in debug mode
.\debug.ps1 debug

# Run in development mode with file watching
.\debug.ps1 dev -Watch

# Run in background
.\debug.ps1 debug -Background

# Use custom port
.\debug.ps1 debug -Port 9090

# Use custom config
.\debug.ps1 debug -Config "config/dev.toml"

# Kill all instances
.\debug.ps1 -Kill

# Show logs
.\debug.ps1 -Logs

# Clean build artifacts
.\debug.ps1 -Clean
```

### Manual Debug Commands

```bash
# Build in debug mode
cargo build

# Run in debug mode
target\debug\windows-service.exe --debug

# Run in development mode
target\debug\windows-service.exe --dev

# Run as Windows service
target\debug\windows-service.exe --service
```

### Environment Variables for Debugging

```bash
# Set debug logging
set RUST_LOG=debug

# Enable backtraces
set RUST_BACKTRACE=1

# Use custom config
set TTAWIN_CONFIG=config/dev.toml

# Use custom port
set TTAWIN_PORT=9090
```

### Debug Features

- **Detailed Logging**: Full request/response logging in debug mode
- **Error Backtraces**: Stack traces for debugging errors
- **File Watching**: Automatic restart on file changes (development mode)
- **Background Mode**: Run service in background for testing
- **Port Override**: Use custom ports for testing
- **Config Override**: Use custom configuration files
- **Process Management**: Easy start/stop/kill commands

## 📦 Installation

### Prerequisites

- Windows 10/11
- Rust 1.70+ with Cargo
- Administrator privileges for service installation

### Building the Service

```bash
# Navigate to the windows-service directory
cd packages/windows-service

# Build the service
cargo build --release

# The executable will be in target/release/windows-service.exe
```

### Installing as a Windows Service

```bash
# Run as administrator
# Install the service
sc create TTAWinService binPath= "C:\path\to\windows-service.exe" start= auto

# Start the service
sc start TTAWinService

# Check service status
sc query TTAWinService
```

### Manual Installation Script

Create a PowerShell script (`install-service.ps1`):

```powershell
# Run as Administrator
$serviceName = "TTAWinService"
$exePath = "C:\path\to\windows-service.exe"

# Stop and remove existing service if it exists
if (Get-Service -Name $serviceName -ErrorAction SilentlyContinue) {
    Stop-Service -Name $serviceName -Force
    sc.exe delete $serviceName
}

# Create the service
sc.exe create $serviceName binPath= $exePath start= auto DisplayName= "TTAWin Backend Service"

# Set service description
sc.exe description $serviceName "TTAWin Backend Service - Provides learning, payment, and streaming functionality"

# Start the service
Start-Service -Name $serviceName

Write-Host "Service installed and started successfully!"
```

## ⚙️ Configuration

The service uses a TOML configuration file located at `config/service.toml` relative to the executable.

### Default Configuration

```toml
[server]
host = "127.0.0.1"
port = 8080
cors_origins = ["http://localhost:3000", "http://127.0.0.1:3000"]
max_connections = 1000

[learning]
model_path = "data/models"
cache_dir = "data/cache"
max_concurrent_analyses = 4
enable_gpu = false
log_level = "info"

[payments]
stripe_secret_key = ""
crypto_enabled = true
supported_currencies = ["usd", "btc", "eth", "usdc"]
webhook_url = ""

[settings]
data_dir = "data"
backup_enabled = true
auto_sync = true
sync_interval_seconds = 300

[stream]
enabled = true
buffer_size = 4096
sample_rate = 16000
channels = 1
```

### Configuration Options

#### Server Configuration
- `host`: HTTP server host address
- `port`: HTTP server port number
- `cors_origins`: Allowed CORS origins
- `max_connections`: Maximum concurrent connections

#### Learning Configuration
- `model_path`: Path to AI models
- `cache_dir`: Cache directory for analysis results
- `max_concurrent_analyses`: Maximum concurrent analysis operations
- `enable_gpu`: Enable GPU acceleration for AI models
- `log_level`: Logging level (debug, info, warn, error)

#### Payment Configuration
- `stripe_secret_key`: Stripe secret key for payment processing
- `crypto_enabled`: Enable cryptocurrency payments
- `supported_currencies`: List of supported currencies
- `webhook_url`: Webhook URL for payment notifications

#### Settings Configuration
- `data_dir`: Data storage directory
- `backup_enabled`: Enable automatic backups
- `auto_sync`: Enable automatic settings synchronization
- `sync_interval_seconds`: Settings sync interval

#### Stream Configuration
- `enabled`: Enable audio streaming
- `buffer_size`: Audio buffer size in bytes
- `sample_rate`: Audio sample rate in Hz
- `channels`: Number of audio channels

## 🌐 API Endpoints

### Health Check
```
GET /health
```

### Learning Endpoints
```
POST /learning/analyze          # Analyze content (text, screenshot, audio)
POST /learning/ocr             # Extract text from images
POST /learning/audio           # Transcribe audio files
POST /learning/summary         # Generate content summaries
POST /learning/insights        # Generate AI insights
GET  /learning/session/{id}    # Get session data
DELETE /learning/session/{id}  # Clear session data
```

### Payment Endpoints
```
POST /payments/process         # Process payments
GET  /payments/status/{id}     # Get payment status
POST /payments/refund/{id}     # Process refunds
GET  /payments/methods         # Get supported payment methods
GET  /payments/currencies      # Get supported currencies
GET  /payments/wallet/{curr}   # Get wallet information
POST /payments/wallet/{curr}   # Create new wallet
```

### Settings Endpoints
```
GET  /settings                 # Get current settings
PUT  /settings                 # Update settings
POST /settings/backup          # Create backup
POST /settings/restore         # Restore from backup
POST /settings/sync            # Sync settings
POST /settings/reset           # Reset to defaults
```

### Stream Endpoints
```
POST /stream/start             # Start audio stream
POST /stream/stop              # Stop audio stream
GET  /stream/status            # Get stream status
GET  /stream/audio             # Get audio stream data
POST /stream/transcribe        # Transcribe stream audio
```

### File Management Endpoints
```
POST /files/upload             # Upload files
GET  /files/{id}               # Get file information
DELETE /files/{id}             # Delete files
GET  /files/list               # List files
```

### System Endpoints
```
GET  /system/status            # Get system status
GET  /system/logs              # Get service logs
POST /system/restart           # Restart service
POST /system/shutdown          # Shutdown service
```

## 🔧 Usage Examples

### Starting the Service

```bash
# Build and run in development mode
cargo run

# Run the compiled executable
./target/release/windows-service.exe

# Install and start as Windows service
sc create TTAWinService binPath= "C:\path\to\windows-service.exe" start= auto
sc start TTAWinService
```

### Testing the API

```bash
# Health check
curl http://localhost:8080/health

# Analyze text content
curl -X POST http://localhost:8080/learning/analyze \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello world", "content_type": "text", "session_id": "test-session"}'

# Process payment
curl -X POST http://localhost:8080/payments/process \
  -H "Content-Type: application/json" \
  -d '{"amount": 1000, "currency": "usd", "payment_method": "stripe", "description": "Test payment"}'

# Get system status
curl http://localhost:8080/system/status
```

### Frontend Integration

The frontend can communicate with the service using standard HTTP requests:

```javascript
// Example: Analyze content
const response = await fetch('http://localhost:8080/learning/analyze', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    content: 'User input text',
    content_type: 'text',
    session_id: 'user-session-123'
  })
});

const result = await response.json();
console.log('Analysis result:', result);
```

## 🛠️ Development

### Building for Development

```bash
# Development build with debug information
cargo build

# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Check for issues
cargo clippy
cargo fmt
```

### Debugging the Service

```bash
# View service logs
Get-EventLog -LogName Application -Source TTAWinService

# Check service status
sc query TTAWinService

# Stop service for debugging
sc stop TTAWinService

# Run manually for debugging
./target/debug/windows-service.exe
```

### Adding New Endpoints

1. Add the endpoint to `http_server.rs`
2. Implement the handler function
3. Add corresponding service method in `services.rs`
4. Update request/response types as needed

## 🔒 Security Considerations

- **Local Access Only**: Service runs on localhost by default
- **CORS Configuration**: Configure allowed origins in settings
- **API Authentication**: Consider adding authentication for production use
- **Secure Configuration**: Store sensitive keys in environment variables
- **File Permissions**: Ensure proper file system permissions

## 🚨 Troubleshooting

### Common Issues

**Service won't start:**
- Check if port 8080 is available
- Verify configuration file exists and is valid
- Check Windows Event Log for errors

**API requests fail:**
- Verify service is running: `sc query TTAWinService`
- Check if port is accessible: `netstat -an | findstr 8080`
- Verify CORS configuration for frontend requests

**Learning features not working:**
- Ensure AI models are downloaded to the model path
- Check GPU drivers if GPU acceleration is enabled
- Verify sufficient disk space for cache

**Payment processing fails:**
- Verify Stripe API key is configured
- Check network connectivity for crypto price feeds
- Ensure wallet addresses are valid

### Logs and Debugging

```bash
# View service logs
Get-EventLog -LogName Application -Source TTAWinService

# Enable debug logging
# Edit config/service.toml and set log_level = "debug"

# Check service dependencies
sc qc TTAWinService
```

## 📈 Performance

### Optimization Tips

- **GPU Acceleration**: Enable GPU for AI model inference
- **Connection Pooling**: Configure appropriate max_connections
- **Caching**: Utilize the built-in caching system
- **Resource Limits**: Monitor memory and CPU usage

### Monitoring

- **System Status**: Use `/system/status` endpoint
- **Logs**: Monitor application logs for errors
- **Resource Usage**: Monitor CPU, memory, and disk usage
- **Network**: Monitor API request/response times

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For questions, issues, or feature requests:

1. Check the troubleshooting section
2. Review the API documentation
3. Search existing issues
4. Create a new issue with detailed information

---

**Built with ❤️ for TTAWin - Empowering developers with intelligent local services.** 