use std::path::Path;

#[test]
fn validate_wasi_unstable() {
    witx::load(Path::new(
        "../../phases/unstable/witx/wasi_unstable_preview0.witx",
    ))
    .unwrap();
}
