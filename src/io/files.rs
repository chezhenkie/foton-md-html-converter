use std::fs;
use std::path::{Path, PathBuf};

const SCAN_EXTENSIONS: &[&str] = &[".md", ".txt"];

pub fn collect_source_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = format!(".{}", ext.to_string_lossy());
                    if SCAN_EXTENSIONS.contains(&ext_str.as_str()) {
                        files.push(path);
                    }
                }
            }
        }
    }

    files.sort();
    files
}
