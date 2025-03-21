use reqwest::blocking;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn prepare_sources(output_dir: &str, urls: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // Prepare __init__ files
    let output_dir_path = Path::new(output_dir);
    if let Some(parent) = output_dir_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let init_files = [
        format!("{}/__init__.py", output_dir),
        format!("{}/src/__init__.py", output_dir),
    ];

    for path in &init_files {
        let file_path = Path::new(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        File::create(file_path)?;
        println!("Created init file: {:?}", file_path);
    }

    // Download and save source files
    for url in urls {
        // Extract relative path from URL
        let base_url = "https://github.com/selfcustody/krux/raw/main/";
        let relative_path = url
            .strip_prefix(base_url)
            .ok_or_else(|| format!("URL '{}' doesn't match base pattern", url))?;

        // Build local file path
        let file_path = Path::new(output_dir).join(relative_path);
        
        // Create parent directories
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Download file
        let response = blocking::get(url)?;
        
        match response.status().as_u16() {
            200 => {
                let content = response.bytes()?;
                let mut file = File::create(&file_path)?;
                file.write_all(&content)?;
                println!("Downloaded: {:?}", file_path);
            },
            404 => return Err(format!("File not found: {}", url).into()),
            status => return Err(format!("HTTP error {} for {}", status, url).into()),
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = "krux";
    let urls = vec![
        "https://github.com/selfcustody/krux/raw/main/src/krux/bbqr.py",
        "https://github.com/selfcustody/krux/raw/main/src/krux/bip39.py",
    ];

    prepare_sources(output_dir, urls)?;
    Ok(())
}