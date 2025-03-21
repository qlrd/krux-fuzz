use std::process::Command;
use serde::de::DeserializeOwned;

pub fn run<T: DeserializeOwned>(
    script_path: &str,
    input_json: &serde_json::Value,
) -> Result<T, Box<dyn std::error::Error>> {
    let output = Command::new("python3")
        .arg(script_path)
        .arg("--data")
        .arg(input_json.to_string())
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Python script failed (status {}): {}",
            output.status, error_msg
        ).into());
    }
    
    let output_str = std::str::from_utf8(&output.stdout)?;
    let result: serde_json::Value = serde_json::from_str(output_str)?;
    
    let result_value = result.get("result")
        .ok_or_else(|| format!("Missing 'result' field in response: {}", result))?;
    
    serde_json::from_value(result_value.clone())
        .map_err(|e| format!("Failed to deserialize result: {} (value: {})", e, result_value).into())
}