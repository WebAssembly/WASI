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

pub fn load<P: AsRef<Path>>(path: P) -> Result<Document, WitxError> {
    use toplevel::parse_witx;
    use validate::validate_document;
    let parsed_decls = parse_witx(path)?;
    validate_document(&parsed_decls).map_err(WitxError::Validation)
}

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

impl WitxError {
    pub fn report(&self) -> String {
        use WitxError::*;
        match self {
            SExpr(sexpr) => sexpr.report(),
            UseResolution(path, ioerr) => format!("when resolving `use {:?}`: {}", path, ioerr),
            Parse(parse) => parse.report(),
            Validation(validation) => validation.report(),
        }
    }
}
impl Location {
    pub fn highlight_source(&self) -> String {
        let mut msg = format!("in {:?}:\n", self.path);
        if let Ok(src_line) = self.source_line() {
            msg += &format!(
                "{line_num: >5} | {src_line}\n{blank: >5}    {caret: >column$}",
                line_num = self.line,
                src_line = src_line,
                blank = " ",
                caret = "^",
                column = self.column,
            );
        }
        msg
    }
    pub fn source_line(&self) -> Result<String, io::Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let f = BufReader::new(File::open(&self.path)?);
        let l = f
            .lines()
            .skip(self.line - 1)
            .next()
            .unwrap_or_else(|| Err(io::Error::new(io::ErrorKind::Other, "TODO")))?;
        Ok(l)
    }
}
