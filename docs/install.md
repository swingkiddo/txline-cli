# Installing the `txodds` CLI

## Quick install

### macOS / Linux

```bash
curl -sfL https://github.com/swingkiddo/txline-cli/releases/latest/download/install.sh | sh
```

### Windows (PowerShell)

```powershell
iwr -useb https://github.com/swingkiddo/txline-cli/releases/latest/download/install.ps1 | iex
```

## What it does

- Detects your OS and CPU architecture
- Fetches the latest release tag from GitHub
- Downloads the matching archive (`txodds-<target>.tar.xz` or `.zip`)
- Installs the binary to `~/.txodds/bin/txodds` (or `txodds.exe` on Windows)
- Adds the install directory to your `PATH` (Windows: user PATH; macOS/Linux: prints hint)
- Runs `txodds --version` to confirm

## Customization

| Env var                | Default                  | Purpose                          |
|------------------------|--------------------------|----------------------------------|
| `TXODDS_INSTALL_REPO`  | `swingkiddo/txline-cli`  | GitHub repo to install from      |
| `TXODDS_INSTALL_DIR`   | `~/.txodds/bin`          | Where to drop the binary         |

## Verify

```bash
txodds --version
txodds auth guest           # prints guest JWT and saves it
```

## Manual install

If you prefer to download the binary yourself:

1. Go to <https://github.com/swingkiddo/txline-cli/releases/latest>
2. Download the archive for your target
3. Extract and place the binary anywhere on your `PATH`

## Build from source

```bash
git clone https://github.com/swingkiddo/txline-cli
cd txline-cli
cargo build --release
./target/release/txodds --version
```

## Uninstall

```bash
rm ~/.txodds/bin/txodds       # macOS / Linux
rm "$env:USERPROFILE\.txodds\bin\txodds.exe"   # Windows
```
