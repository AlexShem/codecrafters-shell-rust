use std::fs;

/// Scan the current directory for files matching a prefix
pub fn scan_directory_files(prefix: &str) -> Vec<String> {
    let mut files = Vec::new();

    let Ok(entries) = fs::read_dir(".") else {
        return files;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Skip directories for now (only match files)
        if path.is_dir() {
            continue;
        }

        // Get the filename as a string
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            // Skip hidden files (starting with .)
            if filename.starts_with('.') {
                continue;
            }

            // Check if filename starts with the prefix
            if filename.starts_with(prefix) {
                files.push(filename.to_string());
            }
        }
    }

    files.sort();
    files
}

/// Find all files in the current directory that match a prefix
pub fn find_matching_files(prefix: &str) -> Vec<String> {
    scan_directory_files(prefix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;
    use std::env;

    #[test]
    fn test_find_matching_files_single_match() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create a test file
        let file_path = temp_path.join("readme.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        // Change to temp directory
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_path).unwrap();

        let results = find_matching_files("read");

        // Restore original directory
        env::set_current_dir(&original_dir).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "readme.txt");
    }

    #[test]
    fn test_find_matching_files_no_match() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create a test file
        let file_path = temp_path.join("readme.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        // Change to temp directory
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_path).unwrap();

        let results = find_matching_files("xyz");

        // Restore original directory
        env::set_current_dir(&original_dir).unwrap();

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_find_matching_files_multiple_matches() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test files
        File::create(temp_path.join("readme.txt")).unwrap();
        File::create(temp_path.join("read_me.md")).unwrap();
        File::create(temp_path.join("other.txt")).unwrap();

        // Change to temp directory
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_path).unwrap();

        let results = find_matching_files("read");

        // Restore original directory
        env::set_current_dir(&original_dir).unwrap();

        assert_eq!(results.len(), 2);
        assert!(results.contains(&"readme.txt".to_string()));
        assert!(results.contains(&"read_me.md".to_string()));
    }

    #[test]
    fn test_find_matching_files_excludes_hidden() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test files
        File::create(temp_path.join("visible.txt")).unwrap();
        File::create(temp_path.join(".hidden.txt")).unwrap();

        // Change to temp directory
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_path).unwrap();

        let results = find_matching_files("");

        // Restore original directory
        env::set_current_dir(&original_dir).unwrap();

        // Should only find visible.txt, not .hidden.txt
        assert!(!results.iter().any(|f| f.starts_with('.')));
    }
}


