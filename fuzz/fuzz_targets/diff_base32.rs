#![no_main]
use base32;
use libfuzzer_sys::fuzz_target;
use std::process::Command;
use std::str;
use serde_json;
use serde::de::DeserializeOwned;

fn call_python_script<T: DeserializeOwned>(
    action: &str,
    padding: bool,
    input: &[u8]
) -> Result<T, Box<dyn std::error::Error>> {
    let input_json = serde_json::json!({
        "action": action,
        "padding": padding,
        "input": input,
    });

    let output = Command::new("python3")
        .arg("fuzz/fuzz_targets/diff_base32.py")
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
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let result: serde_json::Value = serde_json::from_str(&output_str)?;
    
    let result_value = result.get("result")
        .ok_or("Missing 'result' field in response")?;
    
    serde_json::from_value(result_value.clone())
        .map_err(|e| format!("Failed to deserialize result: {}", e).into())
}

fuzz_target!(|data: &[u8]| {
    for &pad in &[false, true] {
        let alphabet = base32::Alphabet::Rfc4648 { padding: pad };
        
        let rust_encoded = base32::encode(alphabet, data);
        let python_encoded: String = call_python_script("encode", pad, data).expect("Python encode failed");
        assert_eq!(
            rust_encoded,
            python_encoded,
            "Encoding mismatch with padding={}",
            pad
        );

        // Fix decoding comparison
        let python_decoded: Vec<u8> = call_python_script("decode", pad, rust_encoded.as_bytes()).expect("Python decode failed");
        assert_eq!(
            data,
            &python_decoded,
            "Decoding mismatch with padding={}",
            pad
        );
    }
});
