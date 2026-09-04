use std::fs;
use std::path::{Path, PathBuf};

const SCAN_EXTENSIONS: &[&str] = &[".md", ".txt"];

pub fn collect_source_files(directory: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_file()
            && let Some(ext) = path.extension()
        {
            let ext_str = format!(".{}", ext.to_string_lossy().to_lowercase());
            if SCAN_EXTENSIONS.contains(&ext_str.as_str()) {
                files.push(path);
            }
        }
    }

    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn extension_match_is_case_insensitive() {
        let dir = env::temp_dir().join("foton-ext-case-test");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("A.MD"), "x").unwrap();
        fs::write(dir.join("b.Txt"), "x").unwrap();
        fs::write(dir.join("c.rs"), "x").unwrap();

        let files = collect_source_files(&dir).unwrap();
        let names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(names.contains(&"A.MD".to_string()));
        assert!(names.contains(&"b.Txt".to_string()));
        assert!(!names.contains(&"c.rs".to_string()));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn missing_directory_is_an_error() {
        let missing = env::temp_dir().join("foton-no-such-dir-xyz");
        assert!(collect_source_files(&missing).is_err());
    }
}
