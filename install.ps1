<#
.SYNOPSIS
  Install the txodds CLI on Windows.
.DESCRIPTION
  Downloads the latest release binary for Windows (x86_64-pc-windows-msvc)
  and installs it to $env:USERPROFILE\.txodds\bin\txodds.exe.
.EXAMPLE
  iwr -useb https://github.com/swingkiddo/txline-cli/releases/latest/download/install.ps1 | iex
#>

$ErrorActionPreference = 'Stop'

$Repo      = if ($env:TXODDS_INSTALL_REPO) { $env:TXODDS_INSTALL_REPO } else { 'swingkiddo/txline-cli' }
$InstallDir = if ($env:TXODDS_INSTALL_DIR) { $env:TXODDS_INSTALL_DIR } else { Join-Path $env:USERPROFILE '.txodds\bin' }
$BinaryName = 'txodds.exe'
$Target = 'x86_64-pc-windows-msvc'

function Write-Info($msg) { Write-Host "==> $msg" -ForegroundColor Cyan }
function Write-Err($msg)  { Write-Host "error: $msg" -ForegroundColor Red }

try {
  Write-Info "Installing txodds CLI (Windows / x86_64)"

  if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
  }

  $apiUrl = "https://api.github.com/repos/$Repo/releases/latest"
  $release = Invoke-RestMethod -Uri $apiUrl
  $tag = $release.tag_name -replace '^v',''
  if (-not $tag) { throw "Could not determine latest release tag." }
  Write-Info "Latest release: v$tag"

  $archiveName = "txodds-$Target.zip"
  $archiveUrl  = "https://github.com/$Repo/releases/download/v$tag/$archiveName"
  Write-Info "Downloading $archiveUrl"

  $tmp = Join-Path $env:TEMP "txodds-install"
  if (-not (Test-Path $tmp)) { New-Item -ItemType Directory -Force -Path $tmp | Out-Null }
  $archivePath = Join-Path $tmp $archiveName
  Invoke-WebRequest -Uri $archiveUrl -OutFile $archivePath -UseBasicParsing

  $extractDir = Join-Path $tmp 'extracted'
  if (Test-Path $extractDir) { Remove-Item -Recurse -Force $extractDir }
  Expand-Archive -Path $archivePath -DestinationPath $extractDir

  $src = Join-Path $extractDir $BinaryName
  if (-not (Test-Path $src)) { throw "Binary not found after extraction: $src" }

  $dest = Join-Path $InstallDir $BinaryName
  Copy-Item -Path $src -Destination $dest -Force

  Write-Info "Installed to $dest"

  $pathEnv = [Environment]::GetEnvironmentVariable('Path', 'User')
  if ($pathEnv -notlike "*$InstallDir*") {
    Write-Info "Adding $InstallDir to user PATH"
    [Environment]::SetEnvironmentVariable('Path', "$pathEnv;$InstallDir", 'User')
    $env:Path = "$env:Path;$InstallDir"
  }

  Write-Info "Verifying install..."
  & $dest --version
}
catch {
  Write-Err $_.Exception.Message
  exit 1
}
