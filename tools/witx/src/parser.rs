use wast::lexer::Comment;
use wast::parser::{Cursor, Parse, Parser, Peek, Result};

///! Parser turns s-expressions into unvalidated syntax constructs.
///! conventions:
///! `Type::starts_parsing(s-expr) -> bool` is for look-ahead: we use
///! this predicate to combine parsers for different `Type`s where both
///! alternatives are accepted.
///! `Type::parse(sexpr: &SExpr) -> Result<Self, ParseError>` takes a single
///! s-expression and parses it into a `Self`.
///! for parsers that take a subset of a vector s-expression, the signature
///! `Type::parse(sexprs: &[SExpr], location: Location) -> Result<Self, ParseError>`
///! has an additional `Location` argument, which should point to the parent SExpr::Vec.
///! This is used for error reporting in case the slice doesn't have the number of elements
///! expected.

mod kw {
    pub use wast::kw::{export, func, import, memory, module, param, result};

    wast::custom_keyword!(array);
    wast::custom_keyword!(const_pointer);
    wast::custom_keyword!(f32);
    wast::custom_keyword!(f64);
    wast::custom_keyword!(field);
    wast::custom_keyword!(flags);
    wast::custom_keyword!(pointer);
    wast::custom_keyword!(r#enum = "enum");
    wast::custom_keyword!(r#struct = "struct");
    wast::custom_keyword!(r#union = "union");
    wast::custom_keyword!(r#use = "use");
    wast::custom_keyword!(s16);
    wast::custom_keyword!(s32);
    wast::custom_keyword!(s64);
    wast::custom_keyword!(s8);
    wast::custom_keyword!(string);
    wast::custom_keyword!(typename);
    wast::custom_keyword!(u16);
    wast::custom_keyword!(u32);
    wast::custom_keyword!(u64);
    wast::custom_keyword!(u8);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinType {
    String,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F32,
    F64,
}

impl Parse<'_> for BuiltinType {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::string>() {
            parser.parse::<kw::string>()?;
            Ok(BuiltinType::String)
        } else if l.peek::<kw::u8>() {
            parser.parse::<kw::u8>()?;
            Ok(BuiltinType::U8)
        } else if l.peek::<kw::u16>() {
            parser.parse::<kw::u16>()?;
            Ok(BuiltinType::U16)
        } else if l.peek::<kw::u32>() {
            parser.parse::<kw::u32>()?;
            Ok(BuiltinType::U32)
        } else if l.peek::<kw::u64>() {
            parser.parse::<kw::u64>()?;
            Ok(BuiltinType::U64)
        } else if l.peek::<kw::s8>() {
            parser.parse::<kw::s8>()?;
            Ok(BuiltinType::S8)
        } else if l.peek::<kw::s16>() {
            parser.parse::<kw::s16>()?;
            Ok(BuiltinType::S16)
        } else if l.peek::<kw::s32>() {
            parser.parse::<kw::s32>()?;
            Ok(BuiltinType::S32)
        } else if l.peek::<kw::s64>() {
            parser.parse::<kw::s64>()?;
            Ok(BuiltinType::S64)
        } else if l.peek::<kw::f32>() {
            parser.parse::<kw::f32>()?;
            Ok(BuiltinType::F32)
        } else if l.peek::<kw::f64>() {
            parser.parse::<kw::f64>()?;
            Ok(BuiltinType::F64)
        } else {
            Err(l.error())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommentSyntax<'a> {
    pub comments: Vec<&'a str>,
}

impl<'a> Parse<'a> for CommentSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<CommentSyntax<'a>> {
        let comments = parser.step(|mut cursor| {
            let mut comments = Vec::new();
            loop {
                let (comment, c) = match cursor.comment() {
                    Some(pair) => pair,
                    None => break,
                };
                cursor = c;
                comments.push(match comment {
                    Comment::Block(s) => &s[2..s.len() - 2],
                    Comment::Line(s) => &s[2..],
                });
            }
            Ok((comments, cursor))
        })?;
        Ok(CommentSyntax { comments })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Documented<'a, T> {
    pub comments: CommentSyntax<'a>,
    pub item: T,
}

impl<'a, T: Parse<'a>> Parse<'a> for Documented<'a, T> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let comments = parser.parse()?;
        let item = parser.parens(T::parse)?;
        Ok(Documented { comments, item })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatatypeIdentSyntax<'a> {
    Builtin(BuiltinType),
    Array(Box<DatatypeIdentSyntax<'a>>),
    Pointer(Box<DatatypeIdentSyntax<'a>>),
    ConstPointer(Box<DatatypeIdentSyntax<'a>>),
    Ident(wast::Id<'a>),
}

impl<'a> Parse<'a> for DatatypeIdentSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if parser.peek::<wast::Id>() {
            Ok(DatatypeIdentSyntax::Ident(parser.parse()?))
        } else if parser.peek2::<kw::array>() {
            Ok(DatatypeIdentSyntax::Array(parser.parens(|p| {
                p.parse::<kw::array>()?;
                Ok(Box::new(parser.parse()?))
            })?))
        } else if parser.peek::<wast::LParen>() {
            parser.parens(|p| {
                p.parse::<AtWitx>()?;
                if p.peek::<kw::const_pointer>() {
                    p.parse::<kw::const_pointer>()?;
                    Ok(DatatypeIdentSyntax::ConstPointer(Box::new(p.parse()?)))
                } else {
                    p.parse::<kw::pointer>()?;
                    Ok(DatatypeIdentSyntax::Pointer(Box::new(p.parse()?)))
                }
            })
        } else {
            Ok(DatatypeIdentSyntax::Builtin(parser.parse()?))
        }
    }
}

struct AtWitx;

impl Parse<'_> for AtWitx {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.step(|c| {
            if let Some(("@witx", rest)) = c.reserved() {
                return Ok((AtWitx, rest));
            }
            Err(c.error("expected `@witx`"))
        })
    }
}

