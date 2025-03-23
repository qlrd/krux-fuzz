use prepare_krux_env::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = "selfcustody";
    let urls = vec![
        "https://github.com/selfcustody/krux/raw/main/src/krux/bbqr.py",
        "https://github.com/selfcustody/krux/raw/main/src/krux/bip39.py",
    ];

    run(output_dir, urls)
}
