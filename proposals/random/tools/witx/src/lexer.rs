use crate::Location;
use failure::Fail;
use std::path::{Path, PathBuf};
use std::str::CharIndices;

///! The lexer turns a string into a stream of located tokens.
///! The tokens are meant for consumption by the s-expression parser.
///!
///! Comments in source text look like `;; rest of line ...`.
///! Words look like `abcde_`
///! Idents look like `$abcde_`
///! Annotations look like `@abcde_`
///! Quotes look like `"a b cde 123 @#$%^&*() _"`
///!
///! This implementation was heavily influenced by `cranelift-reader`

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token<'a> {
    LPar,           // (
    RPar,           // )
    Word(&'a str),  // Bare word
    Ident(&'a str), // Starts with $
    Annot(&'a str), // Starts with @. short for annotation.
    Quote(&'a str), // Found between balanced "". No escaping.
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocatedToken<'a> {
    pub token: Token<'a>,
    pub location: Location,
}

fn token(token: Token<'_>, location: Location) -> Result<LocatedToken<'_>, LocatedError> {
    Ok(LocatedToken { token, location })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Fail)]
pub enum LexError {
    #[fail(display = "Invalid character '{}'", _0)]
    InvalidChar(char),
    #[fail(display = "Empty identifier '$'")]
    EmptyIdentifier,
    #[fail(display = "Empty annotation '@'")]
    EmptyAnnotation,
    #[fail(display = "Unterminated quote")]
    UnterminatedQuote,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocatedError {
    pub error: LexError,
    pub location: Location,
}

fn error<'a>(error: LexError, location: Location) -> Result<LocatedToken<'a>, LocatedError> {
    Err(LocatedError { error, location })
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    lookahead: Option<char>,
    pos: usize,
    line_number: usize,
    column_start: usize,
    tab_compensation: usize,
    path: PathBuf,
}

impl<'a> Lexer<'a> {
    pub fn new<P: AsRef<Path>>(s: &'a str, path: P) -> Lexer<'_> {
        let mut lex = Lexer {
            source: s,
            chars: s.char_indices(),
            lookahead: None,
            pos: 0,
            line_number: 1,
            column_start: 0,
            tab_compensation: 0,
            path: path.as_ref().into(),
        };
        lex.next_ch();
        lex
    }

    fn next_ch(&mut self) -> Option<char> {
        if self.lookahead == Some('\n') {
            self.line_number += 1;
            self.column_start = self.pos + 1; // Next column starts a fresh line
            self.tab_compensation = 0;
        } else if self.lookahead == Some('\t') {
            self.tab_compensation += 7; // One column for the position of the char itself, add 7 more for a tabwidth of 8
        }
        match self.chars.next() {
            Some((idx, ch)) => {
                self.pos = idx;
                self.lookahead = Some(ch);
            }
            None => {
                self.pos = self.source.len();
                self.lookahead = None;
            }
        }
        self.lookahead
    }

    fn loc(&self) -> Location {
        Location {
            path: self.path.clone(),
            line: self.line_number,
            column: self.pos - self.column_start + self.tab_compensation,
        }
    }

    fn looking_at(&self, prefix: &str) -> bool {
        self.source[self.pos..].starts_with(prefix)
    }

    fn scan_char(&mut self, tok: Token<'a>) -> Result<LocatedToken<'a>, LocatedError> {
        assert!(self.lookahead.is_some());
        let loc = self.loc();
        self.next_ch();
        token(tok, loc)
    }

    pub fn rest_of_line(&mut self) -> &'a str {
        let begin = self.pos;
        loop {
            match self.next_ch() {
                None | Some('\n') => return &self.source[begin..self.pos],
                _ => {}
            }
        }
    }

    fn scan_word(&mut self) -> Result<LocatedToken<'a>, LocatedError> {
        let begin = self.pos;
        let loc = self.loc();
        assert!(self.lookahead == Some('_') || self.lookahead.unwrap().is_alphabetic());
        loop {
            match self.next_ch() {
                Some('_') | Some('-') => {}
                Some(ch) if ch.is_alphanumeric() => {}
                _ => break,
            }
        }
        let text = &self.source[begin..self.pos];
        token(Token::Word(text), loc)
    }

    fn scan_ident(&mut self) -> Result<LocatedToken<'a>, LocatedError> {
        let loc = self.loc();
        assert!(self.lookahead == Some('$'));
        match self.next_ch() {
            Some(ch) if ch.is_alphanumeric() || ch == '_' => {}
            _ => Err(LocatedError {
                error: LexError::EmptyIdentifier,
                location: loc.clone(),
            })?,
        }
        let begin = self.pos;

        loop {
            match self.next_ch() {
                Some('_') | Some('-') => {}
                Some(ch) if ch.is_alphanumeric() => {}
                _ => break,
            }
        }

        let text = &self.source[begin..self.pos];
        token(Token::Ident(text), loc)
    }

    fn scan_annotation(&mut self) -> Result<LocatedToken<'a>, LocatedError> {
        let loc = self.loc();
        assert!(self.lookahead == Some('@'));
        match self.next_ch() {
            Some(ch) if ch.is_alphanumeric() || ch == '_' => {}
            _ => Err(LocatedError {
                error: LexError::EmptyAnnotation,
                location: loc.clone(),
            })?,
        }
        let begin = self.pos;

        loop {
            match self.next_ch() {
                Some('_') | Some('-') => {}
                Some(ch) if ch.is_alphanumeric() => {}
                _ => break,
            }
        }

        let text = &self.source[begin..self.pos];
        token(Token::Annot(text), loc)
    }

    fn scan_quote(&mut self) -> Result<LocatedToken<'a>, LocatedError> {
        let begin = self.pos;
        let loc = self.loc();
        assert!(self.lookahead == Some('"'));
        loop {
            match self.next_ch() {
                None => Err(LocatedError {
                    error: LexError::UnterminatedQuote,
                    location: loc.clone(),
                })?,
                Some('"') => {
                    self.next_ch();
                    break;
                }
                _ => {}
            }
        }
        let text = &self.source[(begin + 1)..(self.pos - 1)];
        token(Token::Quote(text), loc)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Result<LocatedToken<'a>, LocatedError>> {
        loop {
            let loc = self.loc();
            return match self.lookahead {
                None => None,
                Some(c) => Some(match c {
                    '(' => self.scan_char(Token::LPar),
                    ')' => self.scan_char(Token::RPar),
                    '$' => self.scan_ident(),
                    '@' => self.scan_annotation(),
                    ';' => {
                        if self.looking_at(";;") {
                            self.rest_of_line();
                            continue;
                        } else {
                            self.next_ch();
                            error(LexError::InvalidChar(';'), loc)
                        }
                    }
                    '"' => self.scan_quote(),
                    '_' => self.scan_word(),
                    ch if ch.is_alphabetic() => self.scan_word(),
                    ch if ch.is_whitespace() => {
                        self.next_ch();
                        continue;
                    }
                    _ => {
                        self.next_ch();
                        error(LexError::InvalidChar(c), loc)
                    }
                }),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::{Path, PathBuf};

    fn testlexer(input: &str) -> Lexer {
        Lexer::new(input, Path::new("/test"))
    }

    fn token(
        token: Token<'_>,
        line: usize,
        column: usize,
    ) -> Option<Result<LocatedToken<'_>, LocatedError>> {
        Some(super::token(
            token,
            Location {
                path: PathBuf::from("/test"),
                line,
                column,
            },
        ))
    }

    fn error<'a>(
        err: LexError,
        line: usize,
        column: usize,
    ) -> Option<Result<LocatedToken<'a>, LocatedError>> {
        Some(super::error(
            err,
            Location {
                path: PathBuf::from("/test"),
                line,
                column,
            },
        ))
    }
    #[test]
    fn words_and_idents() {
        let mut lex = testlexer("$gussie is a good $dog");
        // ruler                  0    5    10   15   20
        assert_eq!(lex.next(), token(Token::Ident("gussie"), 1, 0));
        assert_eq!(lex.next(), token(Token::Word("is"), 1, 8));
        assert_eq!(lex.next(), token(Token::Word("a"), 1, 11));
        assert_eq!(lex.next(), token(Token::Word("good"), 1, 13));
        assert_eq!(lex.next(), token(Token::Ident("dog"), 1, 18));
        assert_eq!(lex.next(), None);

        let mut lex =
            testlexer("$ok $a $_ $ _\nkebab-case\nsnake_case\n$kebab-ident\n$snake_ident");
        assert_eq!(lex.next(), token(Token::Ident("ok"), 1, 0));
        assert_eq!(lex.next(), token(Token::Ident("a"), 1, 4));
        assert_eq!(lex.next(), token(Token::Ident("_"), 1, 7));
        assert_eq!(lex.next(), error(LexError::EmptyIdentifier, 1, 10));
        assert_eq!(lex.next(), token(Token::Word("_"), 1, 12));
        assert_eq!(lex.next(), token(Token::Word("kebab-case"), 2, 0));
        assert_eq!(lex.next(), token(Token::Word("snake_case"), 3, 0));
        assert_eq!(lex.next(), token(Token::Ident("kebab-ident"), 4, 0));
        assert_eq!(lex.next(), token(Token::Ident("snake_ident"), 5, 0));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn comments() {
        let mut lex = testlexer("the quick ;; brown fox\njumped\n;;over the three\nlazy;;dogs");
        assert_eq!(lex.next(), token(Token::Word("the"), 1, 0));
        assert_eq!(lex.next(), token(Token::Word("quick"), 1, 4));
        assert_eq!(lex.next(), token(Token::Word("jumped"), 2, 0));
        assert_eq!(lex.next(), token(Token::Word("lazy"), 4, 0));
        assert_eq!(lex.next(), None);

        let mut lex = testlexer("line1 ;;\n$sym_2;\n\t\tl3;;;333");
        assert_eq!(lex.next(), token(Token::Word("line1"), 1, 0));
        assert_eq!(lex.next(), token(Token::Ident("sym_2"), 2, 0));
        assert_eq!(lex.next(), error(LexError::InvalidChar(';'), 2, 6));
        assert_eq!(lex.next(), token(Token::Word("l3"), 3, 16)); // Two tabs = 16 columns
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn quotes() {
        let mut lex = testlexer("a \"bc\" d");
        assert_eq!(lex.next(), token(Token::Word("a"), 1, 0));
        assert_eq!(lex.next(), token(Token::Quote("bc"), 1, 2));
        assert_eq!(lex.next(), token(Token::Word("d"), 1, 7));

        let mut lex = testlexer("a \"b\nc\" d");
        assert_eq!(lex.next(), token(Token::Word("a"), 1, 0));
        assert_eq!(lex.next(), token(Token::Quote("b\nc"), 1, 2));
        assert_eq!(lex.next(), token(Token::Word("d"), 2, 3));

        let mut lex = testlexer("a \"b");
        assert_eq!(lex.next(), token(Token::Word("a"), 1, 0));
        assert_eq!(lex.next(), error(LexError::UnterminatedQuote, 1, 2));
    }
}
