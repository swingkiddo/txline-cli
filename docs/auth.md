# Authentication

The TxODDS API uses a three-step authentication flow:

1. **Guest auth** — obtain a JWT token
2. **On-chain subscribe** — register a subscription on Solana, receiving a transaction signature (`txSig`)
3. **Activate token** — sign the `txSig` with your wallet to receive a persistent API token

All credentials are stored in `~/.txodds/credentials.toml`.

---

## txodds auth guest

Request a guest JWT token. No flags required.

```
txodds auth guest
```

The JWT is automatically saved to `~/.txodds/credentials.toml` and expires in 30 days.

**Example output:**

```
Guest authentication successful, JWT saved
JWT: eyJhbGciOiJIUzI1NiIsInR5cCI6...
```

---

## txodds subscribe

Submit an on-chain subscription transaction.

```
txodds subscribe --keypair <path> --service-level <id> --weeks <n> --rpc <url>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--keypair <path>` | Yes | Path to Solana keypair file (64-byte Ed25519 JSON array) |
| `--service-level <id>` | Yes | Service level ID (`1` for free World Cup tier) |
| `--weeks <n>` | Yes | Subscription duration in weeks (`4` for free tier) |
| `--rpc <url>` | Yes | Solana RPC URL |

**Returns:** `txSig` (transaction signature) — required for the activate step.

**Example (devnet, free tier):**

```bash
txodds subscribe \
  --keypair ~/.config/solana/id.json \
  --service-level 1 \
  --weeks 4 \
  --rpc https://api.devnet.solana.com
```

Output:

```
Subscribe transaction sent: 5Abc...xyz
txSig: 5Abc...xyz
```

---

## txodds auth activate

Activate a persistent API token using the transaction signature from the subscribe step.

```
txodds auth activate --tx-sig <sig> --keypair <path>
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--tx-sig <sig>` | Yes | Transaction signature returned by `txodds subscribe` |
| `--keypair <path>` | Yes | Same keypair used for the subscribe step |

The command signs the message `${txSig}:${leagues.join(",")}:${jwt}` with the wallet's Ed25519 key and sends it to the API. The resulting API token is automatically saved to `~/.txodds/credentials.toml`.

**Example:**

```bash
txodds auth activate \
  --tx-sig 5Abc...xyz \
  --keypair ~/.config/solana/id.json
```

Output:

```
API token activated and saved
```

---

## Complete example

Full authentication flow from start to finish:

```bash
# 1. Guest auth — obtain JWT
txodds auth guest

# 2. Subscribe on-chain (free tier)
txodds subscribe \
  --keypair ~/.config/solana/id.json \
  --service-level 1 \
  --weeks 4 \
  --rpc https://api.devnet.solana.com

# 3. Activate token (use txSig from step 2)
txodds auth activate \
  --tx-sig <txSig_from_step_2> \
  --keypair ~/.config/solana/id.json
```

After step 3, all subsequent API calls use the saved token automatically.

---

## Troubleshooting

| Error | Cause / Fix |
|-------|-------------|
| `Invalid keypair` | Keypair file must be a 64-byte Ed25519 JSON array (e.g. `~/.config/solana/id.json` from `solana-keygen new`) |
| `JWT expired` | Guest JWTs expire after 30 days — run `txodds auth guest` again |
| `Token activation failed` | Verify the `txSig` is correct and from a recent transaction; old signatures may no longer be valid |
