# Scores

Scores represent match results and statistics. Each scores object contains the current state of a fixture including goals, cards, corners, and other stats, along with a timestamped sequence for verification.

---

## txodds scores snapshot

Get a snapshot of scores for a fixture, optionally at a specific point in time.

```
txodds scores snapshot --fixture-id <id> [--as-of <ts>]
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--fixture-id <id>` | Yes | Fixture ID |
| `--as-of <ts>` | No | Timestamp to query scores as of |

**Returns:** Array of `Scores` objects.

### Examples

```bash
# Get current scores for a fixture
txodds scores snapshot --fixture-id 12345

# Get scores as of a specific timestamp
txodds scores snapshot --fixture-id 12345 --as-of 1700000000
```

---

## txodds scores updates

Get the latest score updates for a fixture.

```
txodds scores updates --fixture-id <id>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--fixture-id <id>` | Yes | Fixture ID |

**Returns:** Array of `Scores` objects (latest updates).

### Example

```bash
txodds scores updates --fixture-id 12345
```

---

## txodds scores updates-by-time

Get score updates for a specific time window.

```
txodds scores updates-by-time --epoch-day <d> --hour-of-day <h> --interval <min>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--epoch-day <d>` | Yes | Days since Unix epoch (1970-01-01) |
| `--hour-of-day <h>` | Yes | Hour of day (0-23) |
| `--interval <min>` | Yes | Interval in minutes |

**Returns:** Array of `Scores` objects for the specified time window.

### Example

```bash
# Get score updates for epoch day 20000, hour 14, 15-minute interval
txodds scores updates-by-time --epoch-day 20000 --hour-of-day 14 --interval 15
```

---

## txodds scores historical

Get historical scores for a fixture.

```
txodds scores historical --fixture-id <id>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--fixture-id <id>` | Yes | Fixture ID |

**Returns:** Array of `Scores` objects (historical data).

### Example

```bash
txodds scores historical --fixture-id 12345
```

---

## txodds scores validate

Validate a score stat using Merkle proof verification.

```
txodds scores validate --fixture-id <id> --seq <n> --stat-key <key> [--stat-key2 <key>]
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--fixture-id <id>` | Yes | Fixture ID |
| `--seq <n>` | Yes | Sequence number |
| `--stat-key <key>` | Yes | Primary stat key (e.g., `goals`, `corners`) |
| `--stat-key2 <key>` | No | Secondary stat key |

**Returns:** `ScoresStatValidation` with Merkle proofs and validation results.

### Merkle proof verification

Scores validation uses a three-level Merkle proof chain:

1. **Stat proof** — proves the individual stat value belongs to the event stat root (`stat_valid`)
2. **Sub-tree proof** — proves the event stat root belongs to the summary's sub-tree root (`sub_tree_valid`)
3. **Main tree proof** — proves the summary belongs to the main tree (`main_tree_valid`)

All three must be `true` for the data to be trusted. The output includes `stat_valid`, `sub_tree_valid`, and `main_tree_valid` booleans.

### Example

```bash
# Validate goals stat
txodds scores validate --fixture-id 12345 --seq 1 --stat-key goals

# Validate with secondary stat key
txodds scores validate --fixture-id 12345 --seq 1 --stat-key goals --stat-key2 corners
```

---

## txodds scores stream

Stream live score updates via Server-Sent Events (SSE).

```
txodds scores stream [--limit <n>] [--timeout <sec>]
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--limit <n>` | No | Maximum number of events to receive |
| `--timeout <sec>` | No | Stop after N seconds |

**Returns:** Array of `SseMessage` objects.

### Example

```bash
# Stream scores with a limit of 10 events
txodds scores stream --limit 10

# Stream scores for 60 seconds
txodds scores stream --timeout 60

# Stream with both limit and timeout
txodds scores stream --limit 50 --timeout 120
```

---

## Scores object

Key fields in a `Scores` object:

| Field | Type | Description |
|-------|------|-------------|
| `FixtureId` | `u64` | Fixture identifier |
| `Id` | `u64` | Message identifier |
| `Ts` | `u64` | Timestamp |
| `Seq` | `u64` | Sequence number |
| `Stats` | `HashMap<String, u32>` | Map of stat keys to values (e.g., `goals`, `corners`, `yellow_cards`) |
| `GameState` | `String` | Current game state |
| `StartTime` | `u64` | Fixture start time |
| `Score` | `SoccerFixtureScore` | Detailed score breakdown (optional) |
| `Clock` | `SoccerFixtureClock` | Match clock info (optional) |

---

## Practical examples

### Get current scores for a fixture

```bash
txodds scores snapshot --fixture-id 12345
```

### Get historical scores

```bash
txodds scores historical --fixture-id 12345
```

### Validate a score stat

```bash
txodds scores validate --fixture-id 12345 --seq 1 --stat-key goals
```

### Stream live scores

```bash
txodds scores stream --limit 10 --timeout 60
```

---

## Output format

- **Default:** Pretty-printed JSON
- **With `--raw`:** Raw JSON (useful for piping to `jq`)

```bash
# Pipe to jq for processing
txodds scores snapshot --fixture-id 12345 --raw | jq '.[0].Stats'
```
