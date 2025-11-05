use serde_json::Value;
use solana_sdk::signer::keypair::Keypair;
use std::fs;
use std::fs::File;
use std::io;

/// Validates if the content of a given JSON file represents a Solana private key.
/// A Solana private key is typically represented as a JSON array of 64 u8 values.
pub fn is_solana_wallet_json_file(file_path: &str) -> io::Result<bool> {
    let contents = fs::read_to_string(file_path)?;

    // Attempt to parse the string as JSON
    let parsed_json: Result<Value, _> = serde_json::from_str(&contents);

    match parsed_json {
        Ok(Value::Array(arr)) => {
            // Check if the array contains 64 numbers (u8 values for a private key)
            if arr.len() == 64 {
                let mut key_bytes: Vec<u8> = Vec::with_capacity(64);
                for val in arr {
                    if let Value::Number(num) = val {
                        if let Some(byte_val) = num.as_u64() {
                            if byte_val <= u8::MAX as u64 {
                                key_bytes.push(byte_val as u8);
                            } else {
                                return Ok(false); // Number out of u8 range
                            }
                        } else {
                            return Ok(false); // Not a valid u64 number
                        }
                    } else {
                        return Ok(false); // Element is not a number
                    }
                }
                // If we successfully collected 64 bytes, try to create a Keypair from it.
                // This is the definitive check for a valid Solana secret key.
                // new_from_array expects only the 32-byte secret key
                let mut secret_key = [0u8; 32];
                secret_key.copy_from_slice(&key_bytes[0..32]);
                let _keypair = Keypair::new_from_array(secret_key);
                // Successfully created a keypair, this is a valid Solana secret key
                Ok(true)
            } else {
                Ok(false) // Array length is not 64
            }
        }
        Ok(_) => Ok(false), // JSON is valid, but not an array (which is expected for Solana keys)
        Err(_) => Ok(false), // Failed to parse as JSON
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signer::Signer;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_valid_solana_key_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("valid_wallet.json");
        let keypair = Keypair::new();
        let keypair_bytes_array = keypair.to_bytes();
        let secret_key_json = format!(
            "[{}]
",
            keypair_bytes_array
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", secret_key_json).unwrap();

        assert!(is_solana_wallet_json_file(file_path.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_invalid_json_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid_format.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{{\"key\": \"not an array\"}}").unwrap();
        assert!(!is_solana_wallet_json_file(file_path.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_json_array_wrong_length() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("wrong_length.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "[1, 2, 3, 4, 5]").unwrap(); // Array of 5, not 64
        assert!(!is_solana_wallet_json_file(file_path.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_json_array_non_numeric() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("non_numeric.json");
        let mut file = File::create(&file_path).unwrap();
        let mut content = String::from("[");
        for i in 0..63 {
            content.push_str(&format!("{}, ", i));
        }
        content.push_str("\"not_a_number\"]"); // Last element is a string
        writeln!(file, "{}", content).unwrap();
        assert!(!is_solana_wallet_json_file(file_path.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_json_array_out_of_u8_range() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("out_of_range.json");
        let mut file = File::create(&file_path).unwrap();
        let mut content = String::from("[");
        for i in 0..63 {
            content.push_str(&format!("{}, ", i));
        }
        content.push_str("256]"); // 256 is out of u8 range
        writeln!(file, "{}", content).unwrap();
        assert!(!is_solana_wallet_json_file(file_path.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_not_a_json_file_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("not_json.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "This is not JSON content at all.").unwrap();
        assert!(!is_solana_wallet_json_file(file_path.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_empty_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.json");
        File::create(&file_path).unwrap();
        assert!(!is_solana_wallet_json_file(file_path.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_file_not_found() {
        let result = is_solana_wallet_json_file("non_existent_file.json");
        assert!(result.is_err());
    }
}
