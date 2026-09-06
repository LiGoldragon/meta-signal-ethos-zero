use ethos_zero::{File, Generating};
use protos::{Actualizable, Potential};
use std::{env, fs, path::PathBuf};
fn main() {
    println!("cargo:rerun-if-changed=ethos/signal.ethos");
    println!("cargo:rerun-if-changed=src/generated/signal.rs");
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let source = fs::read_to_string(root.join("ethos/signal.ethos")).expect("read Signal source");
    let file = Potential::<File>::from(source)
        .actualize(())
        .expect("parse checked Signal source");
    let generated = file.generate().expect("generate checked Signal Rust");
    let checked_in = fs::read_to_string(root.join("src/generated/signal.rs"))
        .expect("read checked-in Signal Rust");
    assert_eq!(
        generated, checked_in,
        "checked-in Signal Rust is stale; regenerate src/generated/signal.rs before building"
    );
}
