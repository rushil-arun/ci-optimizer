use std::fs;
use std::path::{Path, PathBuf};
use std::io::Result;
use std::collections::HashMap;

pub fn filter_by_extension(files: Vec<PathBuf>) -> Result<HashMap<String, Vec<PathBuf>>> {
    let mut extensions: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for file in files {

        let extension_type = file
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("no extension")
            .to_string();

        extensions.entry(extension_type).or_insert(Vec::new()).push(file);
    }
    Ok(extensions)
}

pub fn find_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    collect_files(root, &mut paths)?;
    Ok(paths)
}

fn collect_files(path: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            collect_files(&entry.path(), paths)?;
        }

    } else {
        paths.push(path.to_path_buf());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn find_files_in_a_tree() {
        let mut files = find_files(Path::new(".")).expect("walk should succeed");
        files.sort();
        for path in &files {
            println!("{}", path.display());
        }

        let names: Vec<_> = files
            .iter()
            .filter(|p| !p.starts_with("target") && !p.starts_with(".git"))
            .collect();

        assert!(names.iter().any(|p| p.ends_with("src/files.rs")));
        assert!(names.iter().any(|p| p.ends_with("src/lib.rs")));
        assert!(names.iter().any(|p| p.ends_with("Cargo.toml")));
        assert!(names.iter().any(|p| p.ends_with("README.md")));
    }

    #[test]
    fn extension_filter_test() {
        let mut files = find_files(Path::new(".")).expect("walk should succeed");
        files.sort();
        for path in &files {
            println!("{}", path.display());
        }

        let names: Vec<_> = files
            .into_iter()
            .filter(|p| !p.starts_with("target") && !p.starts_with(".git"))
            .collect();

        let extension_groups = filter_by_extension(names);
        assert!(extension_groups
            .unwrap()
            .get("toml")
            .map(|paths| paths.iter().any(|p| p.ends_with("Cargo.toml")))
            .unwrap_or(false));
    
    }
}