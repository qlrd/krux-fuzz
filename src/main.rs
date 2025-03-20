use reqwest::blocking;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn create_init_files() -> std::io::Result<()> {
    let init_files = ["krux/__init__.py", "krux/src/__init__.py"];

    for path in &init_files {
        let file_path = Path::new(path);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        println!("Creating {:?}", file_path);
        File::create(file_path)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://github.com/selfcustody/krux/raw/main/src/krux/bbqr.py";
    let output_path = "krux/src/bbqr.py";

    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
        create_init_files()?;
    }

    let response = blocking::get(url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    // Write to file
    let mut file = File::create(output_path)?;
    file.write_all(&response.bytes()?)?;

    println!("Successfully downloaded to {}", output_path);
    Ok(())
}
