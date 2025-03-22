use serde::de::DeserializeOwned;
use std::process::Command;

#[derive(Debug)]
pub enum PythonScriptError {
    OutputError(std::io::Error),
    OutputFailed(String),
    Utf8Failed(std::str::Utf8Error),
    JsonError(serde_json::Error),
    MissingResult(serde_json::Value),
    DeserializationFailed(serde_json::Value, serde_json::Error),
}

impl std::fmt::Display for PythonScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PythonScriptError::OutputError(err) => {
                write!(f, "Python output failed: {}", err)
            }
            PythonScriptError::OutputFailed(err) => {
                write!(f, "Python script failed: {}", err)
            }
            PythonScriptError::Utf8Failed(err) => {
                write!(f, "Malformed script: {}", err)
            }
            PythonScriptError::JsonError(err) => {
                write!(f, "Malformed Json: {}", err)
            }
            PythonScriptError::MissingResult(err) => {
                write!(f, "Missing 'result' field in response: {}", err)
            }
            PythonScriptError::DeserializationFailed(result, err) => {
                write!(
                    f,
                    "Failed to deserialize result: {} (value: {})",
                    err, result
                )
            }
        }
    }
}

/// run
///
/// Take an arbitrary CLI-based python script (constructed with argparse module)
/// that have a single option --data and execute it with python3.
///
/// This `--data` option must receive only a JSON formated string. The code below
/// will convert to `python3 my/path/to/script.py --data="{\"attr\": \"value\"}"`:
///
/// ```rust
/// use serde_json::json;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct Output {
///     result: String,
/// }
///
/// // Assuming you have a script at this path that returns:
/// // { "result": "hello world" }
/// let input = json!({ "attr": "value" });
/// let output: Output = call_python_script::run("tests/fixtures/echo_script.py", &input).unwrap();
/// assert_eq!(output.result, "hello world");
/// ```
pub fn run<T: DeserializeOwned>(
    script_path: &str,
    input_json: &serde_json::Value,
) -> Result<T, PythonScriptError> {
    let output = Command::new("python3")
        .arg(script_path)
        .arg("--data")
        .arg(input_json.to_string())
        .output()
        .map_err(PythonScriptError::OutputError)?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(PythonScriptError::OutputFailed(error_msg.to_string()));
    }

    let output_str = std::str::from_utf8(&output.stdout).map_err(PythonScriptError::Utf8Failed)?;
    let result: serde_json::Value =
        serde_json::from_str(output_str).map_err(PythonScriptError::JsonError)?;

    let deserialized = serde_json::from_value(
        result
            .get("result")
            .cloned()
            .ok_or_else(|| PythonScriptError::MissingResult(result.clone()))?,
    )
    .map_err(|err| PythonScriptError::DeserializationFailed(result, err))?;

    Ok(deserialized)
}
