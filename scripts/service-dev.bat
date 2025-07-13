@echo off
REM TTAWin Service Development Script - Quick Start
REM This script starts the Windows service in development mode with file watching

echo TTAWin Service Development - Quick Start
echo =======================================
echo.

REM Change to the windows-service directory
cd packages\windows-service

REM Check if the PowerShell script exists
if not exist "debug.ps1" (
    echo Error: debug.ps1 not found in packages\windows-service
    echo Please ensure you're running this from the TTAWin root directory
    pause
    exit /b 1
)

REM Run the PowerShell development script
echo Starting TTAWin service in development mode with file watching...
echo.
powershell -ExecutionPolicy Bypass -File "debug.ps1" dev -Watch

REM Return to root directory
cd ..\..

echo.
echo Service development session ended.
pause 