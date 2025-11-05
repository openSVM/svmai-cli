// wallet_manager.rs

use crate::key_validator;
use crate::secure_storage::{self, SecureStorageError};
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path; // To validate a key from a file before adding

/// Adds a new wallet by reading a private key from a JSON file and storing it securely.
/// The wallet will be stored under the given `wallet_name`.
pub fn add_wallet_from_file(wallet_name: &str, key_file_path: &str) -> io::Result<()> {
    println!(
        "[wallet_manager] Attempting to add wallet 	{}	 from file: {}",
        wallet_name, key_file_path
    );

    if !Path::new(key_file_path).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Key file not found: {}", key_file_path),
        ));
    }

    // 1. Validate if the file is a Solana wallet JSON
    match key_validator::is_solana_wallet_json_file(key_file_path) {
        Ok(true) => {
            // 2. Read the raw key bytes from the file (assuming it's a JSON array of u8)
            // The is_solana_wallet_json_file already does a good job of parsing and validating structure.
            // We need to extract the actual key bytes here.
            let contents = fs::read_to_string(key_file_path)?;
            let parsed_json: serde_json::Value = serde_json::from_str(&contents).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to parse JSON: {}", e),
                )
            })?;

            if let serde_json::Value::Array(arr) = parsed_json {
                if arr.len() == 64 {
                    let mut key_bytes: Vec<u8> = Vec::with_capacity(64);
                    for val in arr {
                        if let serde_json::Value::Number(num) = val {
                            if let Some(byte_val) = num.as_u64() {
                                if byte_val <= u8::MAX as u64 {
                                    key_bytes.push(byte_val as u8);
                                } else {
                                    return Err(Error::new(
                                        ErrorKind::InvalidData,
                                        "Key byte value out of u8 range",
                                    ));
                                }
                            } else {
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    "Key byte value not a valid u64",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Key array element not a number",
                            ));
                        }
                    }
                    // 3. Store the validated key bytes securely
                    secure_storage::store_private_key(wallet_name, &key_bytes)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                    println!(
                        "[wallet_manager] Wallet 	{}	 added successfully from {}.",
                        wallet_name, key_file_path
                    );
                    Ok(())
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "JSON key array does not contain 64 elements.",
                    ))
                }
            } else {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "Validated JSON key is not an array as expected.",
                ))
            }
        }
        Ok(false) => Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "File 	{}	 is not a valid Solana wallet key file.",
                key_file_path
            ),
        )),
        Err(e) => Err(Error::new(
            ErrorKind::Other,
            format!("Error validating key file {}: {}", key_file_path, e),
        )),
    }
}

