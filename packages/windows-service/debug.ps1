# TTAWin Windows Service Debug Script
# Provides various debugging and development modes for testing the service

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("debug", "dev", "development", "service", "test", "profile")]
    [string]$Mode = "debug",
    
    [Parameter(Mandatory=$false)]
    [string]$Config = "",
    
    [Parameter(Mandatory=$false)]
    [int]$Port = 0,
    
    [switch]$Watch,
    [switch]$Background,
    [switch]$Kill,
    [switch]$Logs,
    [switch]$Clean,
    [switch]$Help
)

# Show help if requested
if ($Help) {
    Write-Host "TTAWin Windows Service Debug Script" -ForegroundColor Green
    Write-Host "====================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Usage: .\debug.ps1 [Mode] [Options]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Modes:" -ForegroundColor Cyan
    Write-Host "  debug       Run in debug mode with detailed logging"
    Write-Host "  dev         Run in development mode with hot reload"
    Write-Host "  development Same as dev"
    Write-Host "  service     Run as Windows service"
    Write-Host "  test        Run tests and examples"
    Write-Host "  profile     Run with performance profiling"
    Write-Host ""
    Write-Host "Options:" -ForegroundColor Cyan
    Write-Host "  -Config <path>    Use custom config file"
    Write-Host "  -Port <number>    Override default port"
    Write-Host "  -Watch            Watch for file changes and restart"
    Write-Host "  -Background       Run in background (detached)"
    Write-Host "  -Kill             Kill any running instances"
    Write-Host "  -Logs             Show service logs"
    Write-Host "  -Clean            Clean build artifacts"
    Write-Host "  -Help             Show this help message"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\debug.ps1 debug                    # Run in debug mode"
    Write-Host "  .\debug.ps1 dev -Watch               # Development with file watching"
    Write-Host "  .\debug.ps1 debug -Background        # Run in background"
    Write-Host "  .\debug.ps1 debug -Port 9090         # Use custom port"
    Write-Host "  .\debug.ps1 -Kill                    # Kill running instances"
    Write-Host "  .\debug.ps1 -Logs                    # Show logs"
    exit 0
}

# Function to kill running instances
function Stop-TTAWinService {
    Write-Host "🛑 Stopping TTAWin service instances..." -ForegroundColor Yellow
    
    # Kill by process name
    $processes = Get-Process -Name "windows-service" -ErrorAction SilentlyContinue
    if ($processes) {
        $processes | Stop-Process -Force
        Write-Host "✅ Stopped $($processes.Count) windows-service processes" -ForegroundColor Green
    }
    
    # Kill by port (common ports)
    $ports = @(8080, 9090, 3000)
    foreach ($port in $ports) {
        $netstat = netstat -ano | Select-String ":$port "
        if ($netstat) {
            $pids = $netstat | ForEach-Object { ($_ -split '\s+')[-1] }
            foreach ($pid in $pids) {
                try {
                    Stop-Process -Id $pid -Force -ErrorAction SilentlyContinue
                    Write-Host "✅ Stopped process $pid using port $port" -ForegroundColor Green
                } catch {
                    # Process might already be stopped
                }
            }
        }
    }
}

# Function to show logs
function Show-TTAWinLogs {
    Write-Host "📋 Recent TTAWin service logs:" -ForegroundColor Yellow
    
    # Try to get Windows Event Log entries
    try {
        $events = Get-EventLog -LogName Application -Source TTAWinService -Newest 10 -ErrorAction SilentlyContinue
        if ($events) {
            $events | ForEach-Object {
                $time = $_.TimeGenerated.ToString("HH:mm:ss")
                $level = if ($_.EntryType -eq "Error") { "❌" } elseif ($_.EntryType -eq "Warning") { "⚠️" } else { "ℹ️" }
                Write-Host "$level [$time] $($_.Message)" -ForegroundColor White
            }
        } else {
            Write-Host "ℹ️ No Windows Event Log entries found for TTAWinService" -ForegroundColor Gray
        }
    } catch {
        Write-Host "ℹ️ Could not access Windows Event Log" -ForegroundColor Gray
    }
    
    # Show recent console output if available
    $logFile = "ttawin-debug.log"
    if (Test-Path $logFile) {
        Write-Host "`n📄 Recent console output:" -ForegroundColor Yellow
        Get-Content $logFile -Tail 20 | ForEach-Object {
            Write-Host "  $_" -ForegroundColor Gray
        }
    }
}

# Function to build the service
function Build-TTAWinService {
    param([string]$BuildType = "debug")
    
    Write-Host "🔨 Building TTAWin service ($BuildType)..." -ForegroundColor Yellow
    
    $cargoArgs = if ($BuildType -eq "release") { "--release" } else { "" }
    $buildResult = cargo build $cargoArgs
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed!"
        exit 1
    }
    
    Write-Host "✅ Build completed successfully!" -ForegroundColor Green
}

