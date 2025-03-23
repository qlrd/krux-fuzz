use reqwest::blocking;
use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// create_dir_and_its_init_file
///
/// Create parent directories and their `__init__.py` files
fn create_dir_and_its_init_file(
    output_dir: &Path,
    dir: &Path,
    seen_dirs: &mut HashSet<PathBuf>,
    seen_inits: &mut HashSet<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    // Create directory if not already done
    if seen_dirs.insert(dir.to_path_buf()) {
        println!("Creating directory: {}", dir.display());
        fs::create_dir_all(dir)?;
    }

    let mut current = dir.to_path_buf();
    while current.starts_with(output_dir) {
        let init_file_path = current.join("__init__.py");

        if seen_inits.insert(init_file_path.clone()) {
            println!("Creating init: {}", init_file_path.display());
            if !init_file_path.exists() {
                File::create(init_file_path)?;
            }
        }

        if !current.pop() {
            break;
        }
    }

    Ok(())
}

/// run
///
/// create directories, __init__ files and download requested krux sources
pub fn run(base_url: &str, output_dir: &str, urls: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let output_root = Path::new(output_dir);

    // Track seen directories and init files
    let mut seen_dirs = HashSet::new();
    let mut seen_inits = HashSet::new();

    for url in urls {
        // Extract relative path
        let relative_path = url
            .strip_prefix(base_url)
            .ok_or_else(|| format!("URL '{}' doesn't match base pattern", url))?;

        // Construct local file path
        let file_path = output_root.join(relative_path);
        let parent_dir = file_path
            .parent()
            .ok_or("File does not have a parent directory")?;

        // Ensure directory structure and init files
        create_dir_and_its_init_file(output_root, parent_dir, &mut seen_dirs, &mut seen_inits)?;

        // Download and save file
        let response = blocking::get(url)?;
        match response.status().as_u16() {
            200 => {
                let content = response.bytes()?;
                let mut file = File::create(&file_path)?;
                file.write_all(&content)?;
                println!("Downloaded: {:?}", file_path);
            }
            404 => return Err(format!("File not found: {}", url).into()),
            status => return Err(format!("HTTP error {} for {}", status, url).into()),
        }
    }

    Ok(())
}