impl Peek for AtWitx {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.reserved().map(|s| s.0) == Some("@witx")
    }

    fn display() -> &'static str {
        "`@witx`"
    }
}

struct AtInterface;

impl Parse<'_> for AtInterface {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.step(|c| {
            if let Some(("@interface", rest)) = c.reserved() {
                return Ok((AtInterface, rest));
            }
            Err(c.error("expected `@interface`"))
        })
    }
}

impl Peek for AtInterface {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.reserved().map(|s| s.0) == Some("@interface")
    }

    fn display() -> &'static str {
        "`@interface`"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopLevelDocument<'a> {
    pub items: Vec<TopLevelSyntax<'a>>,
}

impl<'a> Parse<'a> for TopLevelDocument<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut items = Vec::new();
        while !parser.is_empty() {
            items.push(parser.parens(|p| p.parse())?);
        }
        Ok(TopLevelDocument { items })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelSyntax<'a> {
    Decl(DeclSyntax<'a>),
    Use(&'a str),
}

impl<'a> Parse<'a> for TopLevelSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if parser.peek::<kw::r#use>() {
            parser.parse::<kw::r#use>()?;
            Ok(TopLevelSyntax::Use(parser.parse()?))
        } else {
            Ok(TopLevelSyntax::Decl(parser.parse()?))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclSyntax<'a> {
    Typename(TypenameSyntax<'a>),
    Module(ModuleSyntax<'a>),
}

impl<'a> Parse<'a> for DeclSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::module>() {
            Ok(DeclSyntax::Module(parser.parse()?))
        } else if l.peek::<kw::typename>() {
            Ok(DeclSyntax::Typename(parser.parse()?))
        } else {
            Err(l.error())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypenameSyntax<'a> {
    pub ident: wast::Id<'a>,
    pub def: TypedefSyntax<'a>,
}

impl<'a> Parse<'a> for TypenameSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::typename>()?;
        let ident = parser.parse()?;
        let def = parser.parse()?;
        Ok(TypenameSyntax { ident, def })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedefSyntax<'a> {
    Ident(DatatypeIdentSyntax<'a>),
    Enum(EnumSyntax<'a>),
    Flags(FlagsSyntax<'a>),
    Struct(StructSyntax<'a>),
    Union(UnionSyntax<'a>),
}

impl<'a> Parse<'a> for TypedefSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if !parser.peek::<wast::LParen>() || parser.peek2::<kw::array>() || parser.peek2::<AtWitx>()
        {
            return Ok(TypedefSyntax::Ident(parser.parse()?));
        }
        parser.parens(|parser| {
            let mut l = parser.lookahead1();
            if l.peek::<kw::r#enum>() {
                Ok(TypedefSyntax::Enum(parser.parse()?))
            } else if l.peek::<kw::flags>() {
                Ok(TypedefSyntax::Flags(parser.parse()?))
            } else if l.peek::<kw::r#struct>() {
                Ok(TypedefSyntax::Struct(parser.parse()?))
            } else if l.peek::<kw::r#union>() {
                Ok(TypedefSyntax::Union(parser.parse()?))
            } else {
                Err(l.error())
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumSyntax<'a> {
    pub repr: BuiltinType,
    pub members: Vec<wast::Id<'a>>,
}

impl<'a> Parse<'a> for EnumSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::r#enum>()?;
        let repr = parser.parse()?;
        let mut members = Vec::new();
        members.push(parser.parse()?);
        while !parser.is_empty() {
            members.push(parser.parse()?);
        }
        Ok(EnumSyntax { repr, members })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsSyntax<'a> {
    pub repr: BuiltinType,
    pub flags: Vec<wast::Id<'a>>,
}

impl<'a> Parse<'a> for FlagsSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::flags>()?;
        let repr = parser.parse()?;
        let mut flags = Vec::new();
        while !parser.is_empty() {
            flags.push(parser.parse()?);
        }
        Ok(FlagsSyntax { repr, flags })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructSyntax<'a> {
    pub fields: Vec<FieldSyntax<'a>>,
}

impl<'a> Parse<'a> for StructSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::r#struct>()?;
        let mut fields = Vec::new();
        fields.push(parser.parens(|p| p.parse())?);
        while !parser.is_empty() {
            fields.push(parser.parens(|p| p.parse())?);
        }
        Ok(StructSyntax { fields })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldSyntax<'a> {
    pub name: wast::Id<'a>,
    pub type_: DatatypeIdentSyntax<'a>,
}

impl<'a> Parse<'a> for FieldSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::field>()?;
        let name = parser.parse()?;
        let type_ = parser.parse()?;
        Ok(FieldSyntax { name, type_ })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionSyntax<'a> {
    pub fields: Vec<FieldSyntax<'a>>,
}

impl<'a> Parse<'a> for UnionSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::r#union>()?;
        let mut fields = Vec::new();
        fields.push(parser.parens(|p| p.parse())?);
        while !parser.is_empty() {
            fields.push(parser.parens(|p| p.parse())?);
        }
        Ok(UnionSyntax { fields })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleSyntax<'a> {
    pub name: wast::Id<'a>,
    pub decls: Vec<ModuleDeclSyntax<'a>>,
}

