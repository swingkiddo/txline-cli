# Configuration

The CLI reads configuration from two sources, in priority order:

1. **Environment variables** (highest priority)
2. **`~/.txodds/credentials.toml`** (primary config file)

## credentials.toml

**Location:** `~/.txodds/credentials.toml`

This file is auto-managed by the CLI. It is created after running `txodds auth guest` followed by `txodds auth activate`. You can also edit it manually.

### File structure

```toml
[default]
network = "devnet"
api_host = ""
jwt = "eyJ..."
api_token = "..."
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | string | yes | `"devnet"` or `"mainnet"`. Defaults to `"devnet"` if missing or invalid. |
| `api_host` | string | no | Override API host. If empty, derived automatically from `network`. |
| `jwt` | string | yes | JWT from guest authentication. Auto-saved by `auth activate`. |
| `api_token` | string | no | API token from on-chain subscription. Auto-saved after subscription. |

## Environment variables

All configuration fields can be overridden via environment variables. Env vars take priority over `credentials.toml`.

| Variable | Overrides | Example |
|----------|-----------|---------|
| `TXODDS_NETWORK` | `network` | `devnet`, `mainnet` |
| `TXODDS_API_HOST` | `api_host` | `https://custom-host.example.com` |
| `TXODDS_JWT` | `jwt` | `eyJhbGciOi...` |
| `TXODDS_API_TOKEN` | `api_token` | `abc123...` |

The CLI also loads a `.env` file from `~/.txodds/.env` if present.

## Network configuration

| Network | API Host |
|---------|----------|
| `mainnet` | `https://txline.txodds.com` |
| `devnet` | `https://txline-dev.txodds.com` |

When `api_host` is empty in `credentials.toml`, the CLI derives it from the `network` field using the table above.

## Examples

### Manual credentials.toml edit

```toml
[default]
network = "mainnet"
api_host = ""
jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
api_token = "dG9rZW4tZnJvbS1vbi1jaGFpbi1zdWJzY3JpcHRpb24="
```

### Using environment variables for CI/testing

```bash
export TXODDS_NETWORK=devnet
export TXODDS_JWT="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
export TXODDS_API_TOKEN="dG9rZW4tZnJvbS1vbi1jaGFpbi1zdWJzY3JpcHRpb24="

txodds fixtures snapshot
```

### Using a .env file

Create `~/.txodds/.env`:

```
TXODDS_NETWORK=mainnet
TXODDS_JWT=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
TXODDS_API_TOKEN=dG9rZW4tZnJvbS1vbi1jaGFpbi1zdWJzY3JpcHRpb24=
```
