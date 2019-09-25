use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use witx::{load, Document, WitxError};

const WASI_VERSIONS: &[(&str, &str)] = &[("unstable", "preview0")];

fn load_wasi_spec(phase: &str, version: &str) -> Result<Document, WitxError> {
    let path = format!(
        "../../phases/{phase}/witx/wasi_{phase}_{version}.witx",
        phase = phase,
        version = version,
    );
    println!("cargo:rerun-if-changed={}", path);
    load(Path::new(&path))
}

fn serialize_wasi_spec(phase: &str, version: &str, doc: &Document) {
    let out_dir = env::var("OUT_DIR").expect("cargo outdir available");
    let witx_path = Path::new(&out_dir).join(format!("wasi_{}_{}.witx", phase, version));
    let mut f = File::create(witx_path).expect("create file");
    f.write_all(format!("{}", doc).as_bytes())
        .expect("write data");
}

fn main() {
    for (phase, version) in WASI_VERSIONS {
        let doc =
            load_wasi_spec(phase, version).expect(&format!("load wasi {} {}", phase, version));
        serialize_wasi_spec(phase, version, &doc);
    }
}
