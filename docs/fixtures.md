# Fixtures

Fixtures represent sports events/matches. Each fixture contains participant names, competition info, start time, and unique identifiers.

---

## txodds fixtures snapshot

Get a snapshot of all fixtures, optionally filtered by competition.

```
txodds fixtures snapshot [--competition-id <id>]
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--competition-id <id>` | No | Filter by competition ID |

**Returns:** Array of `Fixture` objects.

### Examples

```bash
# Get all fixtures
txodds fixtures snapshot

# Get fixtures for a specific competition
txodds fixtures snapshot --competition-id 39
```

---

## txodds fixtures updates

Get fixture updates for a specific time window.

```
txodds fixtures updates --epoch-day <d> --hour-of-day <h>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--epoch-day <d>` | Yes | Days since Unix epoch (1970-01-01) |
| `--hour-of-day <h>` | Yes | Hour of day (0-23) |

**Returns:** Array of `Fixture` objects for the specified time window.

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
# Get fixture updates for epoch day 20000, hour 14
txodds fixtures updates --epoch-day 20000 --hour-of-day 14
```

---

## txodds fixtures validate

Validate a fixture by its ID using Merkle proof verification.

```
txodds fixtures validate --fixture-id <id> [--timestamp <ts>]
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--fixture-id <id>` | Yes | Fixture ID to validate |
| `--timestamp <ts>` | No | Expected timestamp |

**Returns:** `ValidationResult` containing the fixture data and two boolean fields:

- `sub_tree_valid` — verifies the fixture against the sub-tree root
- `main_tree_valid` — verifies the sub-tree summary against the main tree

**Both fields must be `true` for the data to be trusted.**

### Merkle proof verification

The TxODDS API uses a two-level Merkle tree structure:

1. **Sub-tree:** Contains individual fixture data. The `sub_tree_valid` field confirms the fixture hash matches the sub-tree root.
2. **Main tree:** Contains batch summaries. The `main_tree_valid` field confirms the summary hash matches the main tree root.

This dual verification ensures data integrity from the individual fixture up to the batch level.

### Example

```bash
# Validate a fixture
txodds fixtures validate --fixture-id 12345

# Validate with expected timestamp
txodds fixtures validate --fixture-id 12345 --timestamp 1700000001
```

---

## txodds fixtures batch-validate

Batch-validate all fixtures for a specific time window.

```
txodds fixtures batch-validate --epoch-day <d> --hour-of-day <h>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--epoch-day <d>` | Yes | Days since Unix epoch (1970-01-01) |
| `--hour-of-day <h>` | Yes | Hour of day (0-23) |

**Returns:** Array of `ValidationResult` objects.

### Example

```bash
# Batch-validate fixtures for a time window
txodds fixtures batch-validate --epoch-day 20000 --hour-of-day 14
```

---

## Output format

By default, output is pretty-printed JSON. Use `--raw` for compact JSON (useful for piping to `jq`):

```bash
# Pretty-printed
txodds fixtures snapshot

# Raw JSON for piping
txodds fixtures snapshot --raw | jq '.[] | select(.CompetitionId == 39)'
```

---

## Fixture object

A `Fixture` object contains the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `FixtureId` | u64 | Unique fixture identifier |
| `StartTime` | u64 | Match start time (Unix timestamp) |
| `Participant1` | string | Name of first participant/team |
| `Participant2` | string | Name of second participant/team |
| `Participant1IsHome` | bool | Whether participant1 is the home team |
| `Competition` | string | Competition name |
| `CompetitionId` | u64 | Competition identifier |
| `Participant1Id` | u64 | Participant1 identifier |
| `Participant2Id` | u64 | Participant2 identifier |
| `FixtureGroupId` | u64 | Fixture group identifier |
| `Ts` | u64 | Timestamp of the fixture data |

---

## Practical examples

### Get all fixtures and filter by competition

```bash
txodds fixtures snapshot --raw | jq '.[] | select(.CompetitionId == 39)'
```

### Get fixtures for a specific date

```bash
# Calculate epoch-day for 2024-11-15
EPOCH_DAY=$(echo $(( $(date -d "2024-11-15" +%s) / 86400 )))

# Get updates for hour 14
txodds fixtures updates --epoch-day $EPOCH_DAY --hour-of-day 14
```

### Validate a fixture and check trust

```bash
txodds fixtures validate --fixture-id 12345 --raw | jq '{subTree: .sub_tree_valid, mainTree: .main_tree_valid}'
```

Output:

```json
{
  "subTree": true,
  "mainTree": true
}
```

Both values must be `true` to trust the data.
