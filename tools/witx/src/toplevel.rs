use crate::io::{Filesystem, WitxIo};
use crate::parser::{DeclSyntax, ParseError, TopLevelSyntax};
use crate::sexpr::SExprParser;
use crate::WitxError;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn parse_witx<P: AsRef<Path>>(i: P) -> Result<Vec<DeclSyntax>, WitxError> {
    parse_witx_with(i, &Filesystem)
}

pub fn parse_witx_with<P: AsRef<Path>>(
    i: P,
    witxio: &dyn WitxIo,
) -> Result<Vec<DeclSyntax>, WitxError> {
    let input_path = witxio.canonicalize(&i.as_ref())?;

    let input = witxio.fgets(&input_path)?;

    let toplevel = parse_toplevel(&input, &input_path)?;
    let mut resolved = HashSet::new();
    resolved.insert(input_path.clone());
    let search_path = input_path.parent().unwrap_or(Path::new("."));
    resolve_uses(toplevel, &search_path, &mut resolved, witxio)
}

fn parse_toplevel(source_text: &str, file_path: &Path) -> Result<Vec<TopLevelSyntax>, WitxError> {
    let mut sexpr_parser = SExprParser::new(source_text, file_path);
    let sexprs = sexpr_parser.match_sexprs().map_err(WitxError::SExpr)?;
    let top_levels = sexprs
        .iter()
        .map(|s| TopLevelSyntax::parse(s))
        .collect::<Result<Vec<TopLevelSyntax>, ParseError>>()
        .map_err(WitxError::Parse)?;
    Ok(top_levels)
}

fn resolve_uses(
    toplevel: Vec<TopLevelSyntax>,
    search_path: &Path,
    used: &mut HashSet<PathBuf>,
    witxio: &dyn WitxIo,
) -> Result<Vec<DeclSyntax>, WitxError> {
    let mut decls = Vec::new();

    for t in toplevel {
        match t {
            TopLevelSyntax::Decl(d) => decls.push(d),
            TopLevelSyntax::Use(u) => {
                let abs_path = witxio.canonicalize(&search_path.join(u.name))?;
                // Include the decls from a use declaration only once
                // in a given toplevel. Same idea as #pragma once.
                if !used.contains(&abs_path) {
                    used.insert(abs_path.clone());

                    let source_text = witxio.fgets(&abs_path)?;
                    let inner_toplevels = parse_toplevel(&source_text, &abs_path)?;

                    let inner_decls = resolve_uses(inner_toplevels, search_path, used, witxio)?;
                    decls.extend(inner_decls)
                }
            }
        }
    }

    Ok(decls)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::io::MockFs;
    use crate::parser::*;
    use crate::Location;

    #[test]
    fn empty() {
        assert_eq!(
            parse_witx_with(&Path::new("/a"), &MockFs::new(&[("/a", ";; empty")])).expect("parse"),
            Vec::new(),
        );
    }

    #[test]
    fn one_use() {
        assert_eq!(
            parse_witx_with(
                &Path::new("/a"),
                &MockFs::new(&[("/a", "(use \"b\")"), ("/b", ";; empty")])
            )
            .expect("parse"),
            Vec::new(),
        );
    }

    #[test]
    fn multi_use() {
        assert_eq!(
            parse_witx_with(
                &Path::new("/a"),
                &MockFs::new(&[
                    ("/a", "(use \"b\")"),
                    ("/b", "(use \"c\")\n(typename $b_float f64)"),
                    ("/c", "(typename $c_int u32)")
                ])
            )
            .expect("parse"),
            vec![
                DeclSyntax::Typename(TypenameSyntax {
                    ident: IdentSyntax {
                        name: "c_int".to_owned(),
                        location: Location {
                            path: PathBuf::from("/c"),
                            line: 1,
                            column: 10,
                        }
                    },
                    def: TypedefSyntax::Ident(DatatypeIdentSyntax::Builtin(BuiltinType::U32))
                }),
                DeclSyntax::Typename(TypenameSyntax {
                    ident: IdentSyntax {
                        name: "b_float".to_owned(),
                        location: Location {
                            path: PathBuf::from("/b"),
                            line: 2,
                            column: 10,
                        }
                    },
                    def: TypedefSyntax::Ident(DatatypeIdentSyntax::Builtin(BuiltinType::F64))
                })
            ],
        );
    }

    #[test]
    fn diamond_dependency() {
        assert_eq!(
            parse_witx_with(
                &Path::new("/a"),
                &MockFs::new(&[
                    ("/a", "(use \"b\")\n(use \"c\")"),
                    ("/b", "(use \"d\")"),
                    ("/c", "(use \"d\")"),
                    ("/d", "(typename $d_char u8)")
                ])
            )
            .expect("parse"),
            vec![DeclSyntax::Typename(TypenameSyntax {
                ident: IdentSyntax {
                    name: "d_char".to_owned(),
                    location: Location {
                        path: PathBuf::from("/d"),
                        line: 1,
                        column: 10,
                    }
                },
                def: TypedefSyntax::Ident(DatatypeIdentSyntax::Builtin(BuiltinType::U8))
            })],
        );
    }

    #[test]
    fn use_not_found() {
        match parse_witx_with(&Path::new("/a"), &MockFs::new(&[("/a", "(use \"b\")")]))
            .err()
            .unwrap()
        {
            WitxError::Io(path, _error) => assert_eq!(path, PathBuf::from("/b")),
            e => panic!("wrong error: {:?}", e),
        }
    }

    #[test]
    fn use_invalid() {
        match parse_witx_with(&Path::new("/a"), &MockFs::new(&[("/a", "(use bbbbbbb)")]))
            .err()
            .unwrap()
        {
            WitxError::Parse(e) => {
                assert_eq!(e.message, "invalid use declaration");
                assert_eq!(
                    e.location,
                    Location {
                        path: PathBuf::from("/a"),
                        line: 1,
                        column: 1
                    }
                );
            }
            e => panic!("wrong error: {:?}", e),
        }
    }
}
