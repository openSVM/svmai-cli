# svmai CLI Tool Architecture Design

## 1. Overview

The `svmai` CLI tool will be a Rust application designed to help users manage Solana wallets. It will feature multi-threaded search for wallet files, validation of Solana private keys, secure storage of these keys using system keychain services, and a Text-based User Interface (TUI) for displaying balances and managing wallets. The tool will also support batch operations and token mixing functionalities.

## 2. Core Components

The application will be structured into several core modules:

### 2.1. `main` (CLI Entry Point)
   - Parses command-line arguments (e.g., search path, operation to perform).
   - Initializes other modules.
   - Launches the TUI or executes direct commands.
   - Handles top-level error management and logging.

### 2.2. `file_searcher` Module
   - **Responsibility**: Locates potential Solana wallet files (`.json`).
   - **Functionality**:
     - Takes a directory path as input.
     - Recursively searches for `.json` files.
     - Implements multi-threading for efficient searching in large directories.
     - Returns a list of found file paths.
   - **Dependencies**: Rust standard library for file system operations (`std::fs`, `std::path`), potentially `rayon` for easy parallelism.

### 2.3. `key_validator` Module
   - **Responsibility**: Validates if a given JSON file contains a Solana private key.
   - **Functionality**:
     - Takes a file path as input.
     - Reads and parses the JSON content (using `serde_json`).
     - Checks if the JSON structure matches the expected format for a Solana private key (an array of 64 numbers, typically u8).
     - Optionally, attempts to derive a public key to confirm validity (using `solana-sdk`).
   - **Dependencies**: `serde_json`, `solana-sdk`.

### 2.4. `secure_storage` Module
   - **Responsibility**: Manages the encrypted storage of validated private keys.
   - **Functionality**:
     - Uses the system's keychain (e.g., macOS Keychain, GNOME Keyring, Windows Credential Manager) via the `keyring-rs` crate.
     - Encrypts the collection of private keys before storing them in a local configuration file (e.g., `~/.config/svmai/config.enc`). The master key for this file encryption will be stored in the system keychain.
     - Provides functions to add, retrieve, update, and delete private keys from the secure store.
     - Handles keychain access permissions and errors gracefully.
   - **Dependencies**: `keyring-rs`, cryptographic libraries (e.g., `aes-gcm` or similar from `rust-crypto` or `ring`) for file encryption if `keyring-rs` only stores a master key.

### 2.5. `wallet_manager` Module
   - **Responsibility**: Provides high-level functions for managing wallets.
   - **Functionality**:
     - Interacts with `secure_storage` to access private keys.
     - Interacts with `solana_client` to fetch balances (SOL and SPL tokens).
     - Implements logic for batch sending of tokens.
     - Implements logic for token mixing operations (details to be further defined based on user needs for privacy/rebalancing).
     - Manages wallet metadata (e.g., user-defined aliases for wallets).
   - **Dependencies**: `secure_storage` module, `solana-sdk`, `solana-client` (or Moralis API via HTTP client if chosen for balance/token data).

### 2.6. `tui_interface` Module
   - **Responsibility**: Provides the Text-based User Interface.
   - **Functionality**:
     - Uses a TUI library like `ratatui`.
     - Displays a list of managed wallets and their balances.
     - Allows users to navigate and select operations (e.g., view details, send, mix).
     - Handles user input and updates the display accordingly.
     - Interacts with `wallet_manager` to perform actions and retrieve data.
   - **Dependencies**: `ratatui`, `wallet_manager` module.

### 2.7. `config_manager` Module
   - **Responsibility**: Manages application configuration (excluding sensitive private keys).
   - **Functionality**:
     - Handles settings like default search paths, TUI preferences, RPC endpoint for Solana.
     - Stores configuration in a plain text file (e.g., `~/.config/svmai/settings.toml`).
   - **Dependencies**: `serde` (for serialization/deserialization), `toml`.

