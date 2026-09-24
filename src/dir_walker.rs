use std::fs;


pub fn find_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    match find_files(root, paths) {
        Ok() => paths,
        Err(error) => println!("Error: {}", error)
    }
}

priv fn find_files(path: &Path, paths: &Vec<PathBuf>) -> Result<(), String> {
    if path.is_dir() {
        

    } else {
        let mut pathbuf: PathBuf = path.to_bath_puf();
        paths.push(pathbuf)
    }
    Ok()
}