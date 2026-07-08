#requires -Version 7.0
# Pester 4.x test suite for install.ps1.
# Run with: pwsh -Command "Invoke-Pester -Path tests/Install-TxoddsCli.Tests.ps1"

$here = Split-Path -Parent $MyInvocation.MyCommand.Definition
. (Join-Path $here '..\install.ps1')

Describe 'Get-Architecture' {
  It 'returns x86_64 for AMD64' {
    $env:PROCESSOR_ARCHITECTURE = 'AMD64'
    Get-Architecture | Should -Be 'x86_64'
  }
  It 'returns x86_64 for X86_64' {
    $env:PROCESSOR_ARCHITECTURE = 'X86_64'
    Get-Architecture | Should -Be 'x86_64'
  }
  It 'returns aarch64 for ARM64' {
    $env:PROCESSOR_ARCHITECTURE = 'ARM64'
    Get-Architecture | Should -Be 'aarch64'
  }
  It 'returns aarch64 for AARCH64' {
    $env:PROCESSOR_ARCHITECTURE = 'AARCH64'
    Get-Architecture | Should -Be 'aarch64'
  }
  It 'throws on unknown arch' {
    $env:PROCESSOR_ARCHITECTURE = 'MIPS'
    { Get-Architecture } | Should -Throw 'Unsupported architecture'
  }
  It 'throws when PROCESSOR_ARCHITECTURE is empty' {
    $env:PROCESSOR_ARCHITECTURE = ''
    { Get-Architecture } | Should -Throw 'PROCESSOR_ARCHITECTURE is not set'
  }
}

Describe 'Resolve-InstallTarget' {
  It 'maps AMD64 to x86_64-pc-windows-msvc' {
    $env:PROCESSOR_ARCHITECTURE = 'AMD64'
    Resolve-InstallTarget | Should -Be 'x86_64-pc-windows-msvc'
  }
  It 'maps ARM64 to aarch64-pc-windows-msvc' {
    $env:PROCESSOR_ARCHITECTURE = 'ARM64'
    Resolve-InstallTarget | Should -Be 'aarch64-pc-windows-msvc'
  }
  It 'throws when no target matches arch' {
    $env:PROCESSOR_ARCHITECTURE = 'AMD64'
    { Resolve-InstallTarget -SupportedTargets @('aarch64-pc-windows-msvc') } | Should -Throw 'No prebuilt target'
  }
}

Describe 'Get-ArchiveName' {
  It 'builds txodds-<target>.zip' {
    Get-ArchiveName -Target 'x86_64-pc-windows-msvc' | Should -Be 'txodds-x86_64-pc-windows-msvc.zip'
  }
  It 'works for aarch64 target' {
    Get-ArchiveName -Target 'aarch64-pc-windows-msvc' | Should -Be 'txodds-aarch64-pc-windows-msvc.zip'
  }
}

Describe 'Get-ArchiveUrl' {
  It 'builds the download URL with v-prefixed tag' {
    $url = Get-ArchiveUrl -Repo 'owner/repo' -Tag '1.2.3' -Target 'x86_64-pc-windows-msvc'
    $url | Should -Be 'https://github.com/owner/repo/releases/download/v1.2.3/txodds-x86_64-pc-windows-msvc.zip'
  }
}

Describe 'Resolve-ReleaseTag' {
  It 'strips v prefix from API response' {
    Mock Invoke-GitHubApi {
      param($Uri)
      [PSCustomObject]@{ tag_name = 'v9.9.9' }
    }
    (Resolve-ReleaseTag -Repo 'foo/bar' -Override $null) | Should -Be '9.9.9'
  }
  It 'queries GitHub API when no override' {
    Mock Invoke-GitHubApi {
      param($Uri)
      [PSCustomObject]@{ tag_name = 'v1.0.0' }
    }
    Resolve-ReleaseTag -Repo 'foo/bar' -Override $null | Out-Null
    Assert-MockCalled Invoke-GitHubApi -Times 1 -Exactly -Scope It
  }
  It 'uses override and skips API call' {
    Mock Invoke-GitHubApi { throw 'should not be called' }
    (Resolve-ReleaseTag -Repo 'foo/bar' -Override '1.2.3') | Should -Be '1.2.3'
    Assert-MockCalled Invoke-GitHubApi -Times 0 -Exactly -Scope It
  }
  It 'strips v prefix from override' {
    (Resolve-ReleaseTag -Repo 'foo/bar' -Override 'v1.2.3') | Should -Be '1.2.3'
  }
  It 'throws with HTTP code on 404' {
    Mock Invoke-GitHubApi {
      param($Uri)
      throw [System.Net.WebException]::new('GitHub API request failed (HTTP 404) at https://api.github.com/repos/foo/bar/releases/latest')
    }
    { Resolve-ReleaseTag -Repo 'foo/bar' -Override $null } | Should -Throw 'GitHub API request failed'
  }
  It 'throws when tag_name missing' {
    Mock Invoke-GitHubApi {
      param($Uri)
      [PSCustomObject]@{}
    }
    { Resolve-ReleaseTag -Repo 'foo/bar' -Override $null } | Should -Throw 'did not contain tag_name'
  }
}

Describe 'Invoke-Download' {
  It 'downloads file successfully' {
    Mock Invoke-DownloadInternal {
      param($Uri, $OutFile)
      'PK' + [char]5 + [char]6 + 'fakezipcontent' | Set-Content -Path $OutFile -NoNewline
    }
    $out = Join-Path ([System.IO.Path]::GetTempPath()) "txodds-test-$([guid]::NewGuid()).bin"
    try {
      Invoke-Download -Url 'http://example.com/file.zip' -OutFile $out
      Test-Path $out | Should -Be $true
      (Get-Item $out).Length | Should -BeGreaterThan 0
    } finally {
      Remove-Item -Force -ErrorAction SilentlyContinue $out
    }
  }
  It 'throws with HTTP code on 404' {
    Mock Invoke-DownloadInternal {
      param($Uri, $OutFile)
      throw [System.Net.WebException]::new('Download failed (HTTP 404) from http://example.com/missing.zip')
    }
    $out = Join-Path ([System.IO.Path]::GetTempPath()) "txodds-test-$([guid]::NewGuid()).bin"
    try {
      { Invoke-Download -Url 'http://example.com/missing.zip' -OutFile $out } | Should -Throw 'Download failed'
    } finally {
      Remove-Item -Force -ErrorAction SilentlyContinue $out
    }
  }
}
