# Example Terminal Sessions for svmai-cli TUI

This document shows example terminal sessions that would be recorded during QA testing.

## Session 1: First Launch and Help Screen

```
$ ./target/release/svmai
Starting svmai tool in TUI mode...

┌────────────────────────────────────────────────────────────────────┐
│                    svmai - Solana Wallet Manager                   │
│                           Version 0.1.0                            │
└────────────────────────────────────────────────────────────────────┘
┌─ Wallet List ──────────────────────────────────────────────────────┐
│                                                                    │
│                    No wallets found                                │
│                                                                    │
│              Press 'A' to add your first wallet                    │
│              Press 'H' to see all available commands               │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Welcome to svmai! Press 'H' for help                      │
└────────────────────────────────────────────────────────────────────┘

[User presses 'H']

┌────────────────────────────────────────────────────────────────────┐
│                    svmai - Solana Wallet Manager                   │
│                           Version 0.1.0                            │
└────────────────────────────────────────────────────────────────────┘
┌─ Help - Keyboard Shortcuts ────────────────────────────────────────┐
│                                                                    │
│  NAVIGATION                                                        │
│  ↑ / ↓        Navigate through lists                               │
│  Enter        Select item or confirm action                        │
│  Esc          Return to previous view / Cancel                     │
│  Q            Quit application                                     │
│                                                                    │
│  WALLET MANAGEMENT                                                 │
│  A            Add new wallet from file                             │
│  V            Create vanity wallet (with 'ai' prefix)              │
│  D            Delete selected wallet (with confirmation)           │
│  R            Refresh wallet list and balances                     │
│  /            Search wallets by name                               │
│  B            Access batch operations menu                         │
│                                                                    │
│  INFORMATION                                                       │
│  H            Show this help screen                                │
│  Enter        View detailed wallet information (in list view)      │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Press Esc to close help                                   │
└────────────────────────────────────────────────────────────────────┘

[User presses 'Esc']
[Returns to wallet list view]
```

## Session 2: Adding a Wallet

```
[Starting from wallet list view with no wallets]

[User presses 'A']

┌─ Add Wallet ───────────────────────────────────────────────────────┐
│                                                                    │
│  Enter the path to your Solana wallet JSON file:                  │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ _                                                            │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  Press Enter to confirm, Esc to cancel                             │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Enter wallet file path                                    │
└────────────────────────────────────────────────────────────────────┘

[User types: /home/user/.solana/main-wallet.json]

┌─ Add Wallet ───────────────────────────────────────────────────────┐
│                                                                    │
│  Enter the path to your Solana wallet JSON file:                  │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ /home/user/.solana/main-wallet.json_                        │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  Press Enter to confirm, Esc to cancel                             │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Enter wallet file path                                    │
└────────────────────────────────────────────────────────────────────┘

[User presses Enter]
[System validates wallet, encrypts keys, saves to storage]

┌─ Wallet List ──────────────────────────────────────────────────────┐
│                                                                    │
│  > main-wallet                                                     │
│    Public Key: 7xKX...9mYz                                         │
│    Balance: Loading...                                             │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: ✓ Wallet added successfully!                              │
└────────────────────────────────────────────────────────────────────┘

[After a moment, balance updates]

┌─ Wallet List ──────────────────────────────────────────────────────┐
│                                                                    │
│  > main-wallet                                                     │
│    Public Key: 7xKX...9mYz                                         │
│    Balance: 2.456 SOL                                              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Balance updated                                           │
└────────────────────────────────────────────────────────────────────┘
```

## Session 3: Viewing Wallet Details

```
[Starting from wallet list with one wallet]

[User presses Enter on selected wallet]

┌─ Wallet Details ───────────────────────────────────────────────────┐
│                                                                    │
│  Name: main-wallet                                                 │
│                                                                    │
│  Public Key:                                                       │
│  7xKXvVzaD9qJ4mYzPk3FnH8gR2tWbL5cX1uN6vM9pQ8s                     │
│                                                                    │
│  SOL Balance: 2.456789123 SOL                                      │
│                                                                    │
│  ┌─ SPL Token Balances ─────────────────────────────────────────┐ │
│  │                                                              │ │
│  │  Token: USD Coin (USDC)                                      │ │
│  │  Amount: 150.00                                              │ │
│  │  Mint: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v          │ │
│  │                                                              │ │
│  │  Token: Bonk (BONK)                                          │ │
│  │  Amount: 5000000.00                                          │ │
│  │  Mint: DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263          │ │
│  │                                                              │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  Last Transaction:                                                 │
│  Signature: 5J4L...Km9P                                            │
│  Status: Confirmed                                                 │
│  Date: 2025-11-04 18:30:22                                         │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Press Esc to return to wallet list                        │
└────────────────────────────────────────────────────────────────────┘
```

## Session 4: Vanity Wallet Generation

```
[User presses 'V' from wallet list]

┌─ Create Vanity Wallet ─────────────────────────────────────────────┐
│                                                                    │
│  Enter wallet name:                                                │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ my_ai_wallet_                                                │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  Prefix: ai (case-insensitive, fixed)                              │
│                                                                    │
│  Press Enter to start generation, Esc to cancel                    │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘

[User presses Enter, generation starts]

┌─ Creating Vanity Wallet ───────────────────────────────────────────┐
│                                                                    │
│  Wallet Name: my_ai_wallet                                         │
│  Prefix: ai (case-insensitive)                                     │
│                                                                    │
│  ┌─ Progress ─────────────────────────────────────────────────┐   │
│  │                                                            │   │
│  │  Attempts:   156,789                                       │   │
│  │  Speed:      52,263 attempts/sec                           │   │
│  │  Elapsed:    3 seconds                                     │   │
│  │                                                            │   │
│  │  [██████░░░░░░░░░░░░░░░░░░░░░░] Searching...               │   │
│  │                                                            │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  Using 8 CPU threads for parallel generation                       │
│                                                                    │
│  Press Esc to cancel                                               │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Generating vanity wallet... This may take a few minutes   │
└────────────────────────────────────────────────────────────────────┘

[Progress continues updating...]

┌─ Creating Vanity Wallet ───────────────────────────────────────────┐
│                                                                    │
│  Wallet Name: my_ai_wallet                                         │
│  Prefix: ai (case-insensitive)                                     │
│                                                                    │
│  ┌─ Progress ─────────────────────────────────────────────────┐   │
│  │                                                            │   │
│  │  Attempts:   2,847,562                                     │   │
│  │  Speed:      49,875 attempts/sec                           │   │
│  │  Elapsed:    57 seconds                                    │   │
│  │                                                            │   │
│  │  [████████████████████████░░░░] Searching...               │   │
│  │                                                            │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  Using 8 CPU threads for parallel generation                       │
│                                                                    │
│  Press Esc to cancel                                               │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Still searching for matching address...                   │
└────────────────────────────────────────────────────────────────────┘

[After finding match]

┌─ Wallet List ──────────────────────────────────────────────────────┐
│                                                                    │
│    main-wallet                                                     │
│    Public Key: 7xKX...9mYz                                         │
│    Balance: 2.456 SOL                                              │
│                                                                    │
│  > my_ai_wallet                                                    │
│    Public Key: ai9N...kL2m                                         │
│    Balance: 0.000 SOL                                              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: ✓ Vanity wallet created! Address starts with 'ai'         │
└────────────────────────────────────────────────────────────────────┘
```

## Session 5: Search and Delete

```
[Starting from wallet list with multiple wallets]

[User presses '/']

┌─ Wallet List (Search Active) ──────────────────────────────────────┐
│                                                                    │
│  Search: _                                                         │
│                                                                    │
│  > main-wallet                                                     │
│    Public Key: 7xKX...9mYz                                         │
│    Balance: 2.456 SOL                                              │
│                                                                    │
│    trading-wallet                                                  │
│    Public Key: 3bYp...7wQr                                         │
│    Balance: 0.123 SOL                                              │
│                                                                    │
│    my_ai_wallet                                                    │
│    Public Key: ai9N...kL2m                                         │
│    Balance: 0.000 SOL                                              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Type to search, Enter to confirm, Esc to cancel           │
└────────────────────────────────────────────────────────────────────┘

[User types 'trading']

┌─ Wallet List (Search Active) ──────────────────────────────────────┐
│                                                                    │
│  Search: trading_                                                  │
│                                                                    │
│  > trading-wallet                                                  │
│    Public Key: 3bYp...7wQr                                         │
│    Balance: 0.123 SOL                                              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: 1 wallet matches | Enter to confirm, Esc to clear         │
└────────────────────────────────────────────────────────────────────┘

[User presses Enter, then 'D' to delete]

┌─ Confirm Delete ───────────────────────────────────────────────────┐
│                                                                    │
│  ⚠️  WARNING ⚠️                                                     │
│                                                                    │
│  Are you sure you want to delete this wallet?                      │
│                                                                    │
│  Wallet: trading-wallet                                            │
│  Public Key: 3bYp...7wQr                                           │
│  Balance: 0.123 SOL                                                │
│                                                                    │
│  This action cannot be undone!                                     │
│  The private key will be permanently removed from storage.         │
│                                                                    │
│  Press Y to confirm, N or Esc to cancel                            │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: ⚠️  Confirm deletion (Y/N)                                 │
└────────────────────────────────────────────────────────────────────┘

[User presses 'Y']

┌─ Wallet List ──────────────────────────────────────────────────────┐
│                                                                    │
│  > main-wallet                                                     │
│    Public Key: 7xKX...9mYz                                         │
│    Balance: 2.456 SOL                                              │
│                                                                    │
│    my_ai_wallet                                                    │
│    Public Key: ai9N...kL2m                                         │
│    Balance: 0.000 SOL                                              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: ✓ Wallet deleted successfully                              │
└────────────────────────────────────────────────────────────────────┘
```

## Session 6: Error Handling

```
[User attempts to add invalid wallet file]

[User presses 'A']
[User types path to non-existent file]

┌─ Wallet List ──────────────────────────────────────────────────────┐
│                                                                    │
│    main-wallet                                                     │
│    Public Key: 7xKX...9mYz                                         │
│    Balance: 2.456 SOL                                              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: ❌ Error: File not found                                   │
└────────────────────────────────────────────────────────────────────┘

[After 5 seconds, status clears]

[User tries again with invalid JSON file]

┌─ Wallet List ──────────────────────────────────────────────────────┐
│                                                                    │
│    main-wallet                                                     │
│    Public Key: 7xKX...9mYz                                         │
│    Balance: 2.456 SOL                                              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: ❌ Error: Invalid wallet file format                       │
└────────────────────────────────────────────────────────────────────┘
```

## Recording Notes

These sessions demonstrate:
1. ✅ Intuitive navigation
2. ✅ Clear status messages
3. ✅ Responsive UI updates
4. ✅ Proper error handling
5. ✅ Confirmation for destructive actions
6. ✅ Real-time progress indicators
7. ✅ Context-sensitive help
8. ✅ Clean visual design

To create actual recordings:
- Use `asciinema rec session-name.cast`
- Follow the test-tui.sh script
- Convert to GIF using `agg` or similar tool
- Each recording should be 30-120 seconds

---

**These are example/mock sessions. Actual TUI behavior may vary slightly.**
**Recordings should be made on a properly configured development machine.**
