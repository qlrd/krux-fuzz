use prepare_krux_env::run;
use std::error::Error;

/// main
///
/// Download both diybitcoinhardware/embit and selfcustody/krux sources
fn main() -> Result<(), Box<dyn Error>> {
    run(
        "https://github.com/diybitcoinhardware/embit/raw/master/",
        "diybitcoinhardware",
        vec![
            "https://github.com/diybitcoinhardware/embit/raw/master/src/embit/util/key.py",
            "https://github.com/diybitcoinhardware/embit/raw/master/src/embit/util/ctypes_secp256k1.py",
            "https://github.com/diybitcoinhardware/embit/raw/master/src/embit/util/py_secp256k1.py",
            "https://github.com/diybitcoinhardware/embit/raw/master/src/embit/util/secp256k1.py",
            "https://github.com/diybitcoinhardware/embit/raw/master/src/embit/misc.py",
            "https://github.com/diybitcoinhardware/embit/raw/master/src/embit/bip39.py",
            "https://github.com/diybitcoinhardware/embit/raw/master/src/embit/wordlists/bip39.py",
        ],
    )?;

    run(
        "https://github.com/selfcustody/krux/raw/main/",
        "selfcustody",
        vec![
            "https://github.com/selfcustody/krux/raw/main/src/krux/bbqr.py",
            "https://github.com/selfcustody/krux/raw/main/src/krux/bip39.py",
        ],
    )?;

    run(
        "https://github.com/coinkite/BBQr/raw/master/",
        "BBQr",
        vec!["https://github.com/coinkite/BBQr/raw/master/python/bbqr/utils.py"],
    )
}
