// src/source.rs

use crate::error::AppError;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

fn format_file_entry(path: &str, content: &str) -> String {
    format!("--- file: {} ---\n{}\n", path, content)
}

pub fn walk_directory(
    dir: &str,
    glob_pattern: &str,
    excludes: &[String],
) -> Result<String, AppError> {
    let dir_path = Path::new(dir)
        .canonicalize()
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {}", dir, e)))?;

    let mut override_builder = OverrideBuilder::new(&dir_path);
    override_builder
        .add(glob_pattern)
        .map_err(|e| AppError::Ignore(e.to_string()))?;

    for pattern in excludes {
        override_builder
            .add(&format!("!{}", pattern))
            .map_err(|e| AppError::Ignore(e.to_string()))?;
    }

    let overrides = override_builder
        .build()
        .map_err(|e| AppError::Ignore(e.to_string()))?;

    let mut entries: Vec<(String, String)> = WalkBuilder::new(&dir_path)
        .overrides(overrides)
        .hidden(true)
        .build()
        .filter_map(|result| result.ok())
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter_map(|entry| {
            let abs = entry.path().to_path_buf();
            let rel = abs.strip_prefix(&dir_path).ok()?.to_string_lossy().replace('\\', "/");
            let content = fs::read_to_string(&abs).ok()?;
            Some((rel, content))
        })
        .collect();

    if entries.is_empty() {
        return Err(AppError::EmptySource(format!(
            "no files matched '{}' in '{}'",
            glob_pattern, dir
        )));
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let output = entries
        .iter()
        .map(|(path, content)| format_file_entry(path, content))
        .collect::<Vec<_>>()
        .join("");

    Ok(output)
}

pub fn read_manifest(manifest_path: &str) -> Result<String, AppError> {
    let manifest_path = Path::new(manifest_path);
    let base_dir = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."));

    let manifest_content = fs::read_to_string(manifest_path)?;

    let mut parts: Vec<String> = Vec::new();

    for (line_no, line) in manifest_content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let resolved = base_dir.join(trimmed);
        let content = fs::read_to_string(&resolved).map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!(
                    "line {}: {}: {}",
                    line_no + 1,
                    resolved.display(),
                    e
                ),
            )
        })?;

        parts.push(format_file_entry(trimmed, &content));
    }

    if parts.is_empty() {
        return Err(AppError::EmptySource(format!(
            "no files listed in manifest '{}'",
            manifest_path.display()
        )));
    }

    Ok(parts.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_tmp() -> TempDir {
        let tmp = tempfile::tempdir().expect("failed to create tempdir");
        let src = tmp.path().join("src");
        fs::create_dir_all(&src).expect("failed to create src dir");
        fs::write(src.join("main.rs"), "fn main() {}").expect("write main.rs");
        fs::write(src.join("lib.rs"), "pub mod foo;").expect("write lib.rs");
        fs::write(tmp.path().join("README.md"), "# hello").expect("write README.md");
        tmp
    }

    #[test]
    fn walk_filters_by_glob() {
        let tmp = setup_tmp();
        let output = walk_directory(tmp.path().to_str().unwrap(), "**/*.rs", &[])
            .expect("walk_directory failed");

        assert!(output.contains("src/lib.rs"), "should contain src/lib.rs");
        assert!(output.contains("src/main.rs"), "should contain src/main.rs");
        assert!(!output.contains("README.md"), "should not contain README.md");
    }

    #[test]
    fn walk_excludes_patterns() {
        let tmp = setup_tmp();
        let excludes = vec!["src/lib.rs".to_string()];
        let output = walk_directory(tmp.path().to_str().unwrap(), "**/*.rs", &excludes)
            .expect("walk_directory failed");

        assert!(output.contains("src/main.rs"), "should contain src/main.rs");
        assert!(!output.contains("src/lib.rs"), "should not contain src/lib.rs");
    }

    #[test]
    fn walk_empty_dir_errors() {
        let tmp = tempfile::tempdir().expect("failed to create tempdir");
        let result = walk_directory(tmp.path().to_str().unwrap(), "**/*.rs", &[]);
        assert!(result.is_err(), "should return Err for empty dir");
    }

    #[test]
    fn manifest_reads_in_order() {
        let tmp = setup_tmp();
        let manifest_path = tmp.path().join("order.manifest");
        fs::write(&manifest_path, "src/main.rs\nsrc/lib.rs\n")
            .expect("write manifest");

        let output = read_manifest(manifest_path.to_str().unwrap())
            .expect("read_manifest failed");

        let pos_main = output.find("src/main.rs").expect("main.rs not found");
        let pos_lib = output.find("src/lib.rs").expect("lib.rs not found");
        assert!(
            pos_main < pos_lib,
            "src/main.rs should appear before src/lib.rs"
        );
    }

    #[test]
    fn manifest_missing_file_errors() {
        let tmp = setup_tmp();
        let manifest_path = tmp.path().join("missing.manifest");
        fs::write(&manifest_path, "src/nonexistent.rs\n").expect("write manifest");

        let result = read_manifest(manifest_path.to_str().unwrap());
        assert!(result.is_err(), "should return Err for missing file");
    }

    #[test]
    fn manifest_empty_errors() {
        let tmp = setup_tmp();
        let manifest_path = tmp.path().join("empty.manifest");
        fs::write(&manifest_path, "# comment\n\n# another comment\n")
            .expect("write manifest");

        let result = read_manifest(manifest_path.to_str().unwrap());
        assert!(result.is_err(), "should return Err for manifest with only comments");
    }
}
