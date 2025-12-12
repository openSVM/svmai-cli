# svmai-cli TUI Feature Documentation

## TUI Layout Overview

```
┌────────────────────────────────────────────────────────────────────┐
│                        svmai - Wallet Manager                      │
│                        Version: 0.1.0                              │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│                                                                    │
│  Main Content Area                                                 │
│  (Dynamically changes based on current view)                       │
│                                                                    │
│  - Wallet List View                                                │
│  - Wallet Detail View                                              │
│  - Help Screen                                                     │
│  - Add Wallet Input                                                │
│  - Search Input                                                    │
│  - Vanity Wallet Progress                                          │
│  - Confirmation Dialogs                                            │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────┐
│ Status: Info/Success/Warning/Error message here                   │
│ Tip: Press 'H' for help, 'Q' to quit                              │
└────────────────────────────────────────────────────────────────────┘
```

## View Details

### 1. Wallet List View

```
┌─ Wallet List ──────────────────────────────────────────────────────┐
│                                                                    │
│  > Wallet 1                                                        │
│    Public Key: AbC1...XyZ9                                         │
│    Balance: 1.234 SOL                                              │
│                                                                    │
│    Wallet 2                                                        │
│    Public Key: DeF2...UvW8                                         │
│    Balance: 0.567 SOL                                              │
│                                                                    │
│    Wallet 3                                                        │
│    Public Key: GhI3...RsT7                                         │
│    Balance: 5.890 SOL                                              │
│                                                                    │
│  [Empty space for more wallets]                                    │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
Status: 3 wallets loaded | Press 'A' to add, 'V' for vanity, '/' to search
```

**Keyboard Shortcuts:**
- `↑/↓` - Navigate through wallets
- `Enter` - View wallet details
- `A` - Add new wallet
- `V` - Create vanity wallet
- `D` - Delete selected wallet
- `R` - Refresh balances
- `/` - Search wallets
- `B` - Batch operations
- `H` - Help
- `Q` - Quit

### 2. Wallet Detail View

```
┌─ Wallet Details ───────────────────────────────────────────────────┐
│                                                                    │
│  Name: My Main Wallet                                              │
│                                                                    │
│  Public Key:                                                       │
│  AbCdEfGhIjKlMnOpQrStUvWxYz1234567890abcdefghijklmnopqrstuv      │
│                                                                    │
│  SOL Balance: 1.234567890 SOL                                      │
│                                                                    │
│  ┌─ SPL Token Balances ─────────────────────────────────────────┐ │
│  │                                                              │ │
│  │  Token: USDC                                                 │ │
│  │  Amount: 100.50                                              │ │
│  │  Mint: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v          │ │
│  │                                                              │ │
│  │  Token: BONK                                                 │ │
│  │  Amount: 1000000.00                                          │ │
│  │  Mint: DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263          │ │
│  │                                                              │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  Last Transaction:                                                 │
│  Signature: 5xYz...Abc1                                            │
│  Status: Confirmed                                                 │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
Status: Press Esc to return to wallet list
```

### 3. Help Screen

```
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
│  TIPS                                                              │
│  - All operations are secure and encrypted                         │
│  - Private keys never leave your device                            │
│  - Balances require network connection                             │
│  - Vanity wallet generation may take time                          │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
Status: Press Esc to close help
```

### 4. Add Wallet Input

```
┌─ Add Wallet ───────────────────────────────────────────────────────┐
│                                                                    │
│  Enter the path to your Solana wallet JSON file:                  │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ /path/to/wallet.json_                                        │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  Press Enter to confirm, Esc to cancel                             │
│                                                                    │
│  Note: The wallet file should be a JSON array of 64 bytes          │
│  representing a Solana keypair.                                    │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
Status: Enter wallet file path
```

### 5. Search Input

```
┌─ Wallet List (Search Active) ──────────────────────────────────────┐
│                                                                    │
│  Search: main_                                                     │
│                                                                    │
│  > Main Wallet                                                     │
│    Public Key: AbC1...XyZ9                                         │
│    Balance: 1.234 SOL                                              │
│                                                                    │
│  [No other matches]                                                │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
Status: 1 wallet matches | Press Enter to confirm, Esc to clear search
```

### 6. Vanity Wallet Progress

