use std::fs;
use std::path::Path;
use witx::{self, Documentation};

#[test]
fn validate_wasi_snapshot() {
    witx::load(&witx::phases::snapshot().unwrap())
        .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn validate_wasi_ephemeral() {
    witx::load(&witx::phases::ephemeral().unwrap())
        .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn validate_wasi_old_snapshot_0() {
    witx::load(&witx::phases::old::snapshot_0().unwrap())
        .unwrap_or_else(|e| panic!("failed to parse: {}", e));
}

#[test]
fn validate_docs() {
    for phase in &[
        witx::phases::snapshot().unwrap(),
        witx::phases::ephemeral().unwrap(),
        witx::phases::old::snapshot_0().unwrap(),
    ] {
        let doc = witx::load(&phase).unwrap_or_else(|e| panic!("failed to parse: {}", e));
        diff_against_filesystem(&doc.to_md(), &witx::phases::docs_path(&phase));
    }
}

#[test]
fn render_roundtrip() {
    let doc = witx::load(&witx::phases::snapshot().unwrap())
        .unwrap_or_else(|e| panic!("failed to parse: {}", e));

    let back_to_sexprs = format!("{}", doc);
    println!("{}", back_to_sexprs);

    let doc2 = witx::parse(&back_to_sexprs)
        .map_err(|e| e.report_with(&witx::MockFs::new(&[("-", &back_to_sexprs)])))
        .unwrap();

    // I'd just assert_eq, but when it fails the debug print is thousands of lines long and impossible
    // to figure out where they are unequal.
    if doc != doc2 {
        for type_ in doc.typenames() {
            let type2 = doc2.typename(&type_.name).expect("doc2 missing datatype");
            assert_eq!(type_, type2);
        }
        for mod_ in doc.modules() {
            let mod2 = doc2.module(&mod_.name).expect("doc2 missing module");
            for import in mod_.imports() {
                let import2 = mod2.import(&import.name).expect("mod2 missing import");
                assert_eq!(import, import2);
            }
            for func in mod_.funcs() {
                let func2 = mod2.func(&func.name).expect("mod2 missing func");
                assert_eq!(func, func2);
            }
        }
    }
    // This should be equivalent to the above, but just in case some code changes where it isnt:
    assert_eq!(doc, doc2);
}

#[test]
fn document_wasi_snapshot() {
    use witx::Documentation;
    println!(
        "{}",
        witx::load(&witx::phases::snapshot().unwrap())
            .unwrap_or_else(|e| panic!("failed to parse: {}", e))
            .to_md()
    );
}

fn dos2unix(s: &str) -> String {
    let mut t = String::new();
    t.reserve(s.len());
    for c in s.chars() {
        if c != '\r' {
            t.push(c)
        }
    }
    t
}

fn diff_against_filesystem(expected: &str, path: &Path) {
    let actual = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("couldn't read {}: {:?}", Path::display(path), e));
    // Git may checkout the file with dos line endings on windows. Strip all \r:
    let actual = dos2unix(&actual);
    if &actual == expected {
        return;
    }

    eprintln!("The following diff was found between the docs generated from .witx and the");
    eprintln!("source {:?} in the tree:", path);
    eprintln!();

    let mut expected_line = 1;
    let mut actual_line = 1;
    let mut separated = false;
    let mut any_lines = false;
    for diff in diff::lines(&expected, &actual) {
        match diff {
            diff::Result::Left(l) => {
                eprintln!("line {}: -{}", expected_line, l);
                expected_line += 1;
                separated = false;
                any_lines = true;
            }
            diff::Result::Both(_, _) => {
                expected_line += 1;
                actual_line += 1;
                if !separated {
                    eprintln!("...");
                    separated = true;
                }
            }
            diff::Result::Right(r) => {
                eprintln!("line {}: +{}", actual_line, r);
                actual_line += 1;
                separated = false;
                any_lines = true;
            }
        }
    }

    if !any_lines {
        eprintln!();
        eprintln!(
            "Somehow there was a diff with no lines differing. Lengths: {} and {}.",
            expected.len(),
            actual.len()
        );
        for (index, (a, b)) in actual.chars().zip(expected.chars()).enumerate() {
            if a != b {
                eprintln!("char difference at index {}: '{}' != '{}'", index, a, b);
            }
        }
        for (index, (a, b)) in actual.bytes().zip(expected.bytes()).enumerate() {
            if a != b {
                eprintln!("byte difference at index {}: b'{}' != b'{}'", index, a, b);
            }
        }
        eprintln!();
        eprintln!("actual: {}", actual);
        eprintln!();
        eprintln!("expected: {}", expected);
    }

    eprintln!();
    eprintln!(
        "To regenerate the files, run `cd tools/witx && cargo run --example witx repo-docs`."
    );
    eprintln!();
    panic!();
}