impl<'a> Parse<'a> for ModuleSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::module>()?;
        let name = parser.parse()?;
        let mut decls = Vec::new();
        while !parser.is_empty() {
            decls.push(parser.parens(|p| p.parse())?);
        }
        Ok(ModuleSyntax { name, decls })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDeclSyntax<'a> {
    Import(ModuleImportSyntax<'a>),
    Func(InterfaceFuncSyntax<'a>),
}

impl<'a> Parse<'a> for ModuleDeclSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::import>() {
            Ok(ModuleDeclSyntax::Import(parser.parse()?))
        } else if l.peek::<AtInterface>() {
            Ok(ModuleDeclSyntax::Func(parser.parse()?))
        } else {
            Err(l.error())
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleImportSyntax<'a> {
    pub name: &'a str,
    pub name_loc: wast::Span,
    pub type_: ImportTypeSyntax,
}

impl<'a> Parse<'a> for ModuleImportSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::import>()?;
        let name_loc = parser.cur_span();
        Ok(ModuleImportSyntax {
            name: parser.parse()?,
            name_loc,
            type_: parser.parens(|p| p.parse())?,
        })
    }
}

impl PartialEq for ModuleImportSyntax<'_> {
    fn eq(&self, other: &ModuleImportSyntax<'_>) -> bool {
        // skip the `name_loc` field
        self.name == other.name && self.type_ == other.type_
    }
}

impl Eq for ModuleImportSyntax<'_> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportTypeSyntax {
    Memory,
}

impl Parse<'_> for ImportTypeSyntax {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<kw::memory>()?;
        Ok(ImportTypeSyntax::Memory)
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceFuncSyntax<'a> {
    pub export: &'a str,
    pub export_loc: wast::Span,
    pub params: Vec<FieldSyntax<'a>>,
    pub results: Vec<FieldSyntax<'a>>,
}

impl<'a> Parse<'a> for InterfaceFuncSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<AtInterface>()?;
        parser.parse::<kw::func>()?;

        let (export_loc, export) = parser.parens(|p| {
            p.parse::<kw::export>()?;
            Ok((p.cur_span(), p.parse()?))
        })?;

        let mut params = Vec::new();
        let mut results = Vec::new();

        while !parser.is_empty() {
            parser.parens(|p| {
                let mut l = p.lookahead1();
                if l.peek::<kw::param>() {
                    parser.parse::<kw::param>()?;
                    params.push(FieldSyntax {
                        name: parser.parse()?,
                        type_: parser.parse()?,
                    });
                } else if l.peek::<kw::result>() {
                    parser.parse::<kw::result>()?;
                    results.push(FieldSyntax {
                        name: parser.parse()?,
                        type_: parser.parse()?,
                    });
                } else {
                    return Err(l.error());
                }
                Ok(())
            })?;
        }

        Ok(InterfaceFuncSyntax {
            export,
            export_loc,
            params,
            results,
        })
    }
}

impl PartialEq for InterfaceFuncSyntax<'_> {
    fn eq(&self, other: &InterfaceFuncSyntax<'_>) -> bool {
        // skip the `export_loc` field
        self.export == other.export && self.params == other.params && self.results == other.results
    }
}

impl Eq for InterfaceFuncSyntax<'_> {}
