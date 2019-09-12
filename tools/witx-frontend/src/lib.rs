/// Types describing a validated witx document
mod ast;
/// Lexer text into tokens
mod lexer;
/// Witx syntax parsing from SExprs
mod parser;
/// SExpr parsing from tokens
mod sexpr;
/// Resolve toplevel `use` declarations across files
mod toplevel;
/// Validate declarations into ast
mod validate;

pub use ast::{
    AliasDatatype, BuiltinType, Datatype, DatatypeIdent, DatatypeVariant, Definition, Document,
    Entry, EnumDatatype, FlagsDatatype, Id, IntRepr, InterfaceFunc, InterfaceFuncParam, Module,
    ModuleDefinition, ModuleEntry, ModuleImport, ModuleImportVariant, StructDatatype, StructMember,
    UnionDatatype, UnionVariant,
};
pub use lexer::LexError;
pub use parser::{DeclSyntax, ParseError};
pub use sexpr::SExprParseError;
pub use validate::ValidationError;

use failure::Fail;
use std::io;
use std::path::{Path, PathBuf};

/// Location in the source text
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Location {
    pub path: PathBuf,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Fail)]
pub enum WitxError {
    #[fail(display = "{}", _0)]
    SExpr(#[cause] SExprParseError),
    #[fail(display = "when resolving use declaration for {:?}: {}", _0, _1)]
    UseResolution(PathBuf, #[cause] io::Error),
    #[fail(display = "{}", _0)]
    Parse(#[cause] ParseError),
    #[fail(display = "{}", _0)]
    Validation(#[cause] ValidationError),
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Document, WitxError> {
    use toplevel::parse_witx;
    use validate::validate_document;
    let parsed_decls = parse_witx(path)?;
    validate_document(&parsed_decls).map_err(WitxError::Validation)
}
