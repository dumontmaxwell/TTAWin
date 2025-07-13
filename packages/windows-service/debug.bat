@echo off
REM TTAWin Windows Service Quick Debug Commands

if "%1"=="" goto help
if "%1"=="help" goto help
if "%1"=="build" goto build
if "%1"=="run" goto run
if "%1"=="debug" goto debug
if "%1"=="dev" goto dev
if "%1"=="kill" goto kill
if "%1"=="test" goto test
if "%1"=="clean" goto clean
if "%1"=="logs" goto logs

:help
echo TTAWin Windows Service Quick Debug Commands
echo ===========================================
echo.
echo Usage: debug.bat [command]
echo.
echo Commands:
echo   build    - Build the service in debug mode
echo   run      - Run the service in debug mode
echo   debug    - Build and run in debug mode
echo   dev      - Build and run in development mode
echo   kill     - Kill running service instances
echo   test     - Run tests
echo   clean    - Clean build artifacts
echo   logs     - Show recent logs
echo   help     - Show this help
echo.
echo Examples:
echo   debug.bat debug    - Quick debug run
echo   debug.bat dev      - Development mode
echo   debug.bat kill     - Stop all instances
echo.
goto end

:build
echo Building TTAWin service...
cargo build
if %ERRORLEVEL% neq 0 (
    echo Build failed!
    goto end
)
echo Build completed successfully!
goto end

:run
echo Running TTAWin service in debug mode...
target\debug\windows-service.exe --debug
goto end

:debug
echo Building and running in debug mode...
cargo build
if %ERRORLEVEL% neq 0 (
    echo Build failed!
    goto end
)
echo Starting debug mode...
target\debug\windows-service.exe --debug
goto end

:dev
echo Building and running in development mode...
cargo build
if %ERRORLEVEL% neq 0 (
    echo Build failed!
    goto end
)
echo Starting development mode...
target\debug\windows-service.exe --dev
goto end

:kill
echo Stopping TTAWin service instances...
taskkill /f /im windows-service.exe 2>nul
echo Service instances stopped.
goto end

:test
echo Running tests...
cargo test
if %ERRORLEVEL% neq 0 (
    echo Tests failed!
    goto end
)
echo Tests completed successfully!
goto end

:clean
echo Cleaning build artifacts...
cargo clean
echo Clean completed!
goto end

:logs
echo Recent TTAWin service logs:
if exist ttawin-debug.log (
    echo Console output:
    type ttawin-debug.log
) else (
    echo No log file found.
)
goto end

:end 