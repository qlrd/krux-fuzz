#![no_main]
use bip39::{Language, Mnemonic};
use call_python_script;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(mnemonic) = Mnemonic::from_entropy_in(Language::English, &data) {
        let python_mnemonic_valid: bool = call_python_script::run(
            "fuzz/fuzz_targets/diff_bip39.py",
            &serde_json::json!({"action": "is_valid", "input": mnemonic}),
        )
        .expect("Python encode failed");

        if !python_mnemonic_valid {
            println!("entropy: {:?}", &data);
            println!("mnemonic: {}", mnemonic);
        }
        assert_eq!(python_mnemonic_valid, true);
    }
});
