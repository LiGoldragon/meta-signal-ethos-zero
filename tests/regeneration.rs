use ethos_zero::{File, Generating};
use protos::{Actualizable, Potential};
use std::fs;
#[test]
fn generated_signal_is_current_ethos_projection() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(root.join("ethos/signal.ethos")).unwrap();
    let file = Potential::<File>::from(source).actualize(()).unwrap();
    assert_eq!(
        fs::read_to_string(root.join("src/generated/signal.rs")).unwrap(),
        file.generate().unwrap()
    );
}
