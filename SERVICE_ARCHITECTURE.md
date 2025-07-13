# TTAWin Service Architecture

## Overview

TTAWin now uses a **service-based architecture** where all core functionality (learning, payments, settings, streaming) is handled by a Windows service that runs independently of the frontend. The frontend acts as a thin client that communicates with the service via HTTP API calls.

## Architecture Diagram

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

## Key Components

### 1. Windows Service (`packages/windows-service/`)

The Windows service is the **single source of truth** for all application functionality:

- **Learning Service**: OCR, audio transcription, content analysis
- **Payment Service**: Stripe and cryptocurrency payments
- **Settings Service**: Configuration management and backups
- **Stream Service**: Real-time audio processing
- **HTTP Server**: REST API endpoints for all services

**Features:**
- Runs as a Windows service (managed by Service Control Manager)
- Provides REST API on localhost:8080
- Handles all data persistence and processing
- Supports debug and development modes
- Automatic restart and recovery

### 2. Service Client (`winapp/src-tauri/src/service_client.rs`)

HTTP client for communicating with the Windows service:

- **HTTP Client**: Makes requests to service endpoints
- **Request/Response Handling**: Serialization and error handling
- **Timeout Management**: Configurable timeouts and retries
- **Global Instance**: Singleton pattern for app-wide access

### 3. Service Bridge (`winapp/src-tauri/src/service_bridge.rs`)

Unified interface that provides fallback capabilities:

- **Service-First**: Prefers Windows service when available
- **Fallback Mode**: Uses direct package calls when service is unavailable
- **Unified API**: Same interface regardless of backend
- **Error Handling**: Graceful degradation between modes

### 4. Tauri Commands (`winapp/src-tauri/src/lib.rs`)

Three layers of Tauri commands:

1. **Unified Commands** (preferred): Use service bridge
   - `unified_analyze_content`
   - `unified_process_ocr`
   - `unified_process_payment`
   - `unified_get_config`
   - `unified_update_config`
   - `unified_start_audio_stream`
   - `unified_stop_audio_stream`

2. **Service Commands** (direct): Direct service access
   - `check_service_health`
   - `get_service_status`
   - `get_ai_models`
   - `analyze_content`
   - `process_ocr`
   - `get_payment_methods`
   - `process_stripe_payment`
   - `process_crypto_payment`
   - `get_service_config`
   - `update_service_config`
   - `start_service_audio_stream`
   - `stop_service_audio_stream`
   - `get_stream_status`

3. **Legacy Commands** (existing): Original functionality
   - `get_monitors`
   - `switch_monitor`
   - `test_hotkey`
   - `trigger_action`
   - `quit_app`
   - `set_click_through`
   - `set_overlay_hidden`
   - `set_overlay_visible`
   - `toggle_overlay`

### 5. Frontend Composable (`winapp/src/composables/useServiceBridge.ts`)

Vue composable for frontend service communication:

- **TypeScript Interfaces**: Strongly typed request/response objects
- **Error Handling**: Custom error classes and error state management
- **Loading States**: Built-in loading indicators
- **Service Health**: Automatic service availability checking
- **Unified API**: Same interface for all operations

## Data Flow

### 1. Service-First Flow (Preferred)

```
Frontend → Tauri Command → Service Bridge → Service Client → Windows Service → Response
```

### 2. Fallback Flow (When Service Unavailable)

```
Frontend → Tauri Command → Service Bridge → Direct Package → Response
```

### 3. Direct Service Flow (Bypass Bridge)

```
Frontend → Tauri Command → Service Client → Windows Service → Response
```

## Benefits

### 1. **Single Source of Truth**
- All data processing happens in the Windows service
- No data corruption from frontend crashes
- Centralized state management

### 2. **Reliability**
- Service runs independently of frontend
- Automatic restart on failure
- Graceful degradation with fallback mode

### 3. **Security**
- Sensitive operations isolated in service
- No direct package access from frontend
- Controlled API access

### 4. **Performance**
- Service can run optimized Rust code
- Background processing without UI blocking
- Efficient resource management

### 5. **Maintainability**
- Clear separation of concerns
- Modular service architecture
- Easy to add new functionality

## Migration Strategy

### Phase 1: Service Implementation ✅
- [x] Windows service with all endpoints
- [x] Service client for HTTP communication
- [x] Service bridge for unified interface
- [x] Tauri commands for all operations

### Phase 2: Frontend Integration ✅
- [x] Vue composable for service communication
- [x] TypeScript interfaces for type safety
- [x] Error handling and loading states
- [x] Service health monitoring

### Phase 3: Testing and Validation
- [ ] End-to-end testing of all flows
- [ ] Performance benchmarking
- [ ] Error scenario testing
- [ ] User acceptance testing

### Phase 4: Production Deployment
- [ ] Service installation automation
- [ ] Configuration management
- [ ] Monitoring and logging
- [ ] Backup and recovery procedures

## Usage Examples

### Frontend Usage

```typescript
import { useServiceBridge } from '@/composables/useServiceBridge'

const { analyzeContent, processPayment, isLoading, hasError } = useServiceBridge()

// Analyze content
const result = await analyzeContent({
  content_type: 'text',
  content: 'Sample content for analysis',
  session_id: 'session-123'
})

// Process payment
const payment = await processPayment({
  amount: 1000,
  currency: 'usd',
  description: 'Test payment',
  customer_email: 'test@example.com',
  payment_method: 'stripe'
})
```

### Service Debugging

```bash
# Start service in debug mode
cd packages/windows-service
debug.bat debug

# Start service in development mode with file watching
.\debug.ps1 dev -Watch

# Test service endpoints
.\test-service.ps1

# Check service health
curl http://localhost:8080/health
```

### Service Configuration

```toml
# config/dev.toml
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

## Troubleshooting

### Service Not Starting
1. Check if service is installed: `sc query TTAWinService`
2. Check service logs: `Get-EventLog -LogName Application -Source TTAWinService`
3. Run in debug mode: `debug.bat debug`

### Frontend Can't Connect
1. Check service health: `curl http://localhost:8080/health`
2. Verify service is running on correct port
3. Check firewall settings
4. Use fallback mode if service unavailable

### Performance Issues
1. Check service resource usage
2. Monitor API response times
3. Optimize service configuration
4. Consider service scaling

## Future Enhancements

1. **WebSocket Support**: Real-time communication
2. **Service Clustering**: Multiple service instances
3. **Load Balancing**: Distribute requests across services
4. **Service Discovery**: Automatic service detection
5. **Metrics Collection**: Performance monitoring
6. **Plugin System**: Extensible service architecture

## Conclusion

The service-based architecture provides a robust, scalable, and maintainable foundation for TTAWin. It ensures data integrity, improves reliability, and enables future enhancements while maintaining backward compatibility through the service bridge pattern. 