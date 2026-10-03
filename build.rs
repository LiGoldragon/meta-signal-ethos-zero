//! The authored ethos file is held in the generator's own vertical print and
//! its committed Rust byte-identical to what ethos-zero generates from it.
//! The contract source the greeting digests is signal-ethos-zero's Library,
//! which this file imports, followed by this file.

use ethos_zero::{Actualizing, File, Generating, Potential, Printable};

fn main() {
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest"));
    let ethos = root.join("ethos/signal.ethos");
    let rust = root.join("src/generated/signal.rs");
    println!("cargo:rerun-if-changed={}", ethos.display());
    println!("cargo:rerun-if-changed={}", rust.display());
    let source = std::fs::read_to_string(&ethos).expect("source");
    let file = Potential::<File>::from(source.clone())
        .actualize()
        .unwrap_or_else(|_| panic!("read signal.ethos"));
    let body: String = source
        .lines()
        .skip_while(|line| line.starts_with(';'))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_eq!(
        body,
        file.print(),
        "signal.ethos is not in the canonical print"
    );
    let generated = file
        .generate()
        .unwrap_or_else(|_| panic!("generate signal.ethos"));
    assert_eq!(
        generated,
        std::fs::read_to_string(&rust).expect("generated"),
        "src/generated/signal.rs is stale"
    );
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("out"));
    std::fs::write(
        out.join("contract.ethos"),
        format!("{}{source}", signal_ethos_zero::LIBRARY),
    )
    .expect("contract source");
}
