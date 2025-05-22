// src/config.rs

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use dirs;

/// Configuration structure for the svmai CLI tool
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// General application settings
    pub general: GeneralConfig,
    
    /// Search-related settings
    pub search: SearchConfig,
    
    /// Wallet management settings
    pub wallet: WalletConfig,
    
    /// Vanity wallet generation settings
    pub vanity: VanityConfig,
    
    /// Logging settings
    pub logging: LoggingConfig,
}

/// General application settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    /// Default mode to start in (tui, cli)
    pub default_mode: String,
}

/// Search-related settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchConfig {
    /// Maximum directory depth to search
    pub max_depth: usize,
    
    /// Maximum number of files to find
    pub max_files: usize,
    
    /// Batch size for parallel processing
    pub batch_size: usize,
}

/// Wallet management settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletConfig {
    /// Default wallet name prefix
    pub default_name_prefix: String,
    
    /// Keychain service name
    pub keychain_service_name: String,
    
    /// Data directory for wallet files
    pub data_dir: String,
}

/// Vanity wallet generation settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VanityConfig {
    /// Default vanity address prefix
    pub default_prefix: String,
    
    /// Whether prefix matching is case-sensitive
    pub case_sensitive: bool,
    
    /// Default timeout in seconds
    pub timeout_seconds: u64,
    
    /// Maximum number of threads to use (0 = auto)
    pub max_threads: usize,
    
    /// Progress update interval in milliseconds
    pub progress_update_ms: u64,
}

/// Logging settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    
    /// Whether to log to file
    pub log_to_file: bool,
    
    /// Log file path
    pub log_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: GeneralConfig {
                default_mode: "tui".to_string(),
            },
            search: SearchConfig {
                max_depth: 10,
                max_files: 100,
                batch_size: 50,
            },
            wallet: WalletConfig {
                default_name_prefix: "wallet_".to_string(),
                keychain_service_name: "svmai_cli_tool".to_string(),
                data_dir: get_default_data_dir().to_string_lossy().to_string(),
            },
            vanity: VanityConfig {
                default_prefix: "ai".to_string(),
                case_sensitive: false,
                timeout_seconds: 120,
                max_threads: 0, // 0 means auto-detect
                progress_update_ms: 500,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                log_to_file: true,
                log_file: get_default_log_file().to_string_lossy().to_string(),
            },
        }
    }
}

/// Get the default configuration file path
pub fn get_config_path() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join("svmai").join("config.toml");
        return config_path;
    }
    PathBuf::from("./config.toml")
}

/// Get the default data directory
pub fn get_default_data_dir() -> PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        return data_dir.join("svmai");
    }
    PathBuf::from("./data")
}

/// Get the default log file path
pub fn get_default_log_file() -> PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        return data_dir.join("svmai").join("svmai.log");
    }
    PathBuf::from("./svmai.log")
}

/// Load configuration from the default location or create if it doesn't exist
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();
    
    // If config file doesn't exist, create it with defaults
    if !config_path.exists() {
        return create_default_config().context("Failed to create default configuration");
    }
    
    // Read and parse the config file
    let config_str = fs::read_to_string(&config_path)
        .context(format!("Failed to read config file: {:?}", config_path))?;
    
    let config: Config = toml::from_str(&config_str)
        .context("Failed to parse config file")?;
    
    Ok(config)
}

/// Create default configuration file
pub fn create_default_config() -> Result<Config> {
    let config = Config::default();
    let config_path = get_config_path();
    
    // Create parent directories if they don't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create config directory: {:?}", parent))?;
    }
    
    // Serialize config to TOML
    let config_str = toml::to_string_pretty(&config)
        .context("Failed to serialize config")?;
    
    // Write config to file
    fs::write(&config_path, config_str)
        .context(format!("Failed to write config file: {:?}", config_path))?;
    
    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    
    // Create parent directories if they don't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create config directory: {:?}", parent))?;
    }
    
    // Serialize config to TOML
    let config_str = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;
    
    // Write config to file
    fs::write(&config_path, config_str)
        .context(format!("Failed to write config file: {:?}", config_path))?;
    
    Ok(())
}

/// Load configuration from a specific file
pub fn load_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
    let config_str = fs::read_to_string(path.as_ref())
        .context(format!("Failed to read config file: {:?}", path.as_ref()))?;
    
    let config: Config = toml::from_str(&config_str)
        .context("Failed to parse config file")?;
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.default_mode, "tui");
        assert_eq!(config.vanity.default_prefix, "ai");
    }
    
    #[test]
    fn test_serialize_deserialize() {
        let config = Config::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        
        assert_eq!(config.general.default_mode, deserialized.general.default_mode);
        assert_eq!(config.vanity.default_prefix, deserialized.vanity.default_prefix);
    }
    
    #[test]
    fn test_save_load_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let mut config = Config::default();
        config.general.default_mode = "cli".to_string();
        
        // Serialize and write to file
        let config_str = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, config_str).unwrap();
        
        // Read and deserialize
        let loaded_config = load_config_from_file(&config_path).unwrap();
        
        assert_eq!(config.general.default_mode, loaded_config.general.default_mode);
    }
}
