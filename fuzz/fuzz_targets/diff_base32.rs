#![no_main]
use base32;
use libfuzzer_sys::fuzz_target;
use call_python_script;

fuzz_target!(|data: &[u8]| {
    
    for &padding in &[false, true] {
        let alphabet = base32::Alphabet::Rfc4648 { padding };
        
        let rust_encoded = base32::encode(alphabet, data);
        let python_encoded: String = call_python_script::run(
            "fuzz/fuzz_targets/diff_base32.py", 
            &serde_json::json!({
                "action": "encode",
                "padding": padding,
                "input": data,
            })
        ).expect("Python encode failed");

        assert_eq!(rust_encoded, python_encoded);

        let python_decoded: Vec<u8> = call_python_script::run(
            "fuzz/fuzz_targets/diff_base32.py",
            &serde_json::json!({
                "action": "decode",
                "padding": padding,
                "input": rust_encoded.as_bytes(),
            })
        ).expect("Python decode failed");

        assert_eq!(data, &python_decoded);
    }
});