# Function to run the service
function Start-TTAWinService {
    param(
        [string]$Mode,
        [string]$Config = "",
        [int]$Port = 0,
        [bool]$Background = $false
    )
    
    # Determine executable path
    $exePath = "target\debug\windows-service.exe"
    if (-not (Test-Path $exePath)) {
        Write-Error "Executable not found. Please build first: cargo build"
        exit 1
    }
    
    # Build command arguments
    $args = @()
    
    # Add mode argument
    switch ($Mode) {
        "debug" { $args += "--debug" }
        "dev" { $args += "--dev" }
        "development" { $args += "--development" }
        "service" { $args += "--service" }
    }
    
    # Add config if specified
    if ($Config -and (Test-Path $Config)) {
        $env:TTAWIN_CONFIG = (Resolve-Path $Config).Path
        Write-Host "📁 Using config: $Config" -ForegroundColor Cyan
    }
    
    # Add port if specified
    if ($Port -gt 0) {
        $env:TTAWIN_PORT = $Port.ToString()
        Write-Host "🔌 Using port: $Port" -ForegroundColor Cyan
    }
    
    # Set environment variables for debugging
    $env:RUST_LOG = "debug"
    $env:RUST_BACKTRACE = "1"
    
    Write-Host "🚀 Starting TTAWin service in $Mode mode..." -ForegroundColor Green
    
    if ($Background) {
        # Run in background
        $job = Start-Job -ScriptBlock {
            param($ExePath, $Args)
            & $ExePath @Args 2>&1 | Tee-Object -FilePath "ttawin-debug.log" -Append
        } -ArgumentList $exePath, $args
        
        Write-Host "✅ Service started in background (Job ID: $($job.Id))" -ForegroundColor Green
        Write-Host "📋 To view output: Receive-Job $($job.Id)" -ForegroundColor Cyan
        Write-Host "🛑 To stop: Stop-Job $($job.Id)" -ForegroundColor Cyan
        
        # Store job ID for later reference
        $job.Id | Out-File -FilePath ".ttawin-job" -Encoding UTF8
    } else {
        # Run in foreground
        & $exePath @args
    }
}

# Function to watch for file changes
function Watch-TTAWinService {
    param([string]$Mode)
    
    Write-Host "👀 Watching for file changes..." -ForegroundColor Yellow
    Write-Host "🔄 Service will restart automatically when files change" -ForegroundColor Cyan
    
    $watchPaths = @("src", "examples", "config")
    $lastRestart = Get-Date
    
    while ($true) {
        try {
            # Check for file changes
            $changed = $false
            foreach ($path in $watchPaths) {
                if (Test-Path $path) {
                    $files = Get-ChildItem -Path $path -Recurse -File | Where-Object { 
                        $_.LastWriteTime -gt $lastRestart 
                    }
                    if ($files) {
                        $changed = $true
                        break
                    }
                }
            }
            
            if ($changed) {
                Write-Host "🔄 File changes detected, restarting service..." -ForegroundColor Yellow
                
                # Kill existing instances
                Stop-TTAWinService
                
                # Wait a moment
                Start-Sleep -Seconds 1
                
                # Rebuild and restart
                Build-TTAWinService
                Start-TTAWinService -Mode $Mode -Background $true
                
                $lastRestart = Get-Date
                Write-Host "✅ Service restarted at $(Get-Date -Format 'HH:mm:ss')" -ForegroundColor Green
            }
            
            Start-Sleep -Seconds 2
        } catch {
            Write-Host "❌ Watch error: $_" -ForegroundColor Red
            Start-Sleep -Seconds 5
        }
    }
}

# Main execution
Write-Host "TTAWin Windows Service Debug Script" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green
Write-Host "Mode: $Mode" -ForegroundColor Cyan
Write-Host ""

# Handle special commands first
if ($Kill) {
    Stop-TTAWinService
    exit 0
}

if ($Logs) {
    Show-TTAWinLogs
    exit 0
}

if ($Clean) {
    Write-Host "🧹 Cleaning build artifacts..." -ForegroundColor Yellow
    cargo clean
    Write-Host "✅ Clean completed" -ForegroundColor Green
    exit 0
}

# Kill any existing instances before starting
Stop-TTAWinService

# Handle different modes
switch ($Mode) {
    "test" {
        Write-Host "🧪 Running tests..." -ForegroundColor Yellow
        cargo test
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Tests passed!" -ForegroundColor Green
        } else {
            Write-Error "Tests failed!"
            exit 1
        }
        
        Write-Host "🎯 Running examples..." -ForegroundColor Yellow
        cargo run --example basic_usage
        Write-Host "✅ Examples completed!" -ForegroundColor Green
    }
    
    "profile" {
        Write-Host "📊 Building with profiling..." -ForegroundColor Yellow
        cargo build --release
        Write-Host "🚀 Starting profiled service..." -ForegroundColor Green
        Start-TTAWinService -Mode "debug" -Background $Background
    }
    
    default {
        # Build the service
        Build-TTAWinService
        
        if ($Watch) {
            # Start in watch mode
            Watch-TTAWinService -Mode $Mode
        } else {
            # Start normally
            Start-TTAWinService -Mode $Mode -Config $Config -Port $Port -Background $Background
        }
    }
}

Write-Host "`n🎉 Debug script completed!" -ForegroundColor Green 