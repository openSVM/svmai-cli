# svmai - Solana Wallet Management CLI

`svmai` is a command-line interface (CLI) tool built with Rust for managing Solana wallets. It allows users to find Solana wallet files, securely store their private keys using system keychain encryption, manage multiple wallets, view balances, and perform batch operations and token mixing for enhanced privacy.

## Features

*   **Multi-threaded Wallet Search**: Efficiently scans specified folders for `.json` files that appear to be Solana wallets, utilizing parallel processing for optimal performance even with large directories.
*   **Solana Private Key Validation**: Confirms that identified JSON files contain valid Solana private keys.
*   **Secure Keychain Storage**: Encrypts and stores private keys using the user's system keychain (e.g., macOS Keychain, GNOME Keyring), requiring user authentication for decryption.
*   **Wallet Management**: 
    *   List stored wallets with public key previews and balances.
    *   Search and filter wallets by name.
    *   Add new wallets by importing key files.
    *   Remove wallets from secure storage with confirmation.
*   **Vanity Wallet Creation**: Generate new wallets with addresses that start with a specific prefix (default: "ai"), with real-time progress feedback and cancellation support.
*   **Enhanced Text-based User Interface (TUI)**: Provides an intuitive, feature-rich TUI for managing wallets and viewing information. Built with `ratatui`.
*   **Detailed Wallet Information**: View comprehensive wallet details including:
    *   SOL balance
    *   SPL token balances with token names and mint addresses
    *   Transaction history preview
*   **Batch Operations**: Allows users to send SOL or SPL tokens to multiple recipients in a batch.
*   **Token Mixing (Simulation)**: Provides a simulated token mixing feature to demonstrate how tokens could be moved between wallets to enhance privacy. *Note: The current implementation is a simulation and does not execute real transactions for mixing.*

## Installation

### Prerequisites

