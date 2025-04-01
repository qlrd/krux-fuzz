#![no_main]
use base32;
use call_python_script;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    for &padding in &[false, true] {
        let alphabet = base32::Alphabet::Rfc4648 { padding };

        let rust_encoded = base32::encode(alphabet, data);
        let krux_encoded: String = call_python_script::run(
            "fuzz/fuzz_targets/diff_base32.py",
            &serde_json::json!({
                "module": "krux",
                "action": "encode",
                "padding": padding,
                "input": data,
            }),
        )
        .expect("krux encode failed");
        assert_eq!(rust_encoded, krux_encoded);

        // BBQr do not have code for padding in utils module
        if !padding {
            let bbqr_encoded: String = call_python_script::run(
                "fuzz/fuzz_targets/diff_base32.py",
                &serde_json::json!({
                    "module": "bbqr",
                    "action": "encode",
                    "padding": padding,
                    "input": data,
                }),
            )
            .expect("bbqr encode failed");

            assert_eq!(krux_encoded, bbqr_encoded);
            assert_eq!(rust_encoded, bbqr_encoded);
        }

        let krux_decoded: Vec<u8> = call_python_script::run(
            "fuzz/fuzz_targets/diff_base32.py",
            &serde_json::json!({
                "module": "krux",
                "action": "decode",
                "padding": padding,
                "input": rust_encoded.as_bytes(),
            }),
        )
        .expect("krux decode failed");

        assert_eq!(data, krux_decoded);
    }
});