### 2.8. `solana_interaction` Module (Potentially part of `wallet_manager` or separate)
   - **Responsibility**: Handles all direct communication with the Solana blockchain.
   - **Functionality**:
     - Connecting to a Solana RPC node (configurable).
     - Fetching SOL and SPL token balances for a given public key.
     - Constructing and sending transactions (e.g., for token transfers, batch sends).
     - Querying token metadata if needed.
   - **Dependencies**: `solana-sdk`, `solana-client`, `spl-token`.

## 3. Data Structures

### 3.1. `WalletInfo`
   - `public_key: String` (Base58 encoded Solana public key)
   - `private_key_ref: String` (Identifier or path to retrieve the actual private key from `secure_storage` - the private key itself is not stored here in plain text if this structure is persisted outside secure storage)
   - `alias: Option<String>` (User-defined name for the wallet)
   - `balance_sol: Option<u64>` (Lamports)
   - `token_balances: Option<Vec<TokenBalance>>`

### 3.2. `TokenBalance`
   - `mint_address: String`
   - `token_symbol: String`
   - `balance: u64` (Raw amount, considering decimals)
   - `decimals: u8`
   - `ui_balance: f64` (User-friendly balance)

### 3.3. `AppConfig` (for `config_manager`)
   - `rpc_url: String` (e.g., "https://api.mainnet-beta.solana.com")
   - `default_search_path: Option<String>`
   - `tui_theme: Option<String>`

### 3.4. `EncryptedWalletStore` (Conceptual structure for what `secure_storage` manages)
   - A collection (e.g., a `HashMap<PublicKeyString, EncryptedPrivateKeyString>`) of private keys, where each private key is encrypted. This entire collection would then be encrypted by a master key stored in the system keychain before being written to a file.

## 4. Workflow Examples

### 4.1. Initial Setup / Find Wallets
   1. User runs `svmai find /path/to/wallets`.
   2. `main` calls `file_searcher` to get all `.json` files.
   3. For each file, `main` calls `key_validator`.
   4. If valid, `main` (or `wallet_manager`) prompts user if they want to add it to `svmai`.
   5. If yes, `main` calls `secure_storage` to encrypt and save the private key, associating it with its public key.

### 4.2. View Balances (TUI)
   1. User runs `svmai` (no arguments, defaults to TUI).
   2. `main` launches `tui_interface`.
   3. `tui_interface` calls `wallet_manager` to get a list of `WalletInfo` (public keys and aliases).
   4. For each wallet, `wallet_manager` (using `solana_interaction`) fetches balances.
   5. `tui_interface` displays the wallets and balances.

### 4.3. Batch Send
   1. User selects "Batch Send" in TUI or uses a command like `svmai batch-send --from <wallet_alias_or_pubkey> --config <batch_send_config_file.csv>`.
   2. `tui_interface` or `main` collects parameters and calls `wallet_manager`.
   3. `wallet_manager` retrieves the private key for the source wallet from `secure_storage`.
   4. `wallet_manager` (using `solana_interaction`) constructs and sends the batch transactions.

## 5. Error Handling
   - Each module will return `Result<T, E>` for fallible operations.
   - Custom error types will be defined for different modules (e.g., `FileSearchError`, `KeyValidationError`, `SecureStorageError`, `SolanaRpcError`).
   - The `main` module and `tui_interface` will be responsible for presenting user-friendly error messages.
   - Logging (e.g., using the `log` crate with `env_logger` or similar) will be implemented for debugging.

## 6. Security Considerations
   - **Private Key Handling**: Private keys are the most sensitive data. They should only be held in memory in decrypted form for the shortest time necessary to sign a transaction. The `secure_storage` module is critical.
   - **Keychain Access**: The tool must clearly inform the user when it needs to access the keychain and why.
   - **Dependencies**: Audit third-party crates for security vulnerabilities, especially those dealing with cryptography and blockchain interactions.
   - **Input Validation**: All user inputs (paths, addresses, amounts) must be rigorously validated.

This architectural design provides a high-level overview. Details for each module and their interactions will be further refined during the implementation phase.

