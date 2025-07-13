# TTAWin Windows Service Build Script

param(
    [string]$BuildType = "release",
    [switch]$Test,
    [switch]$Install,
    [switch]$Clean,
    [switch]$Debug,
    [switch]$Dev
)

Write-Host "TTAWin Windows Service Build Script" -ForegroundColor Green
Write-Host "===================================" -ForegroundColor Green

# Set build type
$cargoArgs = if ($BuildType -eq "release") { "--release" } else { "" }

# Clean if requested
if ($Clean) {
    Write-Host "🧹 Cleaning build artifacts..." -ForegroundColor Yellow
    cargo clean
    Write-Host "✅ Clean completed" -ForegroundColor Green
    Write-Host ""
}

# Build the service
Write-Host "🔨 Building Windows service ($BuildType)..." -ForegroundColor Yellow
$buildResult = cargo build $cargoArgs

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed!"
    exit 1
}

Write-Host "✅ Build completed successfully!" -ForegroundColor Green

# Determine executable path
$exePath = if ($BuildType -eq "release") {
    "target\release\windows-service.exe"
} else {
    "target\debug\windows-service.exe"
}

if (Test-Path $exePath) {
    $fileInfo = Get-Item $exePath
    Write-Host "📦 Executable: $exePath" -ForegroundColor Cyan
    Write-Host "📏 Size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Cyan
    Write-Host "📅 Created: $($fileInfo.CreationTime)" -ForegroundColor Cyan
} else {
    Write-Error "Executable not found at: $exePath"
    exit 1
}

Write-Host ""

# Run tests if requested
if ($Test) {
    Write-Host "🧪 Running tests..." -ForegroundColor Yellow
    $testResult = cargo test $cargoArgs
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Tests failed!"
        exit 1
    }
    
    Write-Host "✅ Tests passed!" -ForegroundColor Green
    Write-Host ""
}

# Run example if requested
if ($Test) {
    Write-Host "🎯 Running example..." -ForegroundColor Yellow
    $exampleResult = cargo run --example basic_usage $cargoArgs
    
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "Example failed (this might be expected in some environments)"
    } else {
        Write-Host "✅ Example completed!" -ForegroundColor Green
    }
    Write-Host ""
}

# Install service if requested
if ($Install) {
    Write-Host "📦 Installing Windows service..." -ForegroundColor Yellow
    
    # Check if running as administrator
    if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
        Write-Error "Installation requires Administrator privileges"
        Write-Host "Please run this script as Administrator" -ForegroundColor Yellow
        exit 1
    }
    
    # Run the installation script
    $installScript = Join-Path $PSScriptRoot "install-service.ps1"
    if (Test-Path $installScript) {
        & $installScript -ExePath (Resolve-Path $exePath).Path
    } else {
        Write-Error "Installation script not found: $installScript"
        exit 1
    }
}

Write-Host "🎉 Build process completed!" -ForegroundColor Green

# Run debug mode if requested
if ($Debug) {
    Write-Host "`n🚀 Starting debug mode..." -ForegroundColor Yellow
    & $exePath --debug
}

if ($Dev) {
    Write-Host "`n🚀 Starting development mode..." -ForegroundColor Yellow
    & $exePath --dev
}

# Display next steps
Write-Host "`nNext Steps:" -ForegroundColor Yellow
Write-Host "  • Run the service: $exePath" -ForegroundColor White
Write-Host "  • Debug mode: $exePath --debug" -ForegroundColor White
Write-Host "  • Development mode: $exePath --dev" -ForegroundColor White
Write-Host "  • Quick debug: .\debug.bat debug" -ForegroundColor White
Write-Host "  • Advanced debug: .\debug.ps1 debug" -ForegroundColor White
Write-Host "  • Test endpoints: .\test-service.ps1" -ForegroundColor White
Write-Host "  • Install as Windows service: .\install-service.ps1" -ForegroundColor White
Write-Host "  • Test API: curl http://localhost:8080/health" -ForegroundColor White
Write-Host "  • View logs: Get-EventLog -LogName Application -Source TTAWinService" -ForegroundColor White 