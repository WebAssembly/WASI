use std::path::Path;
use witx;

#[test]
fn validate_wasi_unstable_preview0() {
    witx::load(Path::new(
        "../../phases/unstable/witx/wasi_unstable_preview0.witx",
    ))
    .unwrap();
}

#[test]
fn validate_wasi_ephemeral_preview0() {
    witx::load(Path::new(
        "../../phases/ephemeral/witx/wasi_ephemeral_preview0.witx",
    ))
    .unwrap();
}

#[test]
fn validate_wasi_old_preview0() {
    witx::load(Path::new("../../phases/old/witx/wasi_unstable.witx")).unwrap();
}
