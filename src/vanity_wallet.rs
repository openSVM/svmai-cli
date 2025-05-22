// vanity_wallet.rs

use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use rayon::prelude::*;
use std::io;

/// Configuration for vanity address generation
#[derive(Clone)]
pub struct VanityConfig {
    /// The prefix that the address should start with (case-insensitive)
    pub prefix: String,
    /// Maximum time to spend searching (in seconds)
    pub timeout_seconds: u64,
    /// Number of threads to use for grinding
    pub thread_count: usize,
    /// How often to report progress (in milliseconds)
    pub progress_interval_ms: u64,
}

impl Default for VanityConfig {
    fn default() -> Self {
        VanityConfig {
            prefix: "ai".to_string(),
            timeout_seconds: 60,
            // Limit to 8 threads to avoid excessive CPU usage
            thread_count: num_cpus::get().min(8),
            progress_interval_ms: 500,
        }
    }
}

/// Status of the vanity address generation process
#[derive(Clone, Debug)]
pub struct VanityStatus {
    /// Number of attempts made so far
    pub attempts: u64,
    /// Whether the process has completed
    pub completed: bool,
    /// Whether the process was successful
    pub success: bool,
    /// The generated public key as string (if successful)
    pub pubkey: Option<String>,
    /// Time elapsed in seconds
    pub elapsed_seconds: f64,
    /// Attempts per second
    pub attempts_per_second: f64,
}

/// Error types for vanity address generation
#[derive(Debug, PartialEq)]
pub enum VanityError {
    /// The process timed out
    Timeout,
    /// The process was cancelled
    Cancelled,
    /// An I/O error occurred
    IoError(String),
}

impl std::fmt::Display for VanityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VanityError::Timeout => write!(f, "Vanity address generation timed out"),
            VanityError::Cancelled => write!(f, "Vanity address generation was cancelled"),
            VanityError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for VanityError {}

impl From<io::Error> for VanityError {
    fn from(error: io::Error) -> Self {
        VanityError::IoError(error.to_string())
    }
}

/// Generate a keypair with a vanity address that starts with the specified prefix
pub fn generate_vanity_keypair(config: &VanityConfig) -> Result<Keypair, VanityError> {
    // Clone all values needed by threads to avoid lifetime issues
    let prefix = config.prefix.to_lowercase();
    let timeout_secs = config.timeout_seconds;
    let thread_count = config.thread_count;
    let progress_interval_ms = config.progress_interval_ms;
    
    let start_time = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    
    // Create a thread pool with the specified number of threads
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();
    
    // Shared state for tracking attempts and result
    let attempts = Arc::new(Mutex::new(0u64));
    let result = Arc::new(Mutex::new(None::<Keypair>));
    let found = Arc::new(AtomicBool::new(false));
    
    // Progress reporting thread
    let attempts_clone = Arc::clone(&attempts);
    let found_clone = Arc::clone(&found);
    let prefix_clone = prefix.clone();
    
    let progress_handle = thread::spawn(move || {
        let progress_interval = Duration::from_millis(progress_interval_ms);
        let mut last_report = Instant::now();
        let mut last_attempts = 0;
        
        while !found_clone.load(Ordering::SeqCst) && start_time.elapsed() < timeout {
            thread::sleep(progress_interval);
            
            let current_attempts = *attempts_clone.lock().unwrap();
            let elapsed = last_report.elapsed();
            let attempts_since_last = current_attempts - last_attempts;
            let attempts_per_second = attempts_since_last as f64 / elapsed.as_secs_f64();
            
            println!(
                "Searching for vanity address with prefix '{}': {} attempts, {:.2} attempts/sec",
                prefix_clone, current_attempts, attempts_per_second
            );
            
            last_attempts = current_attempts;
            last_report = Instant::now();
        }
    });
    
    // Vanity address generation
    pool.install(|| {
        (0..thread_count).into_par_iter().try_for_each(|_| {
            let attempts_ref = Arc::clone(&attempts);
            let result_ref = Arc::clone(&result);
            let found_ref = Arc::clone(&found);
            let prefix_ref = prefix.clone();
            
            while !found_ref.load(Ordering::SeqCst) && start_time.elapsed() < timeout {
                // Generate a new random keypair
                let keypair = Keypair::new();
                let pubkey = keypair.pubkey().to_string();
                
                // Increment attempt counter
                {
                    let mut attempts = attempts_ref.lock().unwrap();
                    *attempts += 1;
                }
                
                // Check if the address starts with the desired prefix
                if pubkey.to_lowercase().starts_with(&prefix_ref) {
                    // We found a match!
                    let mut result = result_ref.lock().unwrap();
                    *result = Some(keypair);
                    
                    found_ref.store(true, Ordering::SeqCst);
                    
                    return Err(());  // Break out of the parallel loop
                }
            }
            
            Ok(())
        })
        .ok();
    });
    
    // Wait for progress thread to finish
    let _ = progress_handle.join();
    
    // Check if we found a keypair
    let mut result_guard = result.lock().unwrap();
    match result_guard.take() {
        Some(keypair) => Ok(keypair),
        None => Err(VanityError::Timeout),
    }
}

