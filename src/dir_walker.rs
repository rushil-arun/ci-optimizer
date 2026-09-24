use std::fs;
use std::path::{Path, PathBuf};
use std::io::Result;


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

        assert!(names.iter().any(|p| p.ends_with("src/dir_walker.rs")));
        assert!(names.iter().any(|p| p.ends_with("src/lib.rs")));
        assert!(names.iter().any(|p| p.ends_with("Cargo.toml")));
        assert!(names.iter().any(|p| p.ends_with("README.md")));
    }
}