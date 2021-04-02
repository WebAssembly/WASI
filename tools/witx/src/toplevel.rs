use crate::io::{Filesystem, WitxIo};
use crate::parser::{TopLevelModule, TopLevelSyntax};
use crate::validate::{ModuleValidation, ValidationError};
use crate::{Location, Module, WitxError};
use std::collections::{hash_map::Entry, HashMap};
use std::path::{Path, PathBuf};

pub fn parse_witx(i: impl AsRef<Path>) -> Result<Module, WitxError> {
    parse_witx_with(i, Filesystem)
}

pub fn parse_witx_with(i: impl AsRef<Path>, witxio: impl WitxIo) -> Result<Module, WitxError> {
    _parse(i.as_ref(), &witxio, &mut Default::default(), None)
}

fn _parse(
    path: &Path,
    io: &dyn WitxIo,
    parsed: &mut HashMap<PathBuf, Option<Module>>,
    parent_location: Option<Location>,
) -> Result<Module, WitxError> {
    let canon_path = io.canonicalize(path)?;
    match parsed.entry(canon_path.clone()) {
        Entry::Occupied(e) => match e.get() {
            Some(doc) => return Ok(doc.clone()),
            None => {
                return Err(ValidationError::CyclicModule {
                    location: parent_location.unwrap(),
                }
                .into())
            }
        },
        Entry::Vacant(v) => {
            v.insert(None);
        }
    }

    let input = io.fgets(&canon_path)?;
    let mut validator = ModuleValidation::new(&input, path);

    let adjust_err = |mut error: wast::Error| {
        error.set_path(&path);
        error.set_text(&input);
        WitxError::Parse(error)
    };
    let buf = wast::parser::ParseBuffer::new(&input).map_err(adjust_err)?;
    let doc = wast::parser::parse::<TopLevelModule>(&buf).map_err(adjust_err)?;

    let mut submodules = HashMap::new();
    for t in doc.decls {
        match t.item {
            TopLevelSyntax::Decl(d) => {
                validator
                    .validate_decl(&d, &t.comments)
                    .map_err(WitxError::Validation)?;
            }
            TopLevelSyntax::Use(u) => {
                let path = path
                    .parent()
                    .unwrap()
                    .join(u.from.name())
                    .with_extension("witx");

                let module = match submodules.entry(u.from.name()) {
                    Entry::Occupied(e) => e.into_mut(),
                    Entry::Vacant(v) => {
                        let location = validator.location(u.from.span());
                        let doc = _parse(&path, io, parsed, Some(location))?;
                        v.insert(doc)
                    }
                };
                validator.validate_use(u, module)?;
            }
        }
    }
    for f in doc.functions {
        validator
            .validate_function(&f.item, &f.comments)
            .map_err(WitxError::Validation)?;
    }

    let doc = validator.into_module();
    parsed.insert(canon_path, Some(doc.clone()));
    Ok(doc)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::*;
    use crate::io::MockFs;

    #[test]
    fn empty() {
        parse_witx_with("/a", &MockFs::new(&[("/a", ";; empty")])).expect("parse");
    }

    #[test]
    fn one_use() {
        parse_witx_with(
            "/a",
            &MockFs::new(&[("/a", "(use $x from $b)"), ("/b.witx", "(typename $x u8)")]),
        )
        .unwrap();
    }

    #[test]
    fn multi_use() {
        let doc = parse_witx_with(
            "/a",
            &MockFs::new(&[
                ("/a", "(use $b_float $c_int from $b)"),
                ("/b.witx", "(use $c_int from $c)\n(typename $b_float f64)"),
                ("/c.witx", "(typename $c_int u32)"),
            ]),
        )
        .expect("parse");

        let b_float = doc.typename(&Id::new("b_float")).unwrap();
        assert_eq!(**b_float.type_(), Type::Builtin(BuiltinType::F64));

        let c_int = doc.typename(&Id::new("c_int")).unwrap();
        assert_eq!(
            **c_int.type_(),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: false
            })
        );
    }

    #[test]
    fn diamond_dependency() {
        let doc = parse_witx_with(
            "/a",
            &MockFs::new(&[
                ("/a", "(use $b_char from $b)\n(use $c_char from $c)"),
                (
                    "/b.witx",
                    "(use $d_char from $d) (typename $b_char $d_char)",
                ),
                (
                    "/c.witx",
                    "(use $d_char from $d) (typename $c_char $d_char)",
                ),
                ("/d.witx", "(typename $d_char u8)"),
            ]),
        )
        .expect("parse");

        let b_char = doc.typename(&Id::new("b_char")).unwrap();
        assert_eq!(
            **b_char.type_(),
            Type::Builtin(BuiltinType::U8 { lang_c_char: false })
        );
        assert!(doc.typename(&Id::new("d_char")).is_none());
    }

    #[test]
    fn use_not_found() {
        match parse_witx_with("/a", &MockFs::new(&[("/a", "(use $x from $b)")]))
            .err()
            .unwrap()
        {
            WitxError::Io(path, _error) => assert_eq!(path, PathBuf::from("/b.witx")),
            e => panic!("wrong error: {:?}", e),
        }
    }
}
