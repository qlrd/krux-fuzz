#![no_main]
use bip39::{Language, Mnemonic};
use call_python_script::run;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let script = "fuzz/fuzz_targets/diff_bip39.py";

    // first create some mnemonic from a arbitrary entropy
    if let Ok(mnemonic) = Mnemonic::from_entropy_in(Language::English, &data) {
        // convert the list into words
        let words: String = mnemonic
            .words()
            .enumerate()
            .map(|(_i, word)| word)
            .collect::<Vec<_>>()
            .join(" ");

        // now check if embit produces the same mnemonic
        let embit_words: String = run(
            script,
            &serde_json::json!({
                "action": "mnemonic_from_bytes",
                "module": "embit",
                "input": &data
            }),
        )
        .expect("embit.bip39.mnemonic_from_bytes failed");

        assert_eq!(words, embit_words);

        // last, check if krux can produce back compatible
        // the same entropy, as an vector of bytes,
        // from the given mnemonic produced by rust
        let krux_mnemonic_bytes: Vec<u8> = run(
            script,
            &serde_json::json!({
                "action": "k_mnemonic_bytes",
                "module": "krux",
                "input": &words
            }),
        )
        .expect("krux.bip39.k_mnemonic_bytes failed");

        assert_eq!(&data, &krux_mnemonic_bytes);
    }
});
