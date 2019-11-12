use crate::ast::{Definition, Document};
use crate::io::{Filesystem, WitxIo};
use crate::parser::{TopLevelDocument, TopLevelSyntax};
use crate::validate::DocValidation;
use crate::WitxError;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn parse_witx(i: &[impl AsRef<Path>]) -> Result<Document, WitxError> {
    let paths = i.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>();
    _parse_witx_with(&paths, &Filesystem)
}

pub fn parse_witx_with(i: &[impl AsRef<Path>], witxio: impl WitxIo) -> Result<Document, WitxError> {
    let paths = i.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>();
    _parse_witx_with(&paths, &witxio)
}

fn _parse_witx_with(paths: &[&Path], io: &dyn WitxIo) -> Result<Document, WitxError> {
    let mut validator = DocValidation::new();
    let mut definitions = Vec::new();
    let mut parsed = HashSet::new();
    for path in paths {
        let root = path.parent().unwrap_or(Path::new("."));

        parse_file(
            path.file_name().unwrap().as_ref(),
            io,
            root,
            &mut validator,
            &mut definitions,
            &mut parsed,
        )?;
    }
    Ok(Document::new(definitions, validator.entries))
}

fn parse_file(
    path: &Path,
    io: &dyn WitxIo,
    root: &Path,
    validator: &mut DocValidation,
    definitions: &mut Vec<Definition>,
    parsed: &mut HashSet<PathBuf>,
) -> Result<(), WitxError> {
    let path = io.canonicalize(&root.join(path))?;
    if !parsed.insert(path.clone()) {
        return Ok(());
    }
    let input = io.fgets(&path)?;

    let adjust_err = |mut error: wast::Error| {
        error.set_path(&path);
        error.set_text(&input);
        WitxError::Parse(error)
    };
    let buf = wast::parser::ParseBuffer::new(&input).map_err(adjust_err)?;
    let doc = wast::parser::parse::<TopLevelDocument>(&buf).map_err(adjust_err)?;

    for t in doc.items {
        match t {
            TopLevelSyntax::Decl(d) => {
                let def = validator
                    .scope(&input, &path)
                    .validate_decl(&d)
                    .map_err(WitxError::Validation)?;
                definitions.push(def);
            }
            TopLevelSyntax::Use(u) => {
                parse_file(u.as_ref(), io, root, validator, definitions, parsed)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::*;
    use crate::io::MockFs;

    #[test]
    fn empty() {
        parse_witx_with(&[Path::new("/a")], &MockFs::new(&[("/a", ";; empty")])).expect("parse");
    }

    #[test]
    fn one_use() {
        parse_witx_with(
            &[Path::new("/a")],
            &MockFs::new(&[("/a", "(use \"b\")"), ("/b", ";; empty")]),
        )
        .unwrap();
    }

    #[test]
    fn multi_use() {
        let doc = parse_witx_with(
            &[Path::new("/a")],
            &MockFs::new(&[
                ("/a", "(use \"b\")"),
                ("/b", "(use \"c\")\n(typename $b_float f64)"),
                ("/c", "(typename $c_int u32)"),
            ]),
        )
        .expect("parse");

        match &doc.datatype(&Id::new("b_float")).unwrap().variant {
            DatatypeVariant::Alias(a) => {
                assert_eq!(a.name.as_str(), "b_float");
                assert_eq!(a.to, DatatypeIdent::Builtin(BuiltinType::F64));
            }
            other => panic!("expected alias, got {:?}", other),
        }

        match &doc.datatype(&Id::new("c_int")).unwrap().variant {
            DatatypeVariant::Alias(a) => {
                assert_eq!(a.name.as_str(), "c_int");
                assert_eq!(a.to, DatatypeIdent::Builtin(BuiltinType::U32));
            }
            other => panic!("expected alias, got {:?}", other),
        }
    }

    #[test]
    fn diamond_dependency() {
        let doc = parse_witx_with(
            &[Path::new("/a")],
            &MockFs::new(&[
                ("/a", "(use \"b\")\n(use \"c\")"),
                ("/b", "(use \"d\")"),
                ("/c", "(use \"d\")"),
                ("/d", "(typename $d_char u8)"),
            ]),
        )
        .expect("parse");

        match &doc.datatype(&Id::new("d_char")).unwrap().variant {
            DatatypeVariant::Alias(a) => {
                assert_eq!(a.name.as_str(), "d_char");
                assert_eq!(a.to, DatatypeIdent::Builtin(BuiltinType::U8));
            }
            other => panic!("expected alias, got {:?}", other),
        }
    }

    #[test]
    fn use_not_found() {
        match parse_witx_with(&[Path::new("/a")], &MockFs::new(&[("/a", "(use \"b\")")]))
            .err()
            .unwrap()
        {
            WitxError::Io(path, _error) => assert_eq!(path, PathBuf::from("/b")),
            e => panic!("wrong error: {:?}", e),
        }
    }

    #[test]
    fn use_invalid() {
        match parse_witx_with(&[Path::new("/a")], &MockFs::new(&[("/a", "(use bbbbbbb)")]))
            .err()
            .unwrap()
        {
            WitxError::Parse(e) => {
                let err = e.to_string();
                assert!(err.contains("expected a string"), "bad error: {}", err);
                assert!(err.contains("/a:1:6"));
            }
            e => panic!("wrong error: {:?}", e),
        }
    }
}
