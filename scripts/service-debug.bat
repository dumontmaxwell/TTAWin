@echo off
REM TTAWin Service Debug Script - Quick Start
REM This script starts the Windows service in debug mode from the root directory

echo TTAWin Service Debug - Quick Start
echo =================================
echo.

REM Change to the windows-service directory
cd packages\windows-service

REM Check if the directory exists
if not exist "debug.bat" (
    echo Error: debug.bat not found in packages\windows-service
    echo Please ensure you're running this from the TTAWin root directory
    pause
    exit /b 1
)

REM Run the debug script
echo Starting TTAWin service in debug mode...
echo.
debug.bat debug

REM Return to root directory
cd ..\..

echo.
echo Service debug session ended.
pause 