<#
.SYNOPSIS
  Install the txodds CLI on Windows.
.DESCRIPTION
  Downloads the latest release binary for the host architecture
  (x86_64-pc-windows-msvc or aarch64-pc-windows-msvc) and installs it to
  $env:USERPROFILE\.txodds\bin\txodds.exe.
.EXAMPLE
  iwr -useb https://github.com/swingkiddo/txline-cli/releases/latest/download/install.ps1 | iex
#>

$ErrorActionPreference = 'Stop'

$Repo       = if ($env:TXODDS_INSTALL_REPO) { $env:TXODDS_INSTALL_REPO } else { 'swingkiddo/txline-cli' }
$UserHome   = if ($env:USERPROFILE) { $env:USERPROFILE } elseif ($env:HOME) { $env:HOME } else { [System.IO.Path]::GetTempPath() }
$InstallDir = if ($env:TXODDS_INSTALL_DIR) { $env:TXODDS_INSTALL_DIR } else { Join-Path $UserHome '.txodds\bin' }
$BinaryName = 'txodds.exe'
$TempRoot   = if ($env:TEMP) { $env:TEMP } else { [System.IO.Path]::GetTempPath() }

$SupportedTargets = @(
  'x86_64-pc-windows-msvc',
  'aarch64-pc-windows-msvc'
)

function Write-Info { param([Parameter(Mandatory)][string]$Message) Write-Host "==> $Message" -ForegroundColor Cyan }
function Write-Err  { param([Parameter(Mandatory)][string]$Message) Write-Host "error: $Message" -ForegroundColor Red }

function Get-Architecture {
  $arch = $env:PROCESSOR_ARCHITECTURE
  if (-not $arch) { throw 'PROCESSOR_ARCHITECTURE is not set' }
  switch -Regex ($arch) {
    '^(ARM64|AARCH64)$'              { return 'aarch64' }
    '^(AMD64|X86_64|X64|IA64)$'      { return 'x86_64' }
    default                           { throw "Unsupported architecture: $arch" }
  }
}

function Resolve-InstallTarget {
  param(
    [string[]]$SupportedTargets = $script:SupportedTargets
  )
  $arch = Get-Architecture
  $match = $SupportedTargets | Where-Object { $_ -like "$arch-*" } | Select-Object -First 1
  if (-not $match) {
    throw "No prebuilt target for architecture '$arch' (supported: $($SupportedTargets -join ', '))"
  }
  return $match
}

function Get-ArchiveName {
  param([Parameter(Mandatory)][string]$Target)
  "txodds-$Target.zip"
}

function Get-ArchiveUrl {
  param(
    [Parameter(Mandatory)][string]$Repo,
    [Parameter(Mandatory)][string]$Tag,
    [Parameter(Mandatory)][string]$Target
  )
  $archive = Get-ArchiveName -Target $Target
  "https://github.com/$Repo/releases/download/v$Tag/$archive"
}

function Invoke-GitHubApi {
  param(
    [Parameter(Mandatory)][string]$Uri
  )
  try {
    Invoke-RestMethod -Uri $Uri -ErrorAction Stop
  } catch {
    if ($_.Exception.Response) {
      $code = [int]$_.Exception.Response.StatusCode
      throw "GitHub API request failed (HTTP $code) at $Uri"
    }
    throw
  }
}

function Invoke-DownloadInternal {
  param(
    [Parameter(Mandatory)][string]$Uri,
    [Parameter(Mandatory)][string]$OutFile
  )
  try {
    Invoke-WebRequest -Uri $Uri -OutFile $OutFile -UseBasicParsing -ErrorAction Stop
  } catch {
    if ($_.Exception.Response) {
      $code = [int]$_.Exception.Response.StatusCode
      throw "Download failed (HTTP $code) from $Uri"
    }
    throw
  }
}

function Resolve-ReleaseTag {
  param(
    [Parameter(Mandatory)][string]$Repo,
    [string]$Override
  )
  if ($Override) { return ($Override -replace '^v','') }

  $apiUrl = "https://api.github.com/repos/$Repo/releases/latest"
  try {
    $release = Invoke-GitHubApi -Uri $apiUrl
  } catch {
    if ($_.Exception.Message -like '*HTTP*') { throw }
    throw "GitHub API request failed at $apiUrl"
  }
  if (-not $release.tag_name) { throw "GitHub API response at $apiUrl did not contain tag_name" }
  return ($release.tag_name -replace '^v','')
}

function Invoke-Download {
  param(
    [Parameter(Mandatory)][string]$Url,
    [Parameter(Mandatory)][string]$OutFile
  )
  try {
    Invoke-DownloadInternal -Uri $Url -OutFile $OutFile
  } catch {
    if ($_.Exception.Message -like '*HTTP*') { throw }
    throw "Download failed from $Url"
  }
}

function Install-TxoddsCli {
  [CmdletBinding()]
  param(
    [string]$Repo = $script:Repo,
    [string]$InstallDir = $script:InstallDir,
    [string]$BinaryName = $script:BinaryName,
    [string]$TempRoot = $script:TempRoot,
    [string[]]$SupportedTargets = $script:SupportedTargets
  )

  $target = Resolve-InstallTarget -SupportedTargets $SupportedTargets
  Write-Info "Installing txodds CLI (Windows / $target)"

  if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
  }

  $tag = Resolve-ReleaseTag -Repo $Repo -Override $env:TXODDS_VERSION
  Write-Info "Installing version: v$tag"

  $archiveName = Get-ArchiveName -Target $target
  $archiveUrl  = Get-ArchiveUrl -Repo $Repo -Tag $tag -Target $target
  Write-Info "Downloading $archiveUrl"

  $tmp = Join-Path $TempRoot 'txodds-install'
  if (-not (Test-Path $tmp)) { New-Item -ItemType Directory -Force -Path $tmp | Out-Null }
  $archivePath = Join-Path $tmp $archiveName
  Invoke-Download -Url $archiveUrl -OutFile $archivePath

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

  Write-Info 'Verifying install...'
  & $dest --version
}

if ($MyInvocation.InvocationName -ne '.') {
  try {
    Install-TxoddsCli
  } catch {
    Write-Err $_.Exception.Message
    exit 1
  }
}
