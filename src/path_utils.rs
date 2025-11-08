use std::path::{Path, PathBuf};
use std::{env, fs};
use std::collections::HashSet;

/// Get the OS-specific PATH separator
pub fn path_separator() -> char {
    if cfg!(windows) {
        ';'
    } else {
        ':'
    }
}

/// Check if a file has execute permissions (Unix only, always true on Windows)
pub fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(path) {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
        false
    }

    #[cfg(not(unix))]
    {
        path.exists()
    }
}

/// Find an executable in PATH directories
pub fn find_in_path(command: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?;

    for dir in path_var.split(path_separator()) {
        let full_path = Path::new(dir).join(command);

        if full_path.exists() && is_executable(&full_path) {
            return Some(full_path);
        }
    }

    None
}

/// Scan all directories in PATH and collect executable file names
pub fn scan_path_executables() -> HashSet<String> {
    let mut executables = HashSet::new();

    let Ok(path_var) = env::var("PATH") else {
        return executables;
    };

    for dir in path_var.split(path_separator()) {
        let dir_path = Path::new(dir);

        let Ok(entries) = fs::read_dir(dir_path) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            if is_executable(&path) {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    executables.insert(filename.to_string());
                }
            }
        }
    }

    executables
}