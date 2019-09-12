pub use crate::lexer::LexError;
use crate::lexer::{Lexer, LocatedError, LocatedToken, Token};
use crate::Location;
use failure::Fail;
use std::path::{Path, PathBuf};

///! The s-expression parser turns a string into a stream of SExprs.
///! It uses the `Lexer` under the hood.
///! This implementation was heavily influenced by `cranelift-reader`

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExpr<'a> {
    Vec(Vec<SExpr<'a>>, Location),
    Word(&'a str, Location),
    Ident(&'a str, Location),
    Quote(&'a str, Location),
    /// Short for Annotation
    Annot(&'a str, Location),
}

impl<'a> SExpr<'a> {
    pub fn location(&self) -> Location {
        match self {
            SExpr::Vec(_, loc) => loc.clone(),
            SExpr::Word(_, loc) => loc.clone(),
            SExpr::Ident(_, loc) => loc.clone(),
            SExpr::Quote(_, loc) => loc.clone(),
            SExpr::Annot(_, loc) => loc.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Fail)]
pub enum SExprParseError {
    #[fail(display = "Lexical error at {:?}: {}", _1, _0)]
    Lex(LexError, Location),
    #[fail(display = "Unexpected ')' at {:?}", _0)]
    UnexpectedCloseParen(Location),
    #[fail(display = "Unexpected end of input in {:?}", _0)]
    UnexpectedEof(PathBuf),
}

pub struct SExprParser<'a> {
    lex: Lexer<'a>,
    lookahead: Option<Token<'a>>,
    location: Location,
}

impl<'a> SExprParser<'a> {
    pub fn new<P: AsRef<Path>>(text: &'a str, path: P) -> SExprParser<'_> {
        SExprParser {
            lex: Lexer::new(text, path.as_ref()),
            lookahead: None,
            location: Location {
                path: path.as_ref().into(),
                line: 0,
                column: 0,
            },
        }
    }
    fn consume(&mut self) -> Token<'a> {
        self.lookahead.take().expect("no token to consume")
    }
    fn token(&mut self) -> Result<Option<Token<'a>>, SExprParseError> {
        while self.lookahead == None {
            match self.lex.next() {
                Some(Ok(LocatedToken { token, location })) => {
                    self.location = location;
                    self.lookahead = Some(token)
                }
                Some(Err(LocatedError { error, location })) => {
                    self.location = location.clone();
                    Err(SExprParseError::Lex(error, location))?;
                }
                None => break,
            }
        }
        Ok(self.lookahead)
    }

    pub fn match_sexpr(&mut self) -> Result<SExpr<'a>, SExprParseError> {
        let location = self.location.clone();
        match self.token()? {
            Some(Token::LPar) => {
                self.consume();
                let mut members = Vec::new();
                loop {
                    match self.token()? {
                        Some(Token::RPar) => {
                            self.consume();
                            break;
                        }
                        _ => {
                            members.push(self.match_sexpr()?);
                        }
                    }
                }
                Ok(SExpr::Vec(members, location))
            }
            Some(Token::Word(word)) => {
                self.consume();
                Ok(SExpr::Word(word, location))
            }
            Some(Token::Ident(id)) => {
                self.consume();
                Ok(SExpr::Ident(id, location))
            }
            Some(Token::Annot(id)) => {
                self.consume();
                Ok(SExpr::Annot(id, location))
            }
            Some(Token::Quote(q)) => {
                self.consume();
                Ok(SExpr::Quote(q, location))
            }
            Some(Token::RPar) => Err(SExprParseError::UnexpectedCloseParen(location)),
            None => Err(SExprParseError::UnexpectedEof(self.location.path.clone())),
        }
    }

    pub fn match_sexprs(&mut self) -> Result<Vec<SExpr<'a>>, SExprParseError> {
        let mut sexprs = Vec::new();
        while self.token()?.is_some() {
            sexprs.push(self.match_sexpr()?);
        }
        Ok(sexprs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn loc(line: usize, col: usize) -> Location {
        Location {
            path: PathBuf::from("/test"),
            line: line,
            column: col,
        }
    }

    fn testparser(input: &str) -> SExprParser {
        SExprParser::new(input, Path::new("/test"))
    }

    #[test]
    fn empty() {
        let mut parser = testparser("");
        assert_eq!(parser.match_sexprs().expect("valid parse"), Vec::new());
        let mut parser = testparser("   ;; just a comment\n;;another");
        assert_eq!(parser.match_sexprs().expect("valid parse"), Vec::new());
    }

    #[test]
    fn atoms() {
        let mut parser = testparser("hello\n$world\n\"a quotation\"");
        assert_eq!(
            parser.match_sexprs().expect("valid parse"),
            vec![
                SExpr::Word("hello", loc(1, 0)),
                SExpr::Ident("world", loc(2, 0)),
                SExpr::Quote("a quotation", loc(3, 0)),
            ]
        );
    }

    #[test]
    fn lists() {
        let mut parser = testparser("()");
        assert_eq!(
            parser.match_sexprs().expect("valid parse"),
            vec![SExpr::Vec(vec![], loc(1, 0))]
        );

        let mut parser = testparser("(hello\n$world\n\"a quotation\")");
        assert_eq!(
            parser.match_sexprs().expect("valid parse"),
            vec![SExpr::Vec(
                vec![
                    SExpr::Word("hello", loc(1, 1)),
                    SExpr::Ident("world", loc(2, 0)),
                    SExpr::Quote("a quotation", loc(3, 0)),
                ],
                loc(1, 0)
            )]
        );

        let mut parser = testparser("((($deep)))");
        assert_eq!(
            parser.match_sexprs().expect("valid parse"),
            vec![SExpr::Vec(
                vec![SExpr::Vec(
                    vec![SExpr::Vec(vec![SExpr::Ident("deep", loc(1, 3))], loc(1, 2))],
                    loc(1, 1)
                )],
                loc(1, 0)
            )]
        );
    }

    #[test]
    fn errors() {
        let mut parser = testparser("(");
        assert_eq!(
            parser.match_sexprs().err().expect("dies"),
            SExprParseError::UnexpectedEof(PathBuf::from("/test"))
        );
        let mut parser = testparser(")");
        assert_eq!(
            parser.match_sexprs().err().expect("dies"),
            SExprParseError::UnexpectedCloseParen(loc(1, 0))
        );
        let mut parser = testparser("())");
        assert_eq!(
            parser.match_sexprs().err().expect("dies"),
            SExprParseError::UnexpectedCloseParen(loc(1, 2))
        );
        let mut parser = testparser("$ ;; should be a lex error");
        assert_eq!(
            parser.match_sexprs().err().expect("dies"),
            SExprParseError::Lex(LexError::EmptyIdentifier, loc(1, 0),),
        );
    }
}