*   **Rust and Cargo**: Ensure you have Rust and Cargo installed. You can install them from [https://rustup.rs/](https://rustup.rs/).
*   **Build Essentials**: A C compiler and related tools are needed for some Rust dependencies.
    *   On Debian/Ubuntu: `sudo apt-get update && sudo apt-get install build-essential`
*   **OpenSSL Development Libraries**:
    *   On Debian/Ubuntu: `sudo apt-get install libssl-dev`
*   **Keychain Service**: Your operating system must have a keychain service available (e.g., macOS Keychain, GNOME Keyring with `libsecret`, Windows Credential Manager).

### Building from Source

1.  Clone the repository (or download the source code).
2.  Navigate to the project directory: `cd svmai`
3.  Build the project: `cargo build --release`
4.  The executable will be located at `target/release/svmai`.

## Usage

Once built, you can run `svmai` from your terminal.

```bash
./target/release/svmai
```

This will launch the Text-based User Interface (TUI).

### TUI Navigation and Keyboard Shortcuts

#### General Navigation
*   **Arrow Keys (↑/↓)**: Navigate through lists (e.g., wallet list)
*   **Enter**: Select item or confirm action
*   **Esc/Backspace**: Return to previous view
*   **Q**: Quit the application

#### Wallet Management
*   **A**: Add a new wallet
*   **V**: Create a new vanity wallet with "ai" prefix
*   **D**: Delete the selected wallet (with confirmation)
*   **R**: Refresh wallet list and balance information
*   **/**: Search wallets by name
*   **B**: Access batch operations menu

#### Help and Information
*   **H**: Show help screen with all available commands

### Core Functionalities

#### 1. Wallet List View

The main view displays your stored wallets with the following information:
- Wallet name
- Public key preview (first and last 4 characters)
- SOL balance

From this view, you can:
- Navigate between wallets
- Search for specific wallets using the `/` key
- Add new wallets with the `A` key
- Create vanity wallets with the `V` key
- Remove wallets with the `D` key
- View detailed wallet information by selecting a wallet and pressing `Enter`

#### 2. Wallet Detail View

This view shows comprehensive information about a selected wallet:
- Wallet name
- Full public key
- SOL balance
- SPL token balances (token name, amount, and mint address)
- Last transaction information

#### 3. Search Functionality

The search feature allows you to quickly find wallets by name:
1. Press `/` from the wallet list view
2. Type your search query
3. Press `Enter` to apply the search filter
4. Press `Esc` to clear the search and return to the full wallet list

#### 4. Adding Wallets

To add a new wallet:
1. Press `A` from the wallet list view
2. Enter the full path to the Solana wallet JSON file
3. Press `Enter` to confirm
4. The wallet will be validated, encrypted, and added to your secure storage

#### 5. Creating Vanity Wallets

To create a new wallet with an address that starts with "ai":
1. Press `V` from the wallet list view
2. Enter a name for the new wallet (default: "ai_wallet")
3. The prefix is fixed to "ai" as requested
4. Press `Enter` to start the generation process
5. A progress screen will show:
   - Number of attempts made
   - Speed (attempts per second)
   - Elapsed time
   - Visual progress indicator
6. You can press `Esc` at any time to cancel the process
7. When a matching address is found, the wallet will be automatically saved and added to your list

**Note on Vanity Address Generation:**
- Finding a vanity address is a probabilistic process that involves generating random keypairs until one with the desired prefix is found
- The time required depends on the prefix length and complexity
- For a 2-character prefix like "ai", the process typically takes a few minutes
- The tool uses multiple CPU threads (up to 8) to accelerate the search
- A timeout of 2 minutes is set by default, after which the process will stop if no matching address is found

#### 6. Batch Operations

Access batch operations by pressing `B` from the wallet list or detail view. This feature allows you to:
- Send SOL or SPL tokens to multiple recipients in a single operation
- Specify different amounts for each recipient
- Review and confirm the batch transaction before execution

#### 7. Token Mixing (Simulation)

This feature demonstrates how tokens could be moved between wallets to enhance privacy:
1. Select source wallets
2. Specify destination addresses
3. Set the total amount to mix
4. Define the number of intermediate steps
5. Review the simulated mixing plan

*Note: This is a simulation and does not execute real transactions on the network.*

### Security Considerations

*   **Private Key Encryption**: Your private keys are encrypted using AES-GCM. The encryption key itself is managed by your operating system's keychain service. Accessing this key typically requires your user password or biometric authentication, providing a strong layer of security.
*   **Keychain Access**: The tool will request permission to access the keychain service when it first needs to store or retrieve the master encryption key. Ensure you grant permission only if you trust the application.
*   **Transaction Signing**: All transactions that involve spending funds (e.g., batch sends) require the decrypted private key for signing. This decryption happens locally after you authenticate with your keychain.
*   **Moralis API Key**: The application uses a Moralis API key to fetch balance information. This key is embedded in the application for ease of use in this context. For production deployments or wider distribution, consider managing API keys more securely (e.g., user-provided or via a backend service).
*   **Vanity Wallet Security**: Vanity addresses provide a cosmetic benefit but do not increase security. The generated keypairs use the same cryptographic strength as regular Solana keypairs.

## Development

This section is for developers looking to contribute or understand the codebase.

### Project Structure

*   `src/main.rs`: Main application entry point, argument parsing, and TUI launch.
*   `src/tui.rs`: Handles the Text-based User Interface logic using `ratatui`.
*   `src/file_searcher.rs`: Implements the multi-threaded search for JSON wallet files.
*   `src/key_validator.rs`: Validates if a JSON file contains a Solana private key.
*   `src/secure_storage.rs`: Manages encryption/decryption of private keys and interaction with the system keychain.
*   `src/wallet_manager.rs`: Handles CRUD operations for wallets (adding, listing, removing, retrieving keypairs).
*   `src/transaction_handler.rs`: Implements batch sending and token mixing logic.
*   `src/vanity_wallet.rs`: Implements vanity address generation with multi-threading and progress reporting.
*   `Cargo.toml`: Project dependencies and metadata.

### TUI Implementation Details

The TUI is built using the `ratatui` crate with `crossterm` for terminal control. Key components include:

*   **View System**: The interface is organized into different views (WalletList, WalletDetail, Help, etc.) with dedicated rendering and key handling functions for each.
*   **App State**: The `App` struct maintains application state, including wallet information, selected indices, and user input.
*   **Event Loop**: The main loop handles drawing the UI and processing user input events.
*   **Responsive Layout**: The UI adapts to different terminal sizes using flexible constraints.

### Multi-threaded File Search Implementation

The file search functionality uses:

*   **Rayon**: For parallel processing of file system entries.
*   **WalkDir**: For efficient recursive directory traversal.
*   **Batched Processing**: Files are processed in configurable batch sizes for optimal performance.
*   **Early Exit**: Search can be configured to stop after finding a specified number of files.
*   **Depth Limiting**: Search depth can be limited to avoid excessive traversal.

### Vanity Wallet Implementation

The vanity wallet generation feature uses:

*   **Multi-threading**: Utilizes Rayon for parallel keypair generation across multiple CPU cores.
*   **Thread-safe Communication**: Uses atomic types and thread-safe containers for reliable inter-thread communication.
*   **Progress Reporting**: Provides real-time feedback on attempts, speed, and elapsed time.
*   **Cancellation Support**: Allows graceful cancellation of the generation process at any time.
*   **Case-insensitive Matching**: Matches prefixes regardless of case for greater flexibility.
*   **Configurable Parameters**: Allows customization of timeout, thread count, and progress update frequency.

### Dependencies

Key dependencies include:

*   `clap`: For command-line argument parsing (if CLI mode is expanded beyond TUI-only).
*   `ratatui` & `crossterm`: For the Text-based User Interface.
*   `rayon`: For parallel processing (e.g., in file searching and vanity address generation).
*   `serde` & `serde_json`: For JSON serialization and deserialization.
*   `solana-sdk`: For Solana-specific data structures and utilities.
*   `spl-token`: For SPL token utilities.
*   `keyring`: For interacting with system keychains.
*   `aes-gcm`: For symmetric encryption of private keys.
*   `walkdir`: For recursive directory traversal.
*   `reqwest`: For making HTTP requests to the Moralis API (or Solana RPC).
*   `tokio`: For asynchronous operations, especially for network requests.
*   `num_cpus`: For detecting the number of CPU cores for optimal thread allocation.

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Error Handling

The application uses a combination of:
- `anyhow` for flexible error handling and propagation
- Custom error types for domain-specific errors
- Consistent error reporting in the TUI status bar

### Performance Considerations

- The file search module is optimized for performance with large directories
- Wallet operations are designed to be efficient even with many wallets
- API calls for balance checking are rate-limited to avoid service disruptions
- Vanity wallet generation is optimized to use multiple CPU cores while avoiding excessive resource consumption

## Troubleshooting

*   **Build Errors related to `cc` or linker not found**: Ensure `build-essential` (or equivalent for your OS) is installed.
*   **Build Errors related to OpenSSL**: Ensure `libssl-dev` (or equivalent) is installed.
*   **Keychain Errors**: 
    *   Ensure your OS has a running keychain service.
    *   On Linux, you might need to have `libsecret-1-dev` installed and a service like GNOME Keyring or KeePassXC running and configured.
    *   The first time the application tries to access the keychain, your OS should prompt you for permission. Make sure to allow access.
*   **Moralis API Rate Limits**: If balance checking fails frequently, you might be hitting API rate limits. The embedded API key is for general use; for heavy usage, you might need your own key.
*   **TUI Display Issues**: If the TUI appears corrupted or doesn't render correctly:
    *   Ensure your terminal supports Unicode and colors
    *   Try resizing your terminal window
    *   Check if your terminal multiplexer (if using one) is compatible with the TUI library
*   **Vanity Wallet Generation Performance**: If vanity wallet generation seems slow:
    *   Check CPU usage to ensure multiple cores are being utilized
    *   For longer prefixes (more than 3 characters), expect significantly longer search times
    *   Consider increasing the timeout for more complex prefixes

## Disclaimer

This tool interacts with Solana private keys and can perform transactions on the Solana network that involve real cryptocurrency. **Use it at your own risk.** The authors are not responsible for any loss of funds. Always ensure you understand what the tool is doing, especially when authorizing transactions or managing private keys. For the token mixing feature, the current implementation is a **simulation** and does not execute real mixing transactions on the network to avoid accidental fund transfers during this demonstration phase.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details (to be created).
