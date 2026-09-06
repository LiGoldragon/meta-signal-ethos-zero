use ethos_zero::{File, Generating};
use protos::{Actualizable, Potential};
use std::{fs, path::PathBuf};
fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(root.join("ethos/signal.ethos")).unwrap();
    let file = Potential::<File>::from(source).actualize(()).unwrap();
    fs::write(
        root.join("src/generated/signal.rs"),
        file.generate().unwrap(),
    )
    .unwrap();
}