/// Generate a vanity keypair with progress updates
pub fn generate_vanity_keypair_with_progress<F>(
    config: &VanityConfig,
    progress_callback: F,
) -> Result<Keypair, VanityError>
where
    F: Fn(&VanityStatus) + Send + Sync + 'static,
{
    // Clone all values needed by threads to avoid lifetime issues
    let prefix = config.prefix.to_lowercase();
    let timeout_secs = config.timeout_seconds;
    let thread_count = config.thread_count;
    let progress_interval_ms = config.progress_interval_ms;
    
    let start_time = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    
    // Shared state for tracking attempts and result
    let attempts = Arc::new(Mutex::new(0u64));
    let result = Arc::new(Mutex::new(None::<Keypair>));
    let found = Arc::new(AtomicBool::new(false));
    let cancelled = Arc::new(AtomicBool::new(false));
    
    // Create a shared callback that can be used in multiple threads
    let callback = Arc::new(progress_callback);
    
    // Progress reporting thread
    let attempts_clone = Arc::clone(&attempts);
    let found_clone = Arc::clone(&found);
    let cancelled_clone = Arc::clone(&cancelled);
    let callback_clone = Arc::clone(&callback);
    
    let progress_handle = thread::spawn(move || {
        let progress_interval = Duration::from_millis(progress_interval_ms);
        let mut last_attempts = 0;
        let mut last_time = Instant::now();
        
        while !found_clone.load(Ordering::SeqCst) && 
              !cancelled_clone.load(Ordering::SeqCst) && 
              start_time.elapsed() < timeout {
            thread::sleep(progress_interval);
            
            let current_attempts = *attempts_clone.lock().unwrap();
            let elapsed = start_time.elapsed();
            let elapsed_seconds = elapsed.as_secs_f64();
            
            let time_diff = last_time.elapsed().as_secs_f64();
            let attempts_diff = current_attempts - last_attempts;
            let attempts_per_second = if time_diff > 0.0 {
                attempts_diff as f64 / time_diff
            } else {
                0.0
            };
            
            // Call the progress callback
            callback_clone(&VanityStatus {
                attempts: current_attempts,
                completed: false,
                success: false,
                pubkey: None,
                elapsed_seconds,
                attempts_per_second,
            });
            
            last_attempts = current_attempts;
            last_time = Instant::now();
        }
    });
    
    // Create a thread pool with the specified number of threads
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();
    
    // Vanity address generation
    pool.install(|| {
        (0..thread_count).into_par_iter().try_for_each(|_| {
            let attempts_ref = Arc::clone(&attempts);
            let result_ref = Arc::clone(&result);
            let found_ref = Arc::clone(&found);
            let cancelled_ref = Arc::clone(&cancelled);
            let prefix_ref = prefix.clone();
            
            let mut counter = 0;
            
            while !found_ref.load(Ordering::SeqCst) && 
                  !cancelled_ref.load(Ordering::SeqCst) && 
                  start_time.elapsed() < timeout {
                // Generate a new random keypair
                let keypair = Keypair::new();
                let pubkey = keypair.pubkey().to_string();
                
                // Increment attempt counter
                {
                    let mut attempts = attempts_ref.lock().unwrap();
                    *attempts += 1;
                }
                
                // Check if the address starts with the desired prefix
                if pubkey.to_lowercase().starts_with(&prefix_ref) {
                    // We found a match!
                    let mut result = result_ref.lock().unwrap();
                    *result = Some(keypair);
                    
                    found_ref.store(true, Ordering::SeqCst);
                    
                    return Err(());  // Break out of the parallel loop
                }
                
                // Check for cancellation more frequently (every 100 attempts)
                counter += 1;
                if counter % 100 == 0 {
                    if cancelled_ref.load(Ordering::SeqCst) {
                        return Err(());  // Break out if cancelled
                    }
                    // Yield to allow other threads to run, especially important for cancellation
                    std::thread::yield_now();
                }
            }
            
            Ok(())
        })
        .ok();
    });
    
    // Wait for progress thread to finish
    let _ = progress_handle.join();
    
    // Check if the operation was cancelled first
    if cancelled.load(Ordering::SeqCst) {
        // Final progress update with cancellation
        let total_attempts = *attempts.lock().unwrap();
        let elapsed = start_time.elapsed();
        let elapsed_seconds = elapsed.as_secs_f64();
        let attempts_per_second = if elapsed_seconds > 0.0 {
            total_attempts as f64 / elapsed_seconds
        } else {
            0.0
        };
        
        callback(&VanityStatus {
            attempts: total_attempts,
            completed: true,
            success: false,
            pubkey: None,
            elapsed_seconds,
            attempts_per_second,
        });
        
        return Err(VanityError::Cancelled);
    }
    
    // Use the original callback reference for the final updates
    let total_attempts = *attempts.lock().unwrap();
    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    let attempts_per_second = if elapsed_seconds > 0.0 {
        total_attempts as f64 / elapsed_seconds
    } else {
        0.0
    };
    
    let mut result_guard = result.lock().unwrap();
    match result_guard.take() {
        Some(keypair) => {
            // Final progress update with success
            callback(&VanityStatus {
                attempts: total_attempts,
                completed: true,
                success: true,
                pubkey: Some(keypair.pubkey().to_string()),
                elapsed_seconds,
                attempts_per_second,
            });
            
            Ok(keypair)
        },
        None => {
            // Final progress update with failure
            callback(&VanityStatus {
                attempts: total_attempts,
                completed: true,
                success: false,
                pubkey: None,
                elapsed_seconds,
                attempts_per_second,
            });
            
            Err(VanityError::Timeout)
        }
    }
}

