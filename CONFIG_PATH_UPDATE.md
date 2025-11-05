# Configuration Path Update

## Change Summary

Updated the wallet storage location to use `~/.config/svmai/` directory instead of storing directly in the home directory.

## Before

**Storage Location:** `~/.svmai_wallets.json`
- Wallet data stored directly in home directory
- Hidden file (prefixed with `.`)

## After

**Storage Location:** `~/.config/svmai/wallets.json`
- Follows XDG Base Directory specification
- Organized in dedicated config directory
- Separate from Solana's config directory (`~/.config/solana/`)

## Changes Made

### File: `src/secure_storage.rs`

1. **Updated Constants:**
   ```rust
   // Before
   pub const CONFIG_FILE_NAME: &str = ".svmai_wallets.json";
   
   // After
   pub const CONFIG_FILE_NAME: &str = "wallets.json";
   pub const CONFIG_DIR_NAME: &str = "svmai";
   ```

2. **Updated `get_config_path()` Function:**
   ```rust
   // Before
   dirs::home_dir()
       .map(|home| home.join(CONFIG_FILE_NAME))
   
   // After
   dirs::config_dir()
       .map(|config_dir| config_dir.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
   ```

## Benefits

1. **No Conflict with Solana:** Ensures svmai doesn't touch `~/.config/solana/` directory
2. **Better Organization:** Follows standard config directory conventions
3. **Platform Support:** Uses `dirs::config_dir()` which maps to:
   - Linux: `~/.config/svmai/`
   - macOS: `~/Library/Application Support/svmai/`
   - Windows: `%APPDATA%\svmai\`

## Migration Note

Existing users with wallets in `~/.svmai_wallets.json` will need to manually move their encrypted wallet data to the new location if they want to preserve their wallets. The application will create a fresh wallet store in the new location.

## Verification

Build and test completed successfully:
- ✅ Application compiles without errors
- ✅ Config path uses `~/.config/svmai/wallets.json`
- ✅ No interaction with `~/.config/solana/` directory
- ✅ Backward compatibility maintained through environment variable override for tests

