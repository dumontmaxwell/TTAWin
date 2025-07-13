@echo off
REM TTAWin Service Testing Script - Quick Start
REM This script tests all service endpoints from the root directory

echo TTAWin Service Testing - Quick Start
echo ===================================
echo.

REM Change to the windows-service directory
cd packages\windows-service

REM Check if the test script exists
if not exist "test-service.ps1" (
    echo Error: test-service.ps1 not found in packages\windows-service
    echo Please ensure you're running this from the TTAWin root directory
    pause
    exit /b 1
)

REM Run the PowerShell test script
echo Testing TTAWin service endpoints...
echo.
powershell -ExecutionPolicy Bypass -File "test-service.ps1"

REM Return to root directory
cd ..\..

echo.
echo Service testing completed.
pause 