/// Cancel an ongoing vanity address generation process
pub fn cancel_vanity_generation(cancelled: &Arc<AtomicBool>) {
    cancelled.store(true, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vanity_generation_basic() {
        let config = VanityConfig {
            prefix: "a".to_string(),  // Use a single character for quick test
            timeout_seconds: 10,
            thread_count: 2,
            progress_interval_ms: 100,
        };
        
        let result = generate_vanity_keypair(&config);
        assert!(result.is_ok(), "Should find an address starting with 'a'");
        
        let keypair = result.unwrap();
        let pubkey = keypair.pubkey().to_string();
        assert!(pubkey.to_lowercase().starts_with("a"), 
                "Generated address should start with 'a', got: {}", pubkey);
    }
    
    #[test]
    fn test_vanity_generation_with_progress() {
        let config = VanityConfig {
            prefix: "a".to_string(),  // Use a single character for quick test
            timeout_seconds: 10,
            thread_count: 2,
            progress_interval_ms: 100,
        };
        
        let progress_updates = Arc::new(Mutex::new(Vec::new()));
        let progress_updates_clone = Arc::clone(&progress_updates);
        
        let result = generate_vanity_keypair_with_progress(
            &config,
            move |status| {
                let mut updates = progress_updates_clone.lock().unwrap();
                updates.push(status.attempts);
            }
        );
        
        assert!(result.is_ok(), "Should find an address starting with 'a'");
        
        let keypair = result.unwrap();
        let pubkey = keypair.pubkey().to_string();
        assert!(pubkey.to_lowercase().starts_with("a"), 
                "Generated address should start with 'a', got: {}", pubkey);
        
        let updates = progress_updates.lock().unwrap();
        assert!(!updates.is_empty(), "Should have received progress updates");
    }
    
    #[test]
    fn test_vanity_generation_timeout() {
        let config = VanityConfig {
            // Use a very unlikely prefix to trigger timeout
            prefix: "impossible1234567890".to_string(),
            timeout_seconds: 1,  // Short timeout
            thread_count: 1,
            progress_interval_ms: 100,
        };
        
        let result = generate_vanity_keypair(&config);
        assert!(matches!(result, Err(VanityError::Timeout)), 
                "Should timeout when searching for an unlikely prefix");
    }
    
    #[test]
    fn test_vanity_generation_cancel() {
        // Create a direct test that doesn't rely on channels or complex synchronization
        let config = VanityConfig {
            // Use a very unlikely prefix so we have time to cancel
            prefix: "impossible1234567890".to_string(),
            timeout_seconds: 30,  // Long timeout to ensure we don't hit it
            thread_count: 1,
            progress_interval_ms: 50,  // Faster progress updates
        };
        
        // Use AtomicBool for thread-safe cancellation
        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_clone = Arc::clone(&cancelled);
        
        // Create a flag to track if the callback was called with a cancellation status
        let was_cancelled = Arc::new(AtomicBool::new(false));
        let was_cancelled_clone = Arc::clone(&was_cancelled);
        
        // Start the vanity generation in a separate thread
        let handle = thread::spawn(move || {
            let result = generate_vanity_keypair_with_progress(
                &config,
                move |status| {
                    // Check if this is a completion callback with cancelled status
                    if status.completed && !status.success {
                        was_cancelled_clone.store(true, Ordering::SeqCst);
                    }
                }
            );
            
            // Return the result for verification
            result
        });
        
        // Wait a bit to ensure the generation has started
        thread::sleep(Duration::from_millis(200));
        
        // Cancel the generation
        cancel_vanity_generation(&cancelled_clone);
        
        // Wait for the thread to complete with a reasonable timeout
        let result = handle.join().unwrap();
        
        // Verify that we got a cancellation error
        assert!(matches!(result, Err(VanityError::Cancelled)), 
                "Should be cancelled when cancel_vanity_generation is called");
        
        // Verify that the cancellation callback was triggered
        assert!(was_cancelled.load(Ordering::SeqCst), 
                "Cancellation callback should have been triggered");
    }
    
    #[test]
    fn test_vanity_generation_case_insensitive() {
        let config = VanityConfig {
            prefix: "A".to_string(),  // Uppercase prefix
            timeout_seconds: 10,
            thread_count: 2,
            progress_interval_ms: 100,
        };
        
        let result = generate_vanity_keypair(&config);
        assert!(result.is_ok(), "Should find an address starting with 'a' or 'A'");
        
        let keypair = result.unwrap();
        let pubkey = keypair.pubkey().to_string();
        assert!(pubkey.to_lowercase().starts_with("a"), 
                "Generated address should start with 'a' (case-insensitive), got: {}", pubkey);
    }
    
    #[test]
    fn test_vanity_generation_performance() {
        let config = VanityConfig {
            prefix: "a".to_string(),  // Common prefix for quick test
            timeout_seconds: 2,
            thread_count: 4,  // Use multiple threads
            progress_interval_ms: 100,
        };
        
        let start = Instant::now();
        let result = generate_vanity_keypair(&config);
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "Should find an address starting with 'a'");
        println!("Found vanity address in {:?}", duration);
        
        // This is not a strict test, but helps verify performance is reasonable
        assert!(duration < Duration::from_secs(5), 
                "Should find common prefix quickly with multiple threads");
    }
}
