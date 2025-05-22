use rayon::prelude::*;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

// Configuration for file search
pub struct SearchConfig {
    pub max_files: Option<usize>, // Maximum number of files to find before early exit
    pub max_depth: Option<usize>, // Maximum directory depth to search
    pub batch_size: usize,        // Size of batches for parallel processing
}

impl Default for SearchConfig {
    fn default() -> Self {
        SearchConfig {
            max_files: None,
            max_depth: None,
            batch_size: 100,
        }
    }
}

// Helper function to check if a DirEntry is a JSON file
fn is_json_file_entry(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".json"))
        .unwrap_or(false)
}

// Optimized recursive parallel search function using WalkDir and Rayon
pub fn search_json_files_parallel_recursive(dir_path: &str) -> io::Result<Vec<String>> {
    search_json_files_parallel_recursive_with_config(dir_path, &SearchConfig::default())
}

// New version with configuration options for better performance tuning
pub fn search_json_files_parallel_recursive_with_config(
    dir_path: &str,
    config: &SearchConfig,
) -> io::Result<Vec<String>> {
    let path = Path::new(dir_path);
    if !path.is_dir() {
        return Ok(Vec::new()); // Or return an error: io::Error::new(io::ErrorKind::InvalidInput, "Path is not a directory")
    }

    // Create a walkdir iterator with optional max depth
    let mut walker = WalkDir::new(path);
    if let Some(max_depth) = config.max_depth {
        walker = walker.max_depth(max_depth);
    }

    // If max_files is set, use early exit strategy with shared counter
    if let Some(max_files) = config.max_files {
        let result = Arc::new(Mutex::new(Vec::with_capacity(max_files)));
        let count = Arc::new(Mutex::new(0));

        walker
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file() && is_json_file_entry(entry))
            .collect::<Vec<_>>() // Collect to avoid holding the iterator lock during parallel processing
            .chunks(config.batch_size)
            .for_each(|chunk| {
                let result = Arc::clone(&result);
                let count = Arc::clone(&count);

                // Process each batch in parallel
                chunk.par_iter().for_each(|entry| {
                    // Check if we've already found enough files
                    let current_count = {
                        let count = count.lock().unwrap();
                        *count
                    };

                    if current_count >= max_files {
                        return;
                    }

                    let path_str = entry.path().to_string_lossy().into_owned();

                    // Add to results and increment counter
                    let mut result = result.lock().unwrap();
                    let mut count = count.lock().unwrap();

                    if *count < max_files {
                        result.push(path_str);
                        *count += 1;
                    }
                });
            });

        Ok(Arc::try_unwrap(result).unwrap().into_inner().unwrap())
    } else {
        // Original implementation for when no max_files limit is set
        let json_files: Vec<String> = walker
            .into_iter()
            .filter_map(Result::ok) // Filter out directory read errors
            .collect::<Vec<_>>() // Collect to avoid holding the iterator lock
            .chunks(config.batch_size)
            .flat_map(|chunk| {
                chunk
                    .par_iter()
                    .filter(|entry| entry.file_type().is_file() && is_json_file_entry(entry))
                    .map(|entry| entry.path().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(json_files)
    }
}

// This is a placeholder for Solana key validation logic
pub fn is_solana_wallet_json(file_path: &str) -> bool {
    // In a real implementation, you would read the file content,
    // parse the JSON, and check for specific Solana key characteristics.
    // For this example, let's assume any .json file found is a potential candidate
    // and actual validation will happen in a later module.
    // We can simulate some basic check, e.g. if it contains "privateKey" or similar common field.
    let mut file = match fs::File::open(file_path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return false;
    }
    // A very simplistic check, actual validation would be more robust
    // Check if it's a JSON array and contains 64 numbers (typical for Solana secret keys)
    let is_likely_solana_key_format = contents.trim().starts_with("[")
        && contents.trim().ends_with("]")
        && contents
            .split(|c: char| c == ',' || c.is_whitespace() || c == '[' || c == ']')
            .filter(|s| !s.is_empty() && s.chars().all(|ch| ch.is_ascii_digit()))
            .count()
            == 64;

    contents.contains("privateKey") || contents.contains("secretKey") || is_likely_solana_key_format
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_search_json_files_parallel_recursive_empty_dir() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().to_str().unwrap();
        let result = search_json_files_parallel_recursive(dir_path).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_search_json_files_parallel_recursive_with_json_files() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create some dummy files and directories for testing
        File::create(dir_path.join("test1.json")).unwrap();
        File::create(dir_path.join("test2.json")).unwrap();
        File::create(dir_path.join("test3.txt")).unwrap(); // Non-json file

        let sub_dir = dir_path.join("sub");
        std::fs::create_dir_all(&sub_dir).unwrap();
        File::create(sub_dir.join("test_inner.json")).unwrap();
        File::create(sub_dir.join("another.txt")).unwrap();

        let sub_sub_dir = sub_dir.join("sub_sub");
        std::fs::create_dir_all(&sub_sub_dir).unwrap();
        File::create(sub_sub_dir.join("test_inner_most.json")).unwrap();

        let result = search_json_files_parallel_recursive(dir_path.to_str().unwrap()).unwrap();

        // Collect expected file names
        let mut expected_files = vec![
            dir_path.join("test1.json").to_string_lossy().into_owned(),
            dir_path.join("test2.json").to_string_lossy().into_owned(),
            sub_dir
                .join("test_inner.json")
                .to_string_lossy()
                .into_owned(),
            sub_sub_dir
                .join("test_inner_most.json")
                .to_string_lossy()
                .into_owned(),
        ];
        expected_files.sort();
        let mut actual_files = result;
        actual_files.sort();

        assert_eq!(actual_files.len(), 4);
        assert_eq!(actual_files, expected_files);
    }

    #[test]
    fn test_search_with_max_files_limit() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create 10 JSON files
        for i in 0..10 {
            File::create(dir_path.join(format!("test{}.json", i))).unwrap();
        }

        // Test with max_files = 5
        let config = SearchConfig {
            max_files: Some(5),
            max_depth: None,
            batch_size: 2, // Small batch size to test batching
        };

        let result =
            search_json_files_parallel_recursive_with_config(dir_path.to_str().unwrap(), &config)
                .unwrap();

        assert_eq!(result.len(), 5);
    }
    #[test]
    fn test_search_with_max_depth() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create a file in the root directory
        File::create(dir_path.join("root.json")).unwrap();

        let sub_dir = dir_path.join("sub");
        std::fs::create_dir_all(&sub_dir).unwrap();
        File::create(sub_dir.join("level1.json")).unwrap();

        let sub_sub_dir = sub_dir.join("sub_sub");
        std::fs::create_dir_all(&sub_sub_dir).unwrap();
        File::create(sub_sub_dir.join("level2.json")).unwrap();

        // Test with max_depth = 1 (root directory only, per WalkDir's semantics)
        let config = SearchConfig {
            max_files: None,
            max_depth: Some(1),
            batch_size: 100,
        };

        let result =
            search_json_files_parallel_recursive_with_config(dir_path.to_str().unwrap(), &config)
                .unwrap();

        // Should find only root.json (WalkDir's max_depth=1 means only the root directory)
        assert_eq!(result.len(), 1);

        // Verify the level1.json and level2.json are not included
        for path in &result {
            assert!(!path.contains("level1.json"));
            assert!(!path.contains("level2.json"));
        }
    }

    #[test]
    fn test_is_solana_wallet_json_positive_case_array() {
        // Create a dummy file with some content that might resemble a Solana key file part
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("dummy_wallet_array.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64]").unwrap();
        assert!(is_solana_wallet_json(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_is_solana_wallet_json_positive_case_privatekey_field() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("dummy_wallet_field.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{{\"privateKey\": \"somevalue\"}}").unwrap();
        assert!(is_solana_wallet_json(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_is_solana_wallet_json_negative_case_empty() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.json");
        File::create(&file_path).unwrap();
        assert!(!is_solana_wallet_json(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_is_solana_wallet_json_negative_case_not_json_like() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("not_wallet.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "This is just some text.").unwrap();
        assert!(!is_solana_wallet_json(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_is_solana_wallet_json_wrong_array_count() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("wrong_array.json");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "[1,2,3,4,5,6,7,8,9,10]").unwrap(); // Only 10 numbers
        assert!(!is_solana_wallet_json(file_path.to_str().unwrap()));
    }
}
