# TTAWin Windows Service Test Script
# Tests all API endpoints to verify the service is working correctly

param(
    [string]$BaseUrl = "http://localhost:8080",
    [switch]$Health,
    [switch]$Learning,
    [switch]$Payments,
    [switch]$Settings,
    [switch]$Stream,
    [switch]$All
)

Write-Host "TTAWin Windows Service Test Script" -ForegroundColor Green
Write-Host "===================================" -ForegroundColor Green
Write-Host "Base URL: $BaseUrl" -ForegroundColor Cyan
Write-Host ""

# Function to make HTTP requests
function Test-Endpoint {
    param(
        [string]$Method,
        [string]$Endpoint,
        [string]$Description,
        [object]$Body = $null
    )
    
    $url = "$BaseUrl$Endpoint"
    $headers = @{
        "Content-Type" = "application/json"
    }
    
    Write-Host "Testing: $Description" -ForegroundColor Yellow
    Write-Host "  $Method $url" -ForegroundColor Gray
    
    try {
        if ($Body) {
            $jsonBody = $Body | ConvertTo-Json -Depth 10
            $response = Invoke-RestMethod -Uri $url -Method $Method -Headers $headers -Body $jsonBody -TimeoutSec 10
        } else {
            $response = Invoke-RestMethod -Uri $url -Method $Method -Headers $headers -TimeoutSec 10
        }
        
        Write-Host "  ✅ Success" -ForegroundColor Green
        if ($response) {
            Write-Host "  📄 Response: $($response | ConvertTo-Json -Depth 2)" -ForegroundColor Gray
        }
    } catch {
        Write-Host "  ❌ Failed: $($_.Exception.Message)" -ForegroundColor Red
        if ($_.Exception.Response) {
            $statusCode = $_.Exception.Response.StatusCode
            Write-Host "  📊 Status Code: $statusCode" -ForegroundColor Red
        }
    }
    Write-Host ""
}

# Test health endpoint
if ($Health -or $All) {
    Write-Host "🏥 Health Check Tests" -ForegroundColor Magenta
    Write-Host "====================" -ForegroundColor Magenta
    
    Test-Endpoint -Method "GET" -Endpoint "/health" -Description "Health Check"
    Test-Endpoint -Method "GET" -Endpoint "/system/status" -Description "System Status"
    Test-Endpoint -Method "GET" -Endpoint "/system/logs" -Description "System Logs"
}

# Test learning endpoints
if ($Learning -or $All) {
    Write-Host "🧠 Learning Service Tests" -ForegroundColor Magenta
    Write-Host "=========================" -ForegroundColor Magenta
    
    Test-Endpoint -Method "GET" -Endpoint "/learning/models" -Description "List AI Models"
    Test-Endpoint -Method "GET" -Endpoint "/learning/sessions" -Description "List Sessions"
    
    # Test content analysis
    $analysisRequest = @{
        content_type = "text"
        content = "This is a test content for analysis."
        session_id = "test-session-123"
    }
    Test-Endpoint -Method "POST" -Endpoint "/learning/analyze" -Description "Content Analysis" -Body $analysisRequest
    
    # Test OCR
    $ocrRequest = @{
        image_path = "test-image.png"
        session_id = "test-session-123"
    }
    Test-Endpoint -Method "POST" -Endpoint "/learning/ocr" -Description "OCR Processing" -Body $ocrRequest
}

# Test payment endpoints
if ($Payments -or $All) {
    Write-Host "💳 Payment Service Tests" -ForegroundColor Magenta
    Write-Host "========================" -ForegroundColor Magenta
    
    Test-Endpoint -Method "GET" -Endpoint "/payments/methods" -Description "Payment Methods"
    Test-Endpoint -Method "GET" -Endpoint "/payments/currencies" -Description "Supported Currencies"
    
    # Test Stripe payment
    $stripeRequest = @{
        amount = 1000
        currency = "usd"
        description = "Test payment"
        customer_email = "test@example.com"
        payment_method = "stripe"
    }
    Test-Endpoint -Method "POST" -Endpoint "/payments/process" -Description "Stripe Payment" -Body $stripeRequest
    
    # Test crypto payment
    $cryptoRequest = @{
        amount = 1000
        currency = "usd"
        description = "Test crypto payment"
        customer_email = "test@example.com"
        payment_method = "crypto"
        crypto_details = @{
            currency = "bitcoin"
            network = "bitcoin"
            wallet_address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
        }
    }
    Test-Endpoint -Method "POST" -Endpoint "/payments/process" -Description "Crypto Payment" -Body $cryptoRequest
}

# Test settings endpoints
if ($Settings -or $All) {
    Write-Host "⚙️ Settings Service Tests" -ForegroundColor Magenta
    Write-Host "=========================" -ForegroundColor Magenta
    
    Test-Endpoint -Method "GET" -Endpoint "/settings/config" -Description "Get Configuration"
    Test-Endpoint -Method "GET" -Endpoint "/settings/backup" -Description "List Backups"
    
    # Test configuration update
    $configUpdate = @{
        server = @{
            port = 8080
        }
        learning = @{
            log_level = "debug"
        }
    }
    Test-Endpoint -Method "PUT" -Endpoint "/settings/config" -Description "Update Configuration" -Body $configUpdate
}

# Test stream endpoints
if ($Stream -or $All) {
    Write-Host "🎵 Stream Service Tests" -ForegroundColor Magenta
    Write-Host "=======================" -ForegroundColor Magenta
    
    Test-Endpoint -Method "GET" -Endpoint "/stream/status" -Description "Stream Status"
    Test-Endpoint -Method "GET" -Endpoint "/stream/sessions" -Description "Stream Sessions"
    
    # Test stream start
    $streamRequest = @{
        session_id = "test-stream-123"
        buffer_size = 2048
        sample_rate = 16000
    }
    Test-Endpoint -Method "POST" -Endpoint "/stream/start" -Description "Start Stream" -Body $streamRequest
    
    # Test stream stop
    $stopRequest = @{
        session_id = "test-stream-123"
    }
    Test-Endpoint -Method "POST" -Endpoint "/stream/stop" -Description "Stop Stream" -Body $stopRequest
}

# If no specific tests selected, run all
if (-not ($Health -or $Learning -or $Payments -or $Settings -or $Stream -or $All)) {
    Write-Host "No specific tests selected. Running all tests..." -ForegroundColor Yellow
    $All = $true
    & $PSCommandPath -BaseUrl $BaseUrl -All
}

Write-Host "🎉 Service testing completed!" -ForegroundColor Green
Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Yellow
Write-Host "  • Check the service logs for detailed information" -ForegroundColor White
Write-Host "  • Use the debug script to restart the service if needed" -ForegroundColor White
Write-Host "  • Test with the frontend application" -ForegroundColor White 