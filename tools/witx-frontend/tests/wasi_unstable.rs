use std::path::Path;
use witx_frontend;

#[test]
fn validate_wasi_unstable() {
    witx_frontend::load(Path::new("../../design/wasi_unstable/wasi_unstable.witx")).unwrap()
}
