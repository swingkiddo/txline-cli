# Global Flags

Global flags can be used with any command. They modify the behavior of the CLI across all subcommands.

---

## --network

Override the network configuration.

```
--network <mainnet|devnet>
```

**Default:** `devnet` (or value from `credentials.toml`)

**Example:**

```bash
# Use mainnet for this command
txodds --network mainnet fixtures snapshot

# Use devnet explicitly
txodds --network devnet auth guest
```

The network flag overrides the `network` field in `credentials.toml` and the `TXODDS_NETWORK` environment variable.

---

## --raw

Output raw JSON instead of pretty-printed JSON.

```
--raw
```

**Default:** `false` (pretty-printed)

**Example:**

```bash
# Pretty-printed (default)
txodds fixtures snapshot

# Raw JSON for piping
txodds fixtures snapshot --raw | jq '.[] | select(.CompetitionId == 39)'
```

Use `--raw` when piping output to other tools like `jq`, `grep`, or when processing output programmatically.

---

## --config

Specify a custom configuration file path.

```
--config <path>
```

**Default:** `~/.txodds/credentials.toml`

**Example:**

```bash
# Use custom config file
txodds --config /path/to/custom/credentials.toml fixtures snapshot
```

The config file must be in TOML format with the same structure as `credentials.toml`.

---

## Combining flags

Global flags can be combined and used with any command:

```bash
txodds --network mainnet --raw fixtures snapshot --competition-id 42
```

Order doesn't matter — flags can appear before or after the subcommand:

```bash
# Both are valid
txodds --network mainnet fixtures snapshot
txodds fixtures snapshot --network mainnet
```

---

## Priority

Configuration values are resolved in the following order (highest to lowest priority):

1. Command-line flags (`--network`, `--config`)
2. Environment variables (`TXODDS_NETWORK`, `TXODDS_JWT`, etc.)
3. `credentials.toml` file
4. Defaults (`devnet`, empty JWT, etc.)
