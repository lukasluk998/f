# PowerShell script to build and auto-rename executable
# Usage: .\rename_build.ps1

Write-Host "Building Rust cheat with randomized name..." -ForegroundColor Green

# Build and capture output
$output = cargo build --release 2>&1 | Out-String

# Extract random name from build warnings
$randomName = ($output | Select-String "Randomized process name: (.+)").Matches.Groups[1].Value

if ($randomName) {
    Write-Host "Generated name: $randomName" -ForegroundColor Cyan
    
    $sourcePath = "target\release\rust-game-cheat.exe"
    $destPath = "target\release\$randomName"
    
    if (Test-Path $sourcePath) {
        Move-Item -Path $sourcePath -Destination $destPath -Force
        Write-Host "✓ Renamed to: $destPath" -ForegroundColor Green
        Write-Host ""
        Write-Host "Run with: .\$destPath" -ForegroundColor Yellow
    } else {
        Write-Host "✗ Build failed or executable not found" -ForegroundColor Red
    }
} else {
    Write-Host "✗ Could not extract random name from build output" -ForegroundColor Red
}
