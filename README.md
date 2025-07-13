# TTAWin - AI-Powered Learning Assistant

TTAWin is a sophisticated Windows application that provides AI-powered learning assistance, payment processing, and real-time audio streaming capabilities. Built with a modern service-based architecture, it combines the power of Rust backend services with a Vue.js frontend.

## 🏗️ Architecture Overview

TTAWin uses a **service-based architecture** where all core functionality is handled by a Windows service that runs independently of the frontend. The frontend acts as a thin client that communicates with the service via HTTP API calls.

```
┌─────────────────┐    HTTP API    ┌──────────────────┐
│   TTAWin App    │ ◄────────────► │  Windows Service │
│   (Frontend)    │                │   (Backend)      │
│                 │                │                  │
│ ┌─────────────┐ │                │ ┌──────────────┐ │
│ │ Service     │ │                │ │ Learning     │ │
│ │ Bridge      │ │                │ │ Service      │ │
│ └─────────────┘ │                │ └──────────────┘ │
│ ┌─────────────┐ │                │ ┌──────────────┐ │
│ │ Tauri       │ │                │ │ Payment      │ │
│ │ Commands    │ │                │ │ Service      │ │
│ └─────────────┘ │                │ └──────────────┘ │
│ ┌─────────────┐ │                │ ┌──────────────┐ │
│ │ Vue UI      │ │                │ │ Settings     │ │
│ │ Components  │ │                │ │ Service      │ │
│ └─────────────┘ │                │ └──────────────┘ │
└─────────────────┘                │ ┌──────────────┐ │
                                   │ │ Stream       │ │
                                   │ │ Service      │ │
                                   │ └──────────────┘ │
                                   └──────────────────┘
```

## 📁 Project Structure

```
TTAWin/
├── Cargo.toml                    # Workspace configuration
├── README.md                     # This file
├── SERVICE_ARCHITECTURE.md       # Detailed service architecture
├── scripts/                      # Convenience scripts
│   ├── service-debug.bat         # Quick service debug
│   ├── service-dev.bat           # Quick service development
│   ├── service-test.bat          # Quick service testing
│   └── service-kill.bat          # Kill service instances
├── winapp/                       # Frontend application
│   ├── src-tauri/               # Tauri backend
│   │   ├── src/
│   │   │   ├── lib.rs           # Main Tauri commands
│   │   │   ├── service_client.rs # HTTP client for service
│   │   │   ├── service_bridge.rs # Unified service interface
│   │   │   └── ...
│   │   └── Cargo.toml
│   ├── src/
│   │   ├── composables/
│   │   │   └── useServiceBridge.ts # Vue composable for service
│   │   └── ...
│   └── package.json
└── packages/                     # Rust packages
    ├── learning/                 # AI/ML functionality
    │   ├── src/
    │   │   ├── lib.rs
    │   │   ├── analysis.rs       # Content analysis
    │   │   ├── audio.rs          # Audio processing
    │   │   ├── llm.rs            # LLM integration
    │   │   └── ocr.rs            # OCR processing
    │   └── Cargo.toml
    ├── payments/                 # Payment processing
    │   ├── src/
    │   │   ├── lib.rs
    │   │   ├── stripe.rs         # Stripe integration
    │   │   └── crypto.rs         # Cryptocurrency payments
    │   └── Cargo.toml
    └── windows-service/          # Windows service backend
        ├── src/
        │   ├── main.rs           # Service entry point
        │   ├── config.rs         # Configuration management
        │   ├── http_server.rs    # REST API server
        │   ├── services.rs       # Service implementations
        │   └── error.rs          # Error handling
        ├── config/
        │   └── dev.toml          # Development configuration
        ├── debug.ps1             # PowerShell debug script
        ├── debug.bat             # Batch debug script
        ├── test-service.ps1      # Service testing script
        ├── build.ps1             # Build script
        └── Cargo.toml
```

