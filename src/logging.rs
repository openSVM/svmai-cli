// src/logging.rs

use log::{LevelFilter, Record, Level, Metadata, SetLoggerError};
use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::fs;
use anyhow::{Result, Context};
use crate::config;

/// Custom logger implementation
struct SvmaiLogger {
    level: LevelFilter,
    file: Option<Mutex<File>>,
}

impl log::Log for SvmaiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let level_str = match record.level() {
                Level::Error => "ERROR",
                Level::Warn => "WARN ",
                Level::Info => "INFO ",
                Level::Debug => "DEBUG",
                Level::Trace => "TRACE",
            };
            
            let log_message = format!(
                "[{}] {} [{}:{}] {}\n",
                now,
                level_str,
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            );
            
            // Print to stderr
            eprintln!("{}", log_message.trim());
            
            // Write to file if configured
            if let Some(file) = &self.file {
                if let Ok(mut file) = file.lock() {
                    let _ = file.write_all(log_message.as_bytes());
                    let _ = file.flush();
                }
            }
        }
    }

    fn flush(&self) {
        if let Some(file) = &self.file {
            if let Ok(mut file) = file.lock() {
                let _ = file.flush();
            }
        }
    }
}

/// Initialize the logger with configuration
pub fn init_logger() -> Result<(), SetLoggerError> {
    // Load configuration
    let config = config::load_config().unwrap_or_else(|_| config::Config::default());
    
    // Determine log level
    let level = match config.logging.level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    
    // Set up file logging if enabled
    let file = if config.logging.log_to_file {
        let log_path = Path::new(&config.logging.log_file);
        
        // Create parent directories if they don't exist
        if let Some(parent) = log_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        // Open log file
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path) {
                Ok(file) => Some(Mutex::new(file)),
                Err(e) => {
                    eprintln!("Warning: Could not open log file: {}", e);
                    None
                }
            }
    } else {
        None
    };
    
    // Create and set the logger
    let logger = Box::new(SvmaiLogger {
        level,
        file,
    });
    
    log::set_boxed_logger(logger)?;
    log::set_max_level(level);
    
    Ok(())
}

/// Log a message at the specified level
pub fn log_message(level: Level, message: &str) {
    match level {
        Level::Error => log::error!("{}", message),
        Level::Warn => log::warn!("{}", message),
        Level::Info => log::info!("{}", message),
        Level::Debug => log::debug!("{}", message),
        Level::Trace => log::trace!("{}", message),
    }
}

/// Helper function to log errors with context
pub fn log_error<E: std::fmt::Display>(error: E, context: &str) {
    log::error!("{}: {}", context, error);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Read;
    
    #[test]
    fn test_logger_file_output() {
        let temp_dir = tempdir().unwrap();
        let log_path = temp_dir.path().join("test.log");
        
        // Create a logger with file output
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .unwrap();
        
        let logger = SvmaiLogger {
            level: LevelFilter::Debug,
            file: Some(Mutex::new(file)),
        };
        
        // Log a test message
        let record = log::Record::builder()
            .args(format_args!("Test message"))
            .level(Level::Info)
            .module_path(Some("test_module"))
            .line(Some(42))
            .build();
        
        logger.log(&record);
        
        // Read the log file and verify the message was written
        let mut file = File::open(&log_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        assert!(contents.contains("INFO"));
        assert!(contents.contains("Test message"));
        assert!(contents.contains("test_module:42"));
    }
}
