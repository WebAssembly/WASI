/// Types describing a validated witx document
mod ast;
/// Interface for filesystem or mock IO
mod io;
/// Lexer text into tokens
mod lexer;
/// Determine module type of function
mod moduletype;
/// Witx syntax parsing from SExprs
mod parser;
/// Render ast to text
mod render;
/// SExpr parsing from tokens
mod sexpr;
/// Resolve toplevel `use` declarations across files
mod toplevel;
/// Validate declarations into ast
mod validate;

pub use ast::{
    AliasDatatype, BuiltinType, Datatype, DatatypeIdent, DatatypeVariant, Definition, Document,
    Entry, EnumDatatype, FlagsDatatype, Id, IntRepr, InterfaceFunc, InterfaceFuncParam,
    InterfaceFuncParamPosition, Module, ModuleDefinition, ModuleEntry, ModuleImport,
    ModuleImportVariant, StructDatatype, StructMember, UnionDatatype, UnionVariant,
};
pub use io::{Filesystem, MockFs, WitxIo};
pub use lexer::LexError;
pub use moduletype::{
    AtomType, DatatypePassedBy, ModuleFuncType, ModuleParamSignifies, ModuleParamType,
};
pub use parser::{DeclSyntax, ParseError};
pub use render::{Render, SExpr as RenderSExpr};
pub use sexpr::SExprParseError;
pub use validate::ValidationError;

use failure::Fail;
use std::path::{Path, PathBuf};

/// Load a witx document from the filesystem
pub fn load<P: AsRef<Path>>(path: P) -> Result<Document, WitxError> {
    use toplevel::parse_witx;
    use validate::validate_document;
    let parsed_decls = parse_witx(path)?;
    validate_document(&parsed_decls).map_err(WitxError::Validation)
}

/// Parse a witx document from a str. `(use ...)` directives are not permitted.
pub fn parse(source: &str) -> Result<Document, WitxError> {
    use toplevel::parse_witx_with;
    use validate::validate_document;
    let mockfs = MockFs::new(&[("-", source)]);
    let parsed_decls = parse_witx_with(Path::new("-"), &mockfs)?;
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
    #[fail(display = "with file {:?}: {}", _0, _1)]
    Io(PathBuf, #[cause] ::std::io::Error),
    #[fail(display = "{}", _0)]
    Parse(#[cause] ParseError),
    #[fail(display = "{}", _0)]
    Validation(#[cause] ValidationError),
}

impl WitxError {
    pub fn report_with(&self, witxio: &dyn WitxIo) -> String {
        use WitxError::*;
        match self {
            SExpr(sexpr) => sexpr.report_with(witxio),
            Io(path, ioerr) => format!("with file {:?}: {}", path, ioerr),
            Parse(parse) => parse.report_with(witxio),
            Validation(validation) => validation.report_with(witxio),
        }
    }
    pub fn report(&self) -> String {
        self.report_with(&Filesystem)
    }
}

impl Location {
    pub fn highlight_source_with(&self, witxio: &dyn WitxIo) -> String {
        let mut msg = format!("in {:?}:\n", self.path);
        if let Ok(src_line) = witxio.fget_line(&self.path, self.line) {
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
    pub fn highlight_source(&self) -> String {
        self.highlight_source_with(&Filesystem)
    }
}