## 🚀 Quick Start

### Prerequisites

- **Windows 10/11**
- **Rust 1.70+** with Cargo
- **Node.js 18+** with npm
- **Administrator privileges** for service installation

### 1. Clone and Setup

```bash
git clone <repository-url>
cd TTAWin

# Install frontend dependencies
cd winapp
npm install
cd ..

# Build all Rust packages
cargo build
```

### 2. Start the Service

```bash
# Quick debug mode (recommended for development)
scripts/service-debug.bat

# Or use PowerShell for more options
cd packages/windows-service
.\debug.ps1 debug

# Or development mode with file watching
.\debug.ps1 dev -Watch
```

### 3. Start the Frontend

```bash
cd winapp
npm run tauri dev
```

### 4. Test the Setup

```bash
# Test service endpoints
cd packages/windows-service
.\test-service.ps1

# Check service health
curl http://localhost:8080/health
```

## 🛠️ Development Workflow

### Service Development

The Windows service supports multiple development modes:

```bash
# Debug mode (detailed logging)
scripts/service-debug.bat

# Development mode (hot reload)
scripts/service-dev.bat

# Test endpoints
scripts/service-test.bat

# Kill running instances
scripts/service-kill.bat
```

### Frontend Development

```bash
cd winapp

# Development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Package Development

```bash
# Build specific package
cargo build -p learning
cargo build -p payments
cargo build -p windows-service

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all
cargo clippy --all-targets --all-features
```

## 🔧 Configuration

### Service Configuration

The Windows service uses TOML configuration files:

```toml
# packages/windows-service/config/dev.toml
[server]
host = "127.0.0.1"
port = 8080

[learning]
model_path = "data/models"
log_level = "debug"

[payments]
stripe_secret_key = "sk_test_..."
crypto_enabled = true

[settings]
data_dir = "data"
backup_enabled = false

[stream]
enabled = true
buffer_size = 2048
```

### Environment Variables

```bash
# Service configuration
set TTAWIN_CONFIG=config/dev.toml
set TTAWIN_PORT=9090

# Debug settings
set RUST_LOG=debug
set RUST_BACKTRACE=1
```

## 📡 API Endpoints

The Windows service provides REST API endpoints:

### Health & System
- `GET /health` - Service health check
- `GET /system/status` - System status
- `GET /system/logs` - System logs

### Learning Service
- `GET /learning/models` - Available AI models
- `GET /learning/sessions` - Learning sessions
- `POST /learning/analyze` - Content analysis
- `POST /learning/ocr` - OCR processing
- `POST /learning/transcribe` - Audio transcription

### Payment Service
- `GET /payments/methods` - Payment methods
- `GET /payments/currencies` - Supported currencies
- `POST /payments/process` - Process payment
- `GET /payments/status/{id}` - Payment status

### Settings Service
- `GET /settings/config` - Get configuration
- `PUT /settings/config` - Update configuration
- `GET /settings/backup` - List backups
- `POST /settings/backup` - Create backup
- `POST /settings/backup/restore` - Restore backup

### Stream Service
- `GET /stream/status` - Stream status
- `GET /stream/sessions` - Stream sessions
- `POST /stream/start` - Start audio stream
- `POST /stream/stop` - Stop audio stream
- `GET /stream/transcription/{id}` - Stream transcription

## 🎯 Key Features

### 🤖 AI-Powered Learning
- **Content Analysis**: Text, image, and audio analysis
- **OCR Processing**: Text extraction from images
- **Audio Transcription**: Speech-to-text conversion
- **Summary Generation**: AI-powered content summarization
- **Insights Generation**: Intelligent insights and recommendations

### 💳 Payment Processing
- **Stripe Integration**: Traditional payment processing
- **Cryptocurrency Support**: Bitcoin, Ethereum, USDC, USDT, DAI
- **Multi-currency Support**: USD, EUR, BTC, ETH, and more
- **Payment Status Tracking**: Real-time payment monitoring
- **Refund Processing**: Payment refunds and cancellations

### ⚙️ Settings Management
- **Configuration Management**: Service settings and preferences
- **Backup & Restore**: Data backup and restoration
- **Settings Synchronization**: Cross-device settings sync
- **File Management**: Upload, download, and file organization

### 🎵 Audio Streaming
- **Real-time Audio**: Live audio capture and processing
- **Stream Management**: Start, stop, and status monitoring
- **Audio Transcription**: Real-time speech-to-text
- **Buffer Management**: Optimized audio buffer handling

### 🖥️ Windows Integration
- **Overlay System**: Transparent overlay for learning assistance
- **Hotkey Support**: Global keyboard shortcuts
- **Click-through**: Background interaction capability
- **Multi-monitor**: Support for multiple displays

## 🔍 Debugging & Troubleshooting

### Service Issues

```bash
# Check service status
sc query TTAWinService

# View service logs
Get-EventLog -LogName Application -Source TTAWinService

# Run in debug mode
scripts/service-debug.bat

# Test endpoints
scripts/service-test.bat
```

### Frontend Issues

```bash
# Check service connectivity
curl http://localhost:8080/health

# View browser console
# Open DevTools in the Tauri app

# Restart frontend
cd winapp
npm run tauri dev
```

### Common Problems

1. **Service won't start**: Check if port 8080 is available
2. **Frontend can't connect**: Verify service is running and accessible
3. **Permission errors**: Run as administrator for service installation
4. **Build failures**: Ensure all dependencies are installed

## 🧪 Testing

### Service Testing

```bash
# Run all service tests
cd packages/windows-service
.\test-service.ps1

# Test specific endpoints
.\test-service.ps1 -Health
.\test-service.ps1 -Learning
.\test-service.ps1 -Payments
```

### Package Testing

```bash
# Run all tests
cargo test --workspace

# Run specific package tests
cargo test -p learning
cargo test -p payments
cargo test -p windows-service
```

### Integration Testing

```bash
# Start service and test frontend integration
scripts/service-debug.bat
# In another terminal:
cd winapp
npm run tauri dev
```

## 📦 Building & Deployment

### Development Build

```bash
# Build all packages
cargo build

# Build frontend
cd winapp
npm run tauri build
```

### Production Build

```bash
# Build optimized service
cd packages/windows-service
cargo build --release

# Build production frontend
cd winapp
npm run tauri build --release
```

### Service Installation

```bash
# Install as Windows service (requires admin)
cd packages/windows-service
.\install-service.ps1
```

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Test thoroughly**: Use the provided debugging tools
5. **Commit your changes**: `git commit -m 'Add amazing feature'`
6. **Push to the branch**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

### Development Guidelines

- **Service-first architecture**: All new features should use the service layer
- **Error handling**: Implement proper error handling and fallbacks
- **Testing**: Write tests for new functionality
- **Documentation**: Update documentation for new features
- **Type safety**: Use TypeScript interfaces for frontend communication

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: See [SERVICE_ARCHITECTURE.md](SERVICE_ARCHITECTURE.md) for detailed architecture
- **Issues**: Report bugs and feature requests via GitHub Issues
- **Discussions**: Join discussions in GitHub Discussions
- **Wiki**: Check the project wiki for additional resources

## 🚀 Roadmap

- [ ] **WebSocket Support**: Real-time communication
- [ ] **Service Clustering**: Multiple service instances
- [ ] **Plugin System**: Extensible service architecture
- [ ] **Mobile Support**: Cross-platform compatibility
- [ ] **Cloud Integration**: Remote service capabilities
- [ ] **Advanced AI**: Enhanced learning algorithms
- [ ] **Analytics**: Usage analytics and insights
- [ ] **Collaboration**: Multi-user learning sessions

---

**TTAWin** - Empowering learning through intelligent assistance and seamless technology integration. 