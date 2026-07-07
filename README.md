# txodds — Solana Sports Data CLI

[![Release](https://img.shields.io/github/v/release/swingkiddo/txline-cli)](https://github.com/swingkiddo/txline-cli/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/actions/workflow/status/swingkiddo/txline-cli/ci.yml?branch=master&label=CI)](https://github.com/swingkiddo/txline-cli/actions/workflows/ci.yml)

`txodds` is the official Rust CLI for the **TxODDS / TxLINE** protocol — a Solana-powered sports data oracle delivering fixtures, odds, and scores with on-chain subscription and Merkle proof verification. Subscriptions are Solana transactions; every payload can be verified locally via SHA-256 Merkle proofs, removing the need to trust the server. Stream live odds and scores over SSE, take snapshots, validate — all in a single cross-platform binary.

## Features

- **Solana-native auth** — guest JWT → on-chain subscription → token activation signed with Ed25519
- **Trustless verification** — two-level (fixtures/odds) and three-level (scores) Merkle proofs; verify locally with zero server trust
- **Live streaming** — Server-Sent Events for odds and scores with `--limit` / `--timeout`
- **Multi-network** — mainnet / devnet, switch via `--network` flag or environment variable
- **Unix-friendly** — `--raw` JSON for `jq` pipelines, shell completions for bash/zsh/fish/powershell
- **Cross-platform** — single binary for Linux x86_64/arm64, macOS Intel/Apple Silicon, Windows

## Install

**macOS / Linux:**
```bash
curl -sfL https://github.com/swingkiddo/txline-cli/releases/latest/download/install.sh | sh
```

**Windows (PowerShell):**
```powershell
iwr -useb https://github.com/swingkiddo/txline-cli/releases/latest/download/install.ps1 | iex
```

**From source:**
```bash
cargo install --git https://github.com/swingkiddo/txline-cli txodds
```

| Env var | Default | Purpose |
|---|---|---|
| `TXODDS_INSTALL_REPO` | `swingkiddo/txline-cli` | GitHub repo to install from |
| `TXODDS_INSTALL_DIR` | `~/.txodds/bin` | Binary install directory |

Details: [docs/install.md](docs/install.md)

## Quickstart

```bash
# 1. Guest JWT
txodds auth guest

# 2. Subscribe on-chain (devnet, free tier)
txodds subscribe \
  --keypair ~/.config/solana/id.json \
  --service-level 1 --weeks 4 \
  --rpc https://api.devnet.solana.com

# 3. Activate token (use txSig from step 2)
txodds auth activate --tx-sig <SIG> --keypair ~/.config/solana/id.json

# 4. Snapshot fixtures
txodds fixtures snapshot --raw | jq '.[0] | {FixtureId, Participant1, Participant2, Competition}'
```

```json
{
  "FixtureId": 12345,
  "Participant1": "Arsenal",
  "Participant2": "Chelsea",
  "Competition": "Premier League"
}
```

```bash
# 5. Live odds stream
txodds odds stream --limit 3
```

## Authentication

Three-step flow:

1. **`txodds auth guest`** — obtain a guest JWT (30-day expiry)
2. **`txodds subscribe`** — send a Solana transaction (PDA, Ed25519) and receive a `txSig`
3. **`txodds auth activate`** — sign `txSig + JWT` with your keypair to activate a persistent API token

Details: [docs/auth.md](docs/auth.md)

## Commands

| Command | Description | Reference |
|---|---|---|
| `txodds auth guest` | Guest JWT | [docs/auth.md](docs/auth.md#txodds-auth-guest) |
| `txodds subscribe` | On-chain subscription | [docs/auth.md](docs/auth.md#txodds-subscribe) |
| `txodds auth activate` | Token activation | [docs/auth.md](docs/auth.md#txodds-auth-activate) |
| `txodds fixtures snapshot` | Snapshot all fixtures | [docs/fixtures.md](docs/fixtures.md#txodds-fixtures-snapshot) |
| `txodds fixtures updates` | Fixture updates by time | [docs/fixtures.md](docs/fixtures.md#txodds-fixtures-updates) |
| `txodds fixtures validate` | Merkle-verify a fixture | [docs/fixtures.md](docs/fixtures.md#txodds-fixtures-validate) |
| `txodds fixtures batch-validate` | Batch verify fixtures | [docs/fixtures.md](docs/fixtures.md#txodds-fixtures-batch-validate) |
| `txodds odds snapshot` | Odds for a fixture | [docs/odds.md](docs/odds.md#txodds-odds-snapshot) |
| `txodds odds updates` | Latest odds updates | [docs/odds.md](docs/odds.md#txodds-odds-updates) |
| `txodds odds updates-by-time` | Odds by time window | [docs/odds.md](docs/odds.md#txodds-odds-updates-by-time) |
| `txodds odds validate` | Merkle-verify odds | [docs/odds.md](docs/odds.md#txodds-odds-validate) |
| `txodds odds stream` | Live odds (SSE) | [docs/odds.md](docs/odds.md#txodds-odds-stream) |
| `txodds scores snapshot` | Score snapshot | [docs/scores.md](docs/scores.md#txodds-scores-snapshot) |
| `txodds scores updates` | Latest score updates | [docs/scores.md](docs/scores.md#txodds-scores-updates) |
| `txodds scores updates-by-time` | Scores by time window | [docs/scores.md](docs/scores.md#txodds-scores-updates-by-time) |
| `txodds scores historical` | Historical scores | [docs/scores.md](docs/scores.md#txodds-scores-historical) |
| `txodds scores validate` | Merkle-verify a score stat | [docs/scores.md](docs/scores.md#txodds-scores-validate) |
| `txodds scores stream` | Live scores (SSE) | [docs/scores.md](docs/scores.md#txodds-scores-stream) |
| `txodds completions <shell>` | Shell completions | — |

Global flags: [docs/global-flags.md](docs/global-flags.md) (`--network`, `--raw`, `--config`)

## Configuration

| Network | API Host |
|---|---|
| `mainnet` | `https://txline.txodds.com` |
| `devnet` | `https://txline-dev.txodds.com` |

Configuration is read from `~/.txodds/credentials.toml` (TOML) and environment variables (`TXODDS_NETWORK`, `TXODDS_JWT`, `TXODDS_API_TOKEN`). Priority: flags > env > config > defaults.

Details: [docs/config.md](docs/config.md)

## Data Integrity (Merkle Proofs)

All API data can be verified locally using Solana-compatible Merkle trees (Borsh + SHA-256).

- **Fixtures / Odds (2 levels):** leaf → sub-tree root → main tree root
- **Scores (3 levels):** stat → event stat root → sub-tree root → main tree root

```bash
txodds scores validate --fixture-id 12345 --seq 1 --stat-key goals --raw \
  | jq '{stat_valid, sub_tree_valid, main_tree_valid}'
```

All three fields must be `true` for the data to be trusted.

## Supported Platforms

| Target | Archive |
|---|---|
| `x86_64-unknown-linux-gnu` | `.tar.xz` |
| `aarch64-unknown-linux-gnu` | `.tar.xz` |
| `x86_64-apple-darwin` | `.tar.xz` |
| `aarch64-apple-darwin` (Apple Silicon) | `.tar.xz` |
| `x86_64-pc-windows-msvc` | `.zip` |

## Build from Source

```bash
git clone https://github.com/swingkiddo/txline-cli
cd txline-cli
cargo build --release
./target/release/txodds --version
```

## Development

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked --all-targets
```

- CI (`ci.yml`): runs fmt, clippy, test, release build on ubuntu/macos/windows on PR and push to master
- Releases (`release.yml`): on tag `v*`, builds 5 targets and publishes a GitHub Release

## Project Layout

```
src/
  cli.rs              — clap derive, all subcommands
  main.rs             — entry point
  auth.rs             — guest JWT, token activation
  subscribe/          — on-chain subscription (PDA, instruction, RPC send)
  fixtures.rs         — fixtures commands
  odds.rs             — odds commands
  scores.rs           — scores commands
  stream.rs           — SSE parser
  validation.rs       — Merkle proof verification (SHA-256, Borsh)
  api.rs              — reqwest HTTP client
  config/             — TOML loader + network enum
  types/              — serde + borsh types
docs/                 — user-facing reference
install.sh            — Unix installer
install.ps1           — Windows installer
```

## License

MIT © swingkiddo

## See Also

- [TxODDS / TxLINE Quickstart](https://txline.txodds.com/documentation/quickstart) — official API documentation
