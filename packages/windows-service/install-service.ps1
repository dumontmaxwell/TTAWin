# TTAWin Windows Service Installation Script
# Run this script as Administrator

param(
    [string]$ServiceName = "TTAWinService",
    [string]$DisplayName = "TTAWin Backend Service",
    [string]$Description = "TTAWin Backend Service - Provides learning, payment, and streaming functionality",
    [string]$ExePath = "",
    [string]$StartType = "auto"
)

# Check if running as administrator
if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Error "This script must be run as Administrator"
    exit 1
}

Write-Host "TTAWin Windows Service Installation" -ForegroundColor Green
Write-Host "===================================" -ForegroundColor Green

# Determine executable path if not provided
if ([string]::IsNullOrEmpty($ExePath)) {
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $ExePath = Join-Path $scriptDir "target\release\windows-service.exe"
    
    # Check if release build exists, otherwise try debug
    if (-not (Test-Path $ExePath)) {
        $ExePath = Join-Path $scriptDir "target\debug\windows-service.exe"
    }
}

# Verify executable exists
if (-not (Test-Path $ExePath)) {
    Write-Error "Executable not found at: $ExePath"
    Write-Host "Please build the service first with: cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host "Using executable: $ExePath" -ForegroundColor Cyan

# Stop and remove existing service if it exists
if (Get-Service -Name $ServiceName -ErrorAction SilentlyContinue) {
    Write-Host "Stopping existing service..." -ForegroundColor Yellow
    try {
        Stop-Service -Name $ServiceName -Force -ErrorAction Stop
        Write-Host "Service stopped successfully" -ForegroundColor Green
    }
    catch {
        Write-Warning "Failed to stop service: $($_.Exception.Message)"
    }
    
    Write-Host "Removing existing service..." -ForegroundColor Yellow
    try {
        sc.exe delete $ServiceName
        Write-Host "Service removed successfully" -ForegroundColor Green
    }
    catch {
        Write-Warning "Failed to remove service: $($_.Exception.Message)"
    }
}

# Create the service
Write-Host "Creating service..." -ForegroundColor Yellow
try {
    $binPath = "`"$ExePath`""
    sc.exe create $ServiceName binPath= $binPath start= $StartType DisplayName= $DisplayName
    Write-Host "Service created successfully" -ForegroundColor Green
}
catch {
    Write-Error "Failed to create service: $($_.Exception.Message)"
    exit 1
}

# Set service description
Write-Host "Setting service description..." -ForegroundColor Yellow
try {
    sc.exe description $ServiceName $Description
    Write-Host "Service description set successfully" -ForegroundColor Green
}
catch {
    Write-Warning "Failed to set service description: $($_.Exception.Message)"
}

# Create data directories
Write-Host "Creating data directories..." -ForegroundColor Yellow
$exeDir = Split-Path -Parent $ExePath
$dataDir = Join-Path $exeDir "data"
$configDir = Join-Path $exeDir "config"
$modelsDir = Join-Path $dataDir "models"
$cacheDir = Join-Path $dataDir "cache"

$directories = @($dataDir, $configDir, $modelsDir, $cacheDir)

foreach ($dir in $directories) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
        Write-Host "Created directory: $dir" -ForegroundColor Green
    }
}

# Copy configuration file if it doesn't exist
$configFile = Join-Path $configDir "service.toml"
if (-not (Test-Path $configFile)) {
    $sourceConfig = Join-Path $scriptDir "config\service.toml"
    if (Test-Path $sourceConfig) {
        Copy-Item $sourceConfig $configFile
        Write-Host "Configuration file copied to: $configFile" -ForegroundColor Green
    }
    else {
        Write-Warning "Source configuration file not found. Please create $configFile manually."
    }
}

# Start the service
Write-Host "Starting service..." -ForegroundColor Yellow
try {
    Start-Service -Name $ServiceName -ErrorAction Stop
    Write-Host "Service started successfully" -ForegroundColor Green
}
catch {
    Write-Error "Failed to start service: $($_.Exception.Message)"
    Write-Host "You can try starting it manually with: Start-Service -Name $ServiceName" -ForegroundColor Yellow
    exit 1
}

# Verify service is running
Start-Sleep -Seconds 2
$service = Get-Service -Name $ServiceName
if ($service.Status -eq "Running") {
    Write-Host "Service is running successfully!" -ForegroundColor Green
    Write-Host "Service Name: $ServiceName" -ForegroundColor Cyan
    Write-Host "Display Name: $DisplayName" -ForegroundColor Cyan
    Write-Host "Status: $($service.Status)" -ForegroundColor Cyan
    Write-Host "Start Type: $($service.StartType)" -ForegroundColor Cyan
}
else {
    Write-Warning "Service may not be running properly. Status: $($service.Status)"
}

# Display useful commands
Write-Host "`nUseful Commands:" -ForegroundColor Yellow
Write-Host "  Check service status: sc query $ServiceName" -ForegroundColor White
Write-Host "  Stop service: sc stop $ServiceName" -ForegroundColor White
Write-Host "  Start service: sc start $ServiceName" -ForegroundColor White
Write-Host "  Remove service: sc delete $ServiceName" -ForegroundColor White
Write-Host "  View logs: Get-EventLog -LogName Application -Source $ServiceName" -ForegroundColor White

# Test API endpoint
Write-Host "`nTesting API endpoint..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -TimeoutSec 5 -ErrorAction Stop
    if ($response.StatusCode -eq 200) {
        Write-Host "API endpoint is responding successfully!" -ForegroundColor Green
        Write-Host "Health check response: $($response.Content)" -ForegroundColor Cyan
    }
}
catch {
    Write-Warning "API endpoint test failed: $($_.Exception.Message)"
    Write-Host "The service may still be starting up. Try again in a few seconds." -ForegroundColor Yellow
}

Write-Host "`nInstallation completed!" -ForegroundColor Green
Write-Host "The TTAWin Windows Service is now installed and running." -ForegroundColor Green
Write-Host "You can access the API at: http://localhost:8080" -ForegroundColor Cyan 