```
┌─ Creating Vanity Wallet ───────────────────────────────────────────┐
│                                                                    │
│  Wallet Name: my_ai_wallet                                         │
│  Prefix: ai (case-insensitive)                                     │
│                                                                    │
│  ┌─ Progress ─────────────────────────────────────────────────┐   │
│  │                                                            │   │
│  │  Attempts:   1,234,567                                     │   │
│  │  Speed:      45,678 attempts/sec                           │   │
│  │  Elapsed:    27 seconds                                    │   │
│  │                                                            │   │
│  │  [████████████████░░░░░░░░░░░░░░░░] Searching...           │   │
│  │                                                            │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  Using 8 CPU threads for parallel generation                       │
│                                                                    │
│  Press Esc to cancel                                               │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
Status: Generating vanity wallet... This may take a few minutes
```

### 7. Delete Confirmation Dialog

```
┌─ Confirm Delete ───────────────────────────────────────────────────┐
│                                                                    │
│  ⚠️  WARNING ⚠️                                                     │
│                                                                    │
│  Are you sure you want to delete this wallet?                      │
│                                                                    │
│  Wallet: Main Wallet                                               │
│  Public Key: AbC1...XyZ9                                           │
│  Balance: 1.234 SOL                                                │
│                                                                    │
│  This action cannot be undone!                                     │
│  The private key will be permanently removed from storage.         │
│                                                                    │
│  ┌─────────────────┐  ┌─────────────────┐                         │
│  │ Press Y to      │  │ Press N or Esc  │                         │
│  │ Confirm Delete  │  │ to Cancel       │                         │
│  └─────────────────┘  └─────────────────┘                         │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
Status: Confirm deletion (Y/N)
```

### 8. Batch Operations Menu

```
┌─ Batch Operations ─────────────────────────────────────────────────┐
│                                                                    │
│  Select Operation:                                                 │
│                                                                    │
│  > Send SOL to Multiple Recipients                                 │
│    Send SPL Tokens to Multiple Recipients                          │
│    Token Mixing (Simulation)                                       │
│                                                                    │
│  [Feature currently in development]                                │
│                                                                    │
│  Use ↑/↓ to navigate, Enter to select, Esc to return              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
Status: Select a batch operation
```

## Status Bar Color Coding

The status bar uses color coding to indicate message type:

- 🔵 **Cyan (Info)**: Informational messages, tips, current mode
- 🟢 **Green (Success)**: Successful operations, confirmations
- 🟡 **Yellow (Warning)**: Warnings, cautions, important notices
- 🔴 **Red (Error)**: Errors, failures, critical issues

## Navigation Flow

```
┌─────────────────┐
│  Wallet List    │────────────────┐
│  (Main View)    │                │
└────┬────────────┘                │
     │                             │
     ├─ (Enter) ──> Wallet Detail  │
     │                             │
     ├─ (A) ──────> Add Wallet     │
     │                             │
     ├─ (V) ──────> Vanity Wallet  │
     │                             │
     ├─ (/) ──────> Search         │
     │                             │
     ├─ (D) ──────> Delete Confirm │
     │                             │
     ├─ (B) ──────> Batch Ops      │
     │                             │
     └─ (H) ──────> Help Screen ───┘
                         │
                    (Esc returns)
```

## Responsive Design

The TUI automatically adapts to terminal size:

- **Minimum Width**: 80 columns recommended
- **Minimum Height**: 24 rows recommended
- **Optimal**: 100x30 or larger

The UI uses flexible constraints:
- Title bar: Fixed 3 rows
- Main content: Fills available space
- Status bar: Fixed 3 rows

## Color Scheme

- **Primary**: Cyan for highlights and selections
- **Success**: Green for positive actions
- **Warning**: Yellow for cautions
- **Error**: Red for errors
- **Border**: Gray for UI elements
- **Text**: White/Light Gray for content

## Accessibility Features

- ✅ Keyboard-only navigation
- ✅ Clear visual feedback for selections
- ✅ Status messages for all actions
- ✅ Confirmation dialogs for destructive actions
- ✅ Help screen always accessible
- ⚠️ Screen reader support (needs testing)
- ⚠️ High contrast mode (not implemented)

## Performance Characteristics

- **Startup Time**: < 1 second (without network)
- **UI Response**: Instant (< 16ms frame time)
- **Wallet Loading**: O(n) where n = number of wallets
- **Search**: O(n) real-time filtering
- **Vanity Generation**: Variable (depends on prefix complexity)

## Security Features Visible in UI

1. **No Plaintext Keys**: Private keys never displayed
2. **Confirmation Dialogs**: For destructive operations
3. **Status Messages**: Never show sensitive data
4. **Secure Input**: No echo for sensitive inputs (future)
5. **Session Info**: Shows encryption status

---

**Note**: This documentation represents the current state of the TUI. 
Some features may be in development or require network connectivity.