/// Lists the names of all securely stored wallets.
pub fn list_wallets() -> io::Result<()> {
    println!("[wallet_manager] Listing all stored wallets...");
    match secure_storage::list_wallet_names()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    {
        Ok(names) => {
            if names.is_empty() {
                println!("No wallets are currently stored.");
            } else {
                println!("Stored wallets:");
                for name in names {
                    println!("- {}", name);
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error listing wallets: {}", e);
            Err(e)
        }
    }
}

/// Removes a wallet with the given `wallet_name` from secure storage.
pub fn remove_wallet(wallet_name: &str) -> io::Result<()> {
    println!(
        "[wallet_manager] Attempting to remove wallet: {}",
        wallet_name
    );
    // First, check if wallet exists to provide better feedback
    let wallets = secure_storage::list_wallet_names()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    if !wallets.contains(&wallet_name.to_string()) {
        println!("Wallet 	{}	 not found.", wallet_name);
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Wallet 	{}	 not found.", wallet_name),
        ));
    }

    match secure_storage::remove_private_key(wallet_name)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    {
        Ok(_) => {
            println!("Wallet 	{}	 removed successfully.", wallet_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error removing wallet 	{}	: {}", wallet_name, e);
            Err(e)
        }
    }
}

// Placeholder for future use, e.g., by transaction module
pub fn get_wallet_keypair(
    wallet_name: &str,
) -> io::Result<Option<solana_sdk::signer::keypair::Keypair>> {
    match secure_storage::retrieve_private_key(wallet_name)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    {
        Some(key_bytes) => {
            // new_from_array expects only the 32-byte secret key, not the full 64-byte keypair
            // Convert Vec<u8> to [u8; 32] array
            if key_bytes.len() != 64 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid key length: expected 64 bytes, got {}", key_bytes.len())
                ));
            }
            let mut secret_key = [0u8; 32];
            secret_key.copy_from_slice(&key_bytes[0..32]);
            let keypair = solana_sdk::signer::keypair::Keypair::new_from_array(secret_key);
            Ok(Some(keypair))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secure_storage; // For direct interaction for setup/teardown if needed
    use solana_sdk::signer::keypair::Keypair;
    use solana_sdk::signer::Signer;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    // Helper to create a dummy Solana key JSON file
    fn create_dummy_key_file(
        dir: &tempfile::TempDir,
        filename: &str,
        keypair_opt: Option<&Keypair>,
    ) -> String {
        let file_path = dir.path().join(filename);
        let mut file = File::create(&file_path).unwrap();
        let key_to_write = match keypair_opt {
            Some(kp) => kp.to_bytes(),
            None => [0u8; 64], // Default invalid key for some tests
        };
        // Solana CLI stores as a simple JSON array of numbers
        let json_array_string = format!(
            "[{}]
",
            key_to_write
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        writeln!(file, "{}", json_array_string).unwrap();
        file_path.to_str().unwrap().to_string()
    }

    fn setup_test_env() -> (tempfile::TempDir, String) {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_str().unwrap();

        // Use a unique timestamp for this test run
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // Set a unique test service name for keychain isolation
        let test_service_name = format!("svmai_test_{}", timestamp);
        println!(
            "[test_debug] Setting test service name: {}",
            &test_service_name
        );
        env::set_var("SVMAI_TEST_SERVICE_NAME", &test_service_name);

        // Create a unique config file path within the temp directory
        let test_config_path = temp_dir
            .path()
            .join(format!(".svmai_wallets_{}.json", timestamp));
        let test_config_path_str = test_config_path.to_str().unwrap();
        println!(
            "[test_debug] Setting test config path: {}",
            test_config_path_str
        );
        env::set_var("SVMAI_TEST_CONFIG_PATH", test_config_path_str);

        // Set HOME to the temp directory
        env::set_var("HOME", temp_dir_path);

        // Clear any existing test key from keychain for a clean run
        if let Ok(entry) = keyring::Entry::new(
            &test_service_name,
            secure_storage::KEYCHAIN_MASTER_KEY_ACCOUNT_NAME,
        ) {
            let _ = entry.delete_credential();
            println!("[test_debug] Cleared existing keychain entry");
        }

        (temp_dir, test_service_name)
    }

    fn teardown_test_env(temp_dir: tempfile::TempDir, test_service_name: &str) {
        println!("[test_debug] Tearing down test environment");

        // Clean up the keychain entry with the test-specific service name
        if let Ok(entry) = keyring::Entry::new(
            test_service_name,
            secure_storage::KEYCHAIN_MASTER_KEY_ACCOUNT_NAME,
        ) {
            let _ = entry.delete_credential();
            println!("[test_debug] Deleted test keychain entry");
        }

        // Clean up environment variables
        env::remove_var("SVMAI_TEST_SERVICE_NAME");
        env::remove_var("SVMAI_TEST_CONFIG_PATH");
        env::remove_var("HOME");

        // Clean up config file
        if let Ok(config_path) = secure_storage::get_config_path() {
            let _ = fs::remove_file(&config_path);
            println!("[test_debug] Removed config file: {:?}", config_path);
        }

        // Drop temp directory
        drop(temp_dir);
        println!("[test_debug] Test environment cleanup complete");
    }

    #[test]
    fn test_add_list_remove_wallet() {
        // Setup with unique environment
        let (temp_dir, test_service_name) = setup_test_env();

        // Create test wallet
        let wallet1_name = "test_wallet1";
        let keypair1 = Keypair::new();
        let pubkey1 = keypair1.pubkey();
        println!("[test_debug] Created test keypair with pubkey: {}", pubkey1);

        let key_file1_path = create_dummy_key_file(&temp_dir, "wallet1.json", Some(&keypair1));

        // Verify environment variables are set correctly
        println!(
            "[test_debug] SVMAI_TEST_SERVICE_NAME: {:?}",
            std::env::var("SVMAI_TEST_SERVICE_NAME")
        );
        println!(
            "[test_debug] SVMAI_TEST_CONFIG_PATH: {:?}",
            std::env::var("SVMAI_TEST_CONFIG_PATH")
        );

        // Add wallet
        assert!(add_wallet_from_file(wallet1_name, &key_file1_path).is_ok());

        // List wallets
        let names = secure_storage::list_wallet_names().unwrap();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], wallet1_name);

        // Retrieve and check keypair
        let retrieved_kp_result = get_wallet_keypair(wallet1_name);
        assert!(retrieved_kp_result.is_ok());

        let retrieved_kp_option = retrieved_kp_result.unwrap();
        assert!(retrieved_kp_option.is_some());

        let retrieved_kp = retrieved_kp_option.unwrap();
        let retrieved_pubkey = retrieved_kp.pubkey();

        println!("[test_debug] Original pubkey: {}", pubkey1);
        println!("[test_debug] Retrieved pubkey: {}", retrieved_pubkey);

        assert_eq!(retrieved_pubkey, pubkey1);

        // Remove wallet
        assert!(remove_wallet(wallet1_name).is_ok());

        // List wallets after removal
        let names_after_remove = secure_storage::list_wallet_names().unwrap();
        assert!(
            names_after_remove.is_empty(),
            "Wallet list should be empty after removal"
        );

        // Try to retrieve removed wallet - should return None
        let result = get_wallet_keypair(wallet1_name);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        // Cleanup
        teardown_test_env(temp_dir, &test_service_name);
    }

    #[test]
    fn test_add_wallet_invalid_file_path() {
        let (temp_dir, test_service_name) = setup_test_env();
        assert!(add_wallet_from_file("w1", "non_existent_file.json").is_err());
        teardown_test_env(temp_dir, &test_service_name);
    }

    #[test]
    fn test_add_wallet_not_solana_key_file() {
        let (temp_dir, test_service_name) = setup_test_env();
        let not_a_key_file_path = temp_dir.path().join("not_a_key.json");
        let mut file = File::create(&not_a_key_file_path).unwrap();
        writeln!(file, "{{\"message\": \"this is not a key\"}}").unwrap();

        assert!(add_wallet_from_file("w_invalid", not_a_key_file_path.to_str().unwrap()).is_err());
        teardown_test_env(temp_dir, &test_service_name);
    }

    #[test]
    fn test_remove_non_existent_wallet() {
        let (temp_dir, test_service_name) = setup_test_env();
        assert!(remove_wallet("ghost_wallet").is_err());
        teardown_test_env(temp_dir, &test_service_name);
    }
}
