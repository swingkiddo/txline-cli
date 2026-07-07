# Odds

Odds represent betting prices for fixtures. Each odds payload contains bookmaker information, market details, price names, and corresponding prices for a specific fixture.

---

## txodds odds snapshot

Get a snapshot of all current odds for a fixture.

```
txodds odds snapshot --fixture-id <id>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--fixture-id <id>` | Yes | Fixture ID |

**Returns:** Array of `OddsPayload` objects.

### Example

```bash
txodds odds snapshot --fixture-id 12345
```

---

## txodds odds updates

Get the latest odds updates for a fixture.

```
txodds odds updates --fixture-id <id>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--fixture-id <id>` | Yes | Fixture ID |

**Returns:** Array of `OddsPayload` objects (latest updates).

### Example

```bash
txodds odds updates --fixture-id 12345
```

---

## txodds odds updates-by-time

Get odds updates for a specific time window.

```
txodds odds updates-by-time --epoch-day <d> --hour-of-day <h> --interval <min>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--epoch-day <d>` | Yes | Days since Unix epoch (1970-01-01) |
| `--hour-of-day <h>` | Yes | Hour of day (0-23) |
| `--interval <min>` | Yes | Interval in minutes |

**Returns:** Array of `OddsPayload` objects for the specified time window.

### Epoch day calculation

The `--epoch-day` value is the number of days since January 1, 1970:

```bash
# Calculate epoch-day for today
echo $(( $(date +%s) / 86400 ))

# Calculate epoch-day for a specific date (e.g., 2024-01-15)
echo $(( $(date -d "2024-01-15" +%s) / 86400 ))
```

### Example

```bash
# Get odds updates for epoch day 20000, hour 14, 60-minute interval
txodds odds updates-by-time --epoch-day 20000 --hour-of-day 14 --interval 60
```

---

## txodds odds validate

Validate an odds message using Merkle proof verification.

```
txodds odds validate --message-id <id> --ts <ts>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--message-id <id>` | Yes | Message ID to validate |
| `--ts <ts>` | Yes | Timestamp |

**Returns:** `ValidationResult` containing the odds data and two boolean fields:

- `sub_tree_valid` — verifies the odds payload against the sub-tree root
- `main_tree_valid` — verifies the batch summary against the main tree

**Both fields must be `true` for the data to be trusted.**

### Merkle proof verification

The TxODDS API uses a two-level Merkle tree structure:

1. **Sub-tree:** Contains individual odds payloads. The `sub_tree_valid` field confirms the odds hash matches the sub-tree root.
2. **Main tree:** Contains batch summaries. The `main_tree_valid` field confirms the summary hash matches the main tree root.

This dual verification ensures data integrity from the individual odds message up to the batch level.

### Example

```bash
# Validate an odds message
txodds odds validate --message-id "msg-001" --ts 1700000002
```

---

## txodds odds stream

Stream live odds updates via Server-Sent Events (SSE).

```
txodds odds stream [--limit <n>] [--timeout <sec>]
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--limit <n>` | No | Maximum number of events to receive |
| `--timeout <sec>` | No | Stop after N seconds |

**Returns:** Array of `SseMessage` objects.

The stream runs indefinitely unless `--limit` or `--timeout` is specified. Each event is printed to stdout as it arrives.

### Examples

```bash
# Stream odds indefinitely (Ctrl+C to stop)
txodds odds stream

# Receive up to 10 events
txodds odds stream --limit 10

# Stream for 30 seconds
txodds odds stream --timeout 30

# Combine limit and timeout
txodds odds stream --limit 5 --timeout 60
```

---

## Output format

By default, output is pretty-printed JSON. Use `--raw` for compact JSON (useful for piping to `jq`):

```bash
# Pretty-printed
txodds odds snapshot --fixture-id 12345

# Raw JSON for piping
txodds odds snapshot --fixture-id 12345 --raw | jq '.[] | select(.SuperOddsType == "moneyline")'
```

---

## OddsPayload object

An `OddsPayload` object contains the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `FixtureId` | u64 | Fixture identifier |
| `MessageId` | string | Unique message identifier |
| `Ts` | u64 | Timestamp of the odds data |
| `Bookmaker` | string | Bookmaker name |
| `BookmakerId` | u64 | Bookmaker identifier |
| `SuperOddsType` | string | Odds type (e.g., "moneyline") |
| `GameState` | string? | Game state (e.g., "PreMatch", "InPlay") |
| `InRunning` | bool | Whether the market is in-running (live) |
| `MarketParameters` | string? | Additional market parameters |
| `MarketPeriod` | string? | Market period (e.g., "FullTime", "FirstHalf") |
| `PriceNames` | []string | Names of the prices (e.g., ["home", "draw", "away"]) |
| `Prices` | []f64 | Decimal odds corresponding to PriceNames |
| `Pct` | []string | Implied probabilities as percentages |

---

## SseMessage object

An `SseMessage` object contains the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `id` | string? | Event ID |
| `event` | string? | Event type |
| `data` | string | Event payload (JSON string) |

---

## Practical examples

### Get odds for a fixture

```bash
txodds odds snapshot --fixture-id 12345 --raw | jq '.[] | {Bookmaker, PriceNames, Prices}'
```

### Stream live odds with limit

```bash
txodds odds stream --limit 5 --raw | jq '.[].data'
```

### Validate an odds message and check trust

```bash
txodds odds validate --message-id "msg-001" --ts 1700000002 --raw | jq '{subTree: .sub_tree_valid, mainTree: .main_tree_valid}'
```

Output:

```json
{
  "subTree": true,
  "mainTree": true
}
```

Both values must be `true` to trust the data.
