@echo off
REM TTAWin Quick Start Script
REM This script sets up and starts the entire TTAWin application

echo TTAWin Quick Start
echo ==================
echo.

REM Check if we're in the right directory
if not exist "packages\windows-service" (
    echo Error: TTAWin root directory not found
    echo Please run this script from the TTAWin root directory
    pause
    exit /b 1
)

echo 🚀 Setting up TTAWin...
echo.

REM Step 1: Install frontend dependencies
echo 📦 Installing frontend dependencies...
cd winapp
call npm install
if %ERRORLEVEL% neq 0 (
    echo ❌ Failed to install frontend dependencies
    pause
    exit /b 1
)
cd ..

REM Step 2: Build Rust packages
echo 🔨 Building Rust packages...
call cargo build
if %ERRORLEVEL% neq 0 (
    echo ❌ Failed to build Rust packages
    pause
    exit /b 1
)

REM Step 3: Start the service in background
echo 🚀 Starting TTAWin service...
start "TTAWin Service" cmd /c "scripts\service-debug.bat"

REM Wait a moment for service to start
echo ⏳ Waiting for service to start...
timeout /t 3 /nobreak >nul

REM Step 4: Test service health
echo 🏥 Testing service health...
powershell -Command "try { $response = Invoke-RestMethod -Uri 'http://localhost:8080/health' -Method GET -TimeoutSec 5; Write-Host '✅ Service is healthy' -ForegroundColor Green } catch { Write-Host '❌ Service not responding yet' -ForegroundColor Yellow }"

REM Step 5: Start the frontend
echo 🖥️ Starting TTAWin frontend...
cd winapp
start "TTAWin Frontend" cmd /c "npm run tauri dev"
cd ..

echo.
echo 🎉 TTAWin is starting up!
echo.
echo 📋 What's running:
echo    • TTAWin Service (background)
echo    • TTAWin Frontend (new window)
echo.
echo 🔧 Useful commands:
echo    • scripts\service-debug.bat     - Restart service
echo    • scripts\service-test.bat      - Test endpoints
echo    • scripts\service-kill.bat      - Stop service
echo    • scripts\service-manager.ps1   - Advanced management
echo.
echo 🌐 Service API: http://localhost:8080
echo 📖 Documentation: SERVICE_ARCHITECTURE.md
echo.
pause 