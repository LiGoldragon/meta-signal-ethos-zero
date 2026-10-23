//! Freshness compares the complete Rust projection from the pinned producer,
//! not the contract's source digest. Mutations stay in memory.

use ethos_zero::{Actualizing, File, Generating, Potential};

const SOURCE: &str = include_str!("../ethos/signal.ethos");
const ARTIFACT: &str = include_str!("../src/generated/signal.rs");

#[test]
fn the_committed_artifact_is_the_pinned_producers_complete_output() {
    let file = Potential::<File>::from(SOURCE.to_owned())
        .actualize()
        .expect("read authored Signal");
    let generated = file.generate().expect("generate authored Signal");
    assert_eq!(generated, ARTIFACT);
}

#[test]
fn a_changed_authored_type_makes_the_committed_artifact_stale() {
    let source = SOURCE.replacen("SourceManifestPath.String", "SourceManifestPath.Integer", 1);
    assert_ne!(source, SOURCE, "the source mutation must take effect");
    let file = Potential::<File>::from(source)
        .actualize()
        .expect("read changed Signal");
    let generated = file.generate().expect("generate changed Signal");
    assert_ne!(generated, ARTIFACT, "old Rust must not pass freshness");
}

#[test]
fn a_changed_rust_type_is_stale_even_with_unchanged_authored_source() {
    let artifact = ARTIFACT.replacen(
        "pub type SourceManifestPath = String;",
        "pub type SourceManifestPath = bool;",
        1,
    );
    assert_ne!(artifact, ARTIFACT, "the artifact mutation must take effect");
    let file = Potential::<File>::from(SOURCE.to_owned())
        .actualize()
        .expect("read unchanged Signal");
    let generated = file.generate().expect("generate unchanged Signal");
    assert_eq!(generated, ARTIFACT, "the original artifact is fresh");
    assert_ne!(generated, artifact, "edited Rust must not pass freshness");
}
