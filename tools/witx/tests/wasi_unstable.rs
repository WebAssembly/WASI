use std::path::Path;
use witx;

#[test]
fn validate_wasi_unstable_preview0() {
    witx::load(Path::new(
        "../../phases/unstable/witx/wasi_unstable_preview0.witx",
    ))
    .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn validate_wasi_ephemeral_preview() {
    witx::load(Path::new(
        "../../phases/ephemeral/witx/wasi_ephemeral_preview.witx",
    ))
    .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn validate_wasi_old_preview0() {
    witx::load(Path::new("../../phases/old/witx/wasi_unstable.witx"))
        .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn render_roundtrip() {
    let doc = witx::load(Path::new(
        "../../phases/unstable/witx/wasi_unstable_preview0.witx",
    ))
    .unwrap_or_else(|e| panic!("failed to parse: {}", e));

    let back_to_sexprs = format!("{}", doc);
    println!("{}", back_to_sexprs);
    let doc2 = witx::parse(&back_to_sexprs)
        .map_err(|e| e.report_with(&witx::MockFs::new(&[("-", &back_to_sexprs)])))
        .unwrap();

    assert_eq!(doc, doc2);
}
