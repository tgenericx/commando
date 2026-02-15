# Commando installation script for Windows
# Usage: irm https://raw.githubusercontent.com/tgenericx/commando/main/install.ps1 | iex

$ErrorActionPreference = 'Stop'

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

Write-ColorOutput Green "=== Commando Installer for Windows ==="
Write-Output ""

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
if ($arch -eq "x86") {
    Write-ColorOutput Red "Error: 32-bit Windows is not supported"
    exit 1
}

Write-ColorOutput Green "Detected architecture: $arch"

# Get latest version
Write-ColorOutput Yellow "Fetching latest version..."
try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/tgenericx/commando/releases/latest"
    $version = $release.tag_name -replace '^v', ''
    Write-ColorOutput Green "Latest version: v$version"
} catch {
    Write-ColorOutput Red "Error: Could not fetch latest version"
    exit 1
}

# Download binary
$binaryName = "commando-windows-$arch.exe"
$downloadUrl = "https://github.com/tgenericx/commando/releases/download/v$version/$binaryName"

Write-ColorOutput Yellow "Downloading from: $downloadUrl"

$tempDir = [System.IO.Path]::GetTempPath()
$tempFile = Join-Path $tempDir $binaryName

try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile
    Write-ColorOutput Green "Download complete"
} catch {
    Write-ColorOutput Red "Error: Failed to download binary"
    exit 1
}

# Install binary
$installDir = "$env:LOCALAPPDATA\Programs\commando"
$installPath = Join-Path $installDir "commando.exe"

# Create installation directory
if (!(Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

# Move binary
Move-Item -Path $tempFile -Destination $installPath -Force
Write-ColorOutput Green "Installed to: $installPath"

# Add to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    Write-ColorOutput Yellow "Adding to PATH..."
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$userPath;$installDir",
        "User"
    )
    Write-ColorOutput Green "Added $installDir to PATH"
    Write-ColorOutput Yellow "You may need to restart your terminal for PATH changes to take effect"
} else {
    Write-ColorOutput Green "Installation directory already in PATH"
}

# Verify installation
Write-Output ""
Write-ColorOutput Green "=== Verification ==="

# Refresh environment variables for current session
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")

try {
    $versionOutput = & $installPath --version 2>&1
    Write-ColorOutput Green "✓ Installation successful!"
    Write-Output "Version: $versionOutput"
} catch {
    Write-ColorOutput Yellow "Note: You may need to restart your terminal to use 'commando'"
    Write-Output "Binary location: $installPath"
}

Write-Output ""
Write-ColorOutput Green "Installation complete!"
Write-Output ""
Write-Output "Try it out:"
Write-Output "  cd C:\path\to\your\git\repo"
Write-Output "  git add <files>"
Write-Output "  commando"
Write-Output ""
Write-ColorOutput Yellow "Note: If 'commando' is not found, restart your terminal"
