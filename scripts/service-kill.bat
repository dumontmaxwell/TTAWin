@echo off
REM TTAWin Service Kill Script - Quick Start
REM This script kills all running TTAWin service instances

echo TTAWin Service Kill - Quick Start
echo =================================
echo.

REM Change to the windows-service directory
cd packages\windows-service

REM Check if the debug script exists
if not exist "debug.bat" (
    echo Error: debug.bat not found in packages\windows-service
    echo Please ensure you're running this from the TTAWin root directory
    pause
    exit /b 1
)

REM Run the kill command
echo Stopping all TTAWin service instances...
echo.
debug.bat kill

REM Also try to kill any remaining processes
echo.
echo Checking for any remaining processes...
taskkill /f /im windows-service.exe 2>nul
if %ERRORLEVEL% equ 0 (
    echo Stopped remaining windows-service.exe processes
) else (
    echo No remaining windows-service.exe processes found
)

REM Return to root directory
cd ..\..

echo.
echo Service kill operation completed.
pause 