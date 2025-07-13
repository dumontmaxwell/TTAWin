# TTAWin Service Manager - Root Level Script
# Provides comprehensive service management from the project root

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("debug", "dev", "development", "test", "kill", "build", "clean", "logs", "health", "install")]
    [string]$Action = "debug",
    
    [Parameter(Mandatory=$false)]
    [string]$Config = "",
    
    [Parameter(Mandatory=$false)]
    [int]$Port = 0,
    
    [switch]$Watch,
    [switch]$Background,
    [switch]$Help
)

# Show help if requested
if ($Help) {
    Write-Host "TTAWin Service Manager - Root Level Script" -ForegroundColor Green
    Write-Host "===========================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Usage: .\scripts\service-manager.ps1 [Action] [Options]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Actions:" -ForegroundColor Cyan
    Write-Host "  debug       Start service in debug mode"
    Write-Host "  dev         Start service in development mode"
    Write-Host "  development Same as dev"
    Write-Host "  test        Test all service endpoints"
    Write-Host "  kill        Kill all service instances"
    Write-Host "  build       Build the service"
    Write-Host "  clean       Clean build artifacts"
    Write-Host "  logs        Show service logs"
    Write-Host "  health      Check service health"
    Write-Host "  install     Install as Windows service"
    Write-Host ""
    Write-Host "Options:" -ForegroundColor Cyan
    Write-Host "  -Config <path>    Use custom config file"
    Write-Host "  -Port <number>    Override default port"
    Write-Host "  -Watch            Watch for file changes (dev mode)"
    Write-Host "  -Background       Run in background"
    Write-Host "  -Help             Show this help message"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\scripts\service-manager.ps1 debug                    # Debug mode"
    Write-Host "  .\scripts\service-manager.ps1 dev -Watch               # Development with watching"
    Write-Host "  .\scripts\service-manager.ps1 test                     # Test endpoints"
    Write-Host "  .\scripts\service-manager.ps1 kill                     # Kill instances"
    Write-Host "  .\scripts\service-manager.ps1 health                   # Check health"
    exit 0
}

# Function to check if we're in the right directory
function Test-TTAWinRoot {
    if (-not (Test-Path "packages\windows-service")) {
        Write-Error "TTAWin root directory not found. Please run this script from the TTAWin root directory."
        exit 1
    }
}

# Function to run service command
function Invoke-ServiceCommand {
    param(
        [string]$Command,
        [string[]]$Arguments = @()
    )
    
    $serviceDir = "packages\windows-service"
    $originalDir = Get-Location
    
    try {
        Set-Location $serviceDir
        
        if ($Command -eq "debug.ps1") {
            & ".\debug.ps1" @Arguments
        } elseif ($Command -eq "debug.bat") {
            & ".\debug.bat" @Arguments
        } elseif ($Command -eq "test-service.ps1") {
            & ".\test-service.ps1" @Arguments
        } elseif ($Command -eq "build.ps1") {
            & ".\build.ps1" @Arguments
        } else {
            & $Command @Arguments
        }
    } finally {
        Set-Location $originalDir
    }
}

# Function to check service health
function Test-ServiceHealth {
    try {
        $response = Invoke-RestMethod -Uri "http://localhost:8080/health" -Method GET -TimeoutSec 5
        Write-Host "✅ Service is healthy" -ForegroundColor Green
        Write-Host "Response: $($response | ConvertTo-Json)" -ForegroundColor Gray
        return $true
    } catch {
        Write-Host "❌ Service is not responding" -ForegroundColor Red
        Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

# Main execution
Write-Host "TTAWin Service Manager" -ForegroundColor Green
Write-Host "=====================" -ForegroundColor Green
Write-Host "Action: $Action" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the right directory
Test-TTAWinRoot

# Handle different actions
switch ($Action) {
    "debug" {
        Write-Host "🚀 Starting TTAWin service in debug mode..." -ForegroundColor Yellow
        
        $args = @("debug")
        if ($Config) { $args += "-Config", $Config }
        if ($Port -gt 0) { $args += "-Port", $Port.ToString() }
        if ($Background) { $args += "-Background" }
        
        Invoke-ServiceCommand "debug.ps1" $args
    }
    
    "dev" {
        Write-Host "🚀 Starting TTAWin service in development mode..." -ForegroundColor Yellow
        
        $args = @("dev")
        if ($Watch) { $args += "-Watch" }
        if ($Config) { $args += "-Config", $Config }
        if ($Port -gt 0) { $args += "-Port", $Port.ToString() }
        if ($Background) { $args += "-Background" }
        
        Invoke-ServiceCommand "debug.ps1" $args
    }
    
    "development" {
        # Same as dev
        Write-Host "🚀 Starting TTAWin service in development mode..." -ForegroundColor Yellow
        
        $args = @("dev")
        if ($Watch) { $args += "-Watch" }
        if ($Config) { $args += "-Config", $Config }
        if ($Port -gt 0) { $args += "-Port", $Port.ToString() }
        if ($Background) { $args += "-Background" }
        
        Invoke-ServiceCommand "debug.ps1" $args
    }
    
    "test" {
        Write-Host "🧪 Testing TTAWin service endpoints..." -ForegroundColor Yellow
        Invoke-ServiceCommand "test-service.ps1"
    }
    
    "kill" {
        Write-Host "🛑 Killing all TTAWin service instances..." -ForegroundColor Yellow
        Invoke-ServiceCommand "debug.bat" @("kill")
        
        # Also try to kill any remaining processes
        $processes = Get-Process -Name "windows-service" -ErrorAction SilentlyContinue
        if ($processes) {
            $processes | Stop-Process -Force
            Write-Host "✅ Stopped $($processes.Count) remaining windows-service processes" -ForegroundColor Green
        }
    }
    
    "build" {
        Write-Host "🔨 Building TTAWin service..." -ForegroundColor Yellow
        Invoke-ServiceCommand "build.ps1"
    }
    
    "clean" {
        Write-Host "🧹 Cleaning TTAWin service build artifacts..." -ForegroundColor Yellow
        Set-Location "packages\windows-service"
        cargo clean
        Set-Location $PSScriptRoot
        Write-Host "✅ Clean completed" -ForegroundColor Green
    }
    
    "logs" {
        Write-Host "📋 Showing TTAWin service logs..." -ForegroundColor Yellow
        Invoke-ServiceCommand "debug.ps1" @("-Logs")
    }
    
    "health" {
        Write-Host "🏥 Checking TTAWin service health..." -ForegroundColor Yellow
        Test-ServiceHealth
    }
    
    "install" {
        Write-Host "📦 Installing TTAWin as Windows service..." -ForegroundColor Yellow
        
        # Check if running as administrator
        if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
            Write-Error "Installation requires Administrator privileges"
            Write-Host "Please run this script as Administrator" -ForegroundColor Yellow
            exit 1
        }
        
        Invoke-ServiceCommand "build.ps1" @("-Install")
    }
    
    default {
        Write-Error "Unknown action: $Action"
        Write-Host "Use -Help to see available actions" -ForegroundColor Yellow
        exit 1
    }
}

Write-Host "`n🎉 Service manager operation completed!" -ForegroundColor Green 