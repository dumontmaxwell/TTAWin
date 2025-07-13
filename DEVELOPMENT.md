# TTAWin Development Guide

This guide provides comprehensive instructions for developing, testing, and debugging TTAWin.

## 🚀 Quick Start

### One-Command Setup
```bash
# Clone and setup everything
git clone <repository-url>
cd TTAWin
scripts/quick-start.bat
```

### Manual Setup
```bash
# 1. Install dependencies
cd winapp && npm install && cd ..

# 2. Build Rust packages
cargo build

# 3. Start service
scripts/service-debug.bat

# 4. Start frontend (in new terminal)
cd winapp && npm run tauri dev
```

## 🛠️ Development Commands

### Service Management

#### Quick Commands (from root)
```bash
# Start service in debug mode
scripts/service-debug.bat

# Start service in development mode with file watching
scripts/service-dev.bat

# Test all service endpoints
scripts/service-test.bat

# Kill all service instances
scripts/service-kill.bat

# Quick health check
scripts/service-manager.ps1 health
```

#### Advanced Service Management
```bash
# PowerShell service manager (from root)
.\scripts\service-manager.ps1 debug                    # Debug mode
.\scripts\service-manager.ps1 dev -Watch               # Development with watching
.\scripts\service-manager.ps1 test                     # Test endpoints
.\scripts\service-manager.ps1 kill                     # Kill instances
.\scripts\service-manager.ps1 build                    # Build service
.\scripts\service-manager.ps1 clean                    # Clean build artifacts
.\scripts\service-manager.ps1 logs                     # Show logs
.\scripts\service-manager.ps1 health                   # Check health
.\scripts\service-manager.ps1 install                  # Install as Windows service
```

#### Direct Service Commands (from packages/windows-service)
```bash
# Debug mode
debug.bat debug

# Development mode
debug.bat dev

# PowerShell with more options
.\debug.ps1 debug
.\debug.ps1 dev -Watch
.\debug.ps1 debug -Background
.\debug.ps1 debug -Port 9090
.\debug.ps1 debug -Config "config/dev.toml"
```

### Frontend Development

```bash
# Development mode
cd winapp
npm run tauri dev

# Build for production
npm run tauri build

# Install dependencies
npm install
```

### Rust Package Development

```bash
# Build specific packages
cargo build -p learning
cargo build -p payments
cargo build -p windows-service

# Build all packages
cargo build --workspace

# Build optimized
cargo build --release --workspace

# Run tests
cargo test --workspace
cargo test -p learning
cargo test -p payments
cargo test -p windows-service

# Code quality
cargo fmt --all
cargo clippy --all-targets --all-features
cargo check --workspace
```

## 🔧 Configuration

### Service Configuration

The service uses TOML configuration files located in `packages/windows-service/config/`:

```toml
# config/dev.toml (Development)
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

# Frontend settings
set VITE_API_URL=http://localhost:8080
```

## 🧪 Testing

### Service Testing

```bash
# Test all endpoints
scripts/service-test.bat

# Test specific services
.\scripts\service-manager.ps1 test

# Manual API testing
curl http://localhost:8080/health
curl http://localhost:8080/system/status
```

### Package Testing

```bash
# Run all tests
cargo test --workspace

# Run specific package tests
cargo test -p learning
cargo test -p payments
cargo test -p windows-service

# Run with output
cargo test --workspace -- --nocapture
```

### Integration Testing

```bash
# 1. Start service
scripts/service-debug.bat

# 2. Start frontend (in new terminal)
cd winapp && npm run tauri dev

# 3. Test frontend-service communication
# Use the Vue composable: useServiceBridge()
```

## 🔍 Debugging

### Service Debugging

#### Debug Mode
```bash
# Start in debug mode with detailed logging
scripts/service-debug.bat

# Check service logs
scripts/service-manager.ps1 logs

# Monitor service health
scripts/service-manager.ps1 health
```

#### Development Mode
```bash
# Start with file watching (auto-restart on changes)
scripts/service-dev.bat

# Or use PowerShell
.\scripts\service-manager.ps1 dev -Watch
```

#### Troubleshooting
```bash
# Check if service is running
sc query TTAWinService

# View Windows Event Logs
Get-EventLog -LogName Application -Source TTAWinService

# Kill all instances
scripts/service-kill.bat

# Check port usage
netstat -ano | findstr :8080
```

### Frontend Debugging

#### Tauri DevTools
```bash
# Start with DevTools
cd winapp
npm run tauri dev

# DevTools will open automatically in the app
# Press F12 or right-click → Inspect
```

#### Service Communication
```typescript
// Use the service bridge composable
import { useServiceBridge } from '@/composables/useServiceBridge'

const { 
  analyzeContent, 
  processPayment, 
  isLoading, 
  hasError,
  checkServiceHealth 
} = useServiceBridge()

// Check service availability
await checkServiceHealth()

// Test content analysis
const result = await analyzeContent({
  content_type: 'text',
  content: 'Test content',
  session_id: 'test-123'
})
```

### Rust Debugging

#### Cargo Commands
```bash
# Check for compilation errors
cargo check --workspace

# Run with debug output
RUST_LOG=debug cargo run -p windows-service -- --debug

# Run specific examples
cargo run --example basic_usage -p windows-service
```

#### IDE Integration
- **VS Code**: Install Rust extension for debugging
- **IntelliJ**: Use Rust plugin for debugging
- **CLion**: Native Rust debugging support

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
.\scripts\service-manager.ps1 install

# Or manually
cd packages/windows-service
.\install-service.ps1
```

## 🔄 Development Workflow

### Typical Development Session

1. **Start Development Environment**
   ```bash
   scripts/quick-start.bat
   ```

2. **Make Changes**
   - Edit service code in `packages/windows-service/src/`
   - Edit frontend code in `winapp/src/`
   - Service auto-restarts on file changes (dev mode)

3. **Test Changes**
   ```bash
   # Test service endpoints
   scripts/service-test.bat
   
   # Test frontend integration
   # Use the app UI to test features
   ```

4. **Debug Issues**
   ```bash
   # Check service logs
   scripts/service-manager.ps1 logs
   
   # Check service health
   scripts/service-manager.ps1 health
   ```

5. **Clean Up**
   ```bash
   # Stop all services
   scripts/service-kill.bat
   ```

### Code Quality Workflow

```bash
# 1. Format code
cargo fmt --all

# 2. Run linter
cargo clippy --all-targets --all-features

# 3. Run tests
cargo test --workspace

# 4. Check types
cargo check --workspace

# 5. Build
cargo build --workspace
```

## 🐛 Common Issues & Solutions

### Service Won't Start
```bash
# Check if port is in use
netstat -ano | findstr :8080

# Kill processes using the port
taskkill /f /pid <PID>

# Check service logs
scripts/service-manager.ps1 logs
```

### Frontend Can't Connect to Service
```bash
# Check service health
scripts/service-manager.ps1 health

# Verify service is running
curl http://localhost:8080/health

# Check firewall settings
# Allow TTAWin through Windows Firewall
```

### Build Failures
```bash
# Clean and rebuild
cargo clean --workspace
cargo build --workspace

# Update dependencies
cargo update

# Check Rust version
rustc --version
```

### Permission Errors
```bash
# Run as administrator for service installation
# Right-click PowerShell → Run as Administrator
.\scripts\service-manager.ps1 install
```

## 📚 Additional Resources

- **Architecture Documentation**: [SERVICE_ARCHITECTURE.md](SERVICE_ARCHITECTURE.md)
- **API Documentation**: Service endpoints at `http://localhost:8080`
- **Tauri Documentation**: [https://tauri.app/docs](https://tauri.app/docs)
- **Rust Documentation**: [https://doc.rust-lang.org](https://doc.rust-lang.org)
- **Vue.js Documentation**: [https://vuejs.org/guide](https://vuejs.org/guide)

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
- **Code formatting**: Run `cargo fmt` before committing
- **Linting**: Fix all clippy warnings

---

**Happy coding!** 🚀 