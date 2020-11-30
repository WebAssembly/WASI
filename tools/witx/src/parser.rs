use crate::BuiltinType;
use wast::lexer::Comment;
use wast::parser::{Parse, Parser, Peek, Result};

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
    wast::custom_keyword!(char8);
    wast::custom_keyword!(const_pointer);
    wast::custom_keyword!(f32);
    wast::custom_keyword!(f64);
    wast::custom_keyword!(field);
    wast::custom_keyword!(empty);
    wast::custom_keyword!(flags);
    wast::custom_keyword!(handle);
    wast::custom_keyword!(int);
    wast::custom_keyword!(noreturn);
    wast::custom_keyword!(pointer);
    wast::custom_keyword!(r#const = "const");
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
    wast::custom_keyword!(usize);
}

mod annotation {
    wast::annotation!(interface);
    wast::annotation!(witx);
}

impl Parse<'_> for BuiltinType {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::string>() {
            parser.parse::<kw::string>()?;
            Ok(BuiltinType::String)
        } else if l.peek::<kw::char8>() {
            parser.parse::<kw::char8>()?;
            Ok(BuiltinType::Char8)
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

impl wast::parser::Peek for BuiltinType {
    fn peek(cursor: wast::parser::Cursor<'_>) -> bool {
        <kw::string as Peek>::peek(cursor)
            || <kw::char8 as Peek>::peek(cursor)
            || <kw::u8 as Peek>::peek(cursor)
            || <kw::u16 as Peek>::peek(cursor)
            || <kw::u32 as Peek>::peek(cursor)
            || <kw::u64 as Peek>::peek(cursor)
            || <kw::s8 as Peek>::peek(cursor)
            || <kw::s16 as Peek>::peek(cursor)
            || <kw::s32 as Peek>::peek(cursor)
            || <kw::s64 as Peek>::peek(cursor)
            || <kw::f32 as Peek>::peek(cursor)
            || <kw::f64 as Peek>::peek(cursor)
    }
    fn display() -> &'static str {
        "builtin type"
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

impl<'a> CommentSyntax<'a> {
    pub fn docs(&self) -> String {
        // Perform a small amount of preprocessing by removing all trailing
        // whitespace, and then also filter for only "doc comments" which are `;;;`
        // or `(;; ... ;)`.
        let docs = self
            .comments
            .iter()
            .map(|d| d.trim_end())
            .filter_map(|d| {
                if d.starts_with(";") {
                    Some(&d[1..])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Figure out how much leading whitespace we're going to be trimming from
        // all docs, trimming the minimum amount in each doc comment.
        let to_trim = docs
            .iter()
            .filter(|d| !d.is_empty())
            .map(|d| d.len() - d.trim().len())
            .min()
            .unwrap_or(0);

        // Separate all documents by a newline and collect everything into a single
        // string.
        let mut ret = String::new();
        for doc in docs {
            if !doc.is_empty() {
                ret.push_str(doc[to_trim..].trim_end());
            }
            ret.push_str("\n");
        }
        return ret;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Documented<'a, T> {
    pub comments: CommentSyntax<'a>,
    pub item: T,
}

impl<'a, T: Parse<'a>> Parse<'a> for Documented<'a, T> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let _r1 = parser.register_annotation("witx");
        let _r1 = parser.register_annotation("interface");
        let comments = parser.parse()?;
        let item = parser.parse()?;
        Ok(Documented { comments, item })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopLevelDocument<'a> {
    pub items: Vec<Documented<'a, TopLevelSyntax<'a>>>,
}

impl<'a> Parse<'a> for TopLevelDocument<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut items = Vec::new();
        while !parser.is_empty() {
            items.push(parser.parse()?);
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
        parser.parens(|p| {
            if p.peek::<kw::r#use>() {
                p.parse::<kw::r#use>()?;
                Ok(TopLevelSyntax::Use(p.parse()?))
            } else {
                Ok(TopLevelSyntax::Decl(p.parse()?))
            }
        })
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
    Enum(EnumSyntax<'a>),
    Int(IntSyntax<'a>),
    Flags(FlagsSyntax<'a>),
    Struct(StructSyntax<'a>),
    Union(UnionSyntax<'a>),
    Handle(HandleSyntax),
    Array(Box<TypedefSyntax<'a>>),
    Pointer(Box<TypedefSyntax<'a>>),
    ConstPointer(Box<TypedefSyntax<'a>>),
    Builtin(BuiltinType),
    Ident(wast::Id<'a>),
}

impl<'a> Parse<'a> for TypedefSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<wast::Id>() {
            Ok(TypedefSyntax::Ident(parser.parse()?))
        } else if l.peek::<BuiltinType>() {
            Ok(TypedefSyntax::Builtin(parser.parse()?))
        } else if l.peek::<wast::LParen>() {
            parser.parens(|parser| {
                let mut l = parser.lookahead1();
                if l.peek::<kw::r#enum>() {
                    Ok(TypedefSyntax::Enum(parser.parse()?))
                } else if l.peek::<kw::int>() {
                    Ok(TypedefSyntax::Int(parser.parse()?))
                } else if l.peek::<kw::flags>() {
                    Ok(TypedefSyntax::Flags(parser.parse()?))
                } else if l.peek::<kw::r#struct>() {
                    Ok(TypedefSyntax::Struct(parser.parse()?))
                } else if l.peek::<kw::r#union>() {
                    Ok(TypedefSyntax::Union(parser.parse()?))
                } else if l.peek::<kw::handle>() {
                    Ok(TypedefSyntax::Handle(parser.parse()?))
                } else if l.peek::<kw::array>() {
                    parser.parse::<kw::array>()?;
                    Ok(TypedefSyntax::Array(Box::new(parser.parse()?)))
                } else if l.peek::<annotation::witx>() {
                    parser.parse::<annotation::witx>()?;
                    let mut l = parser.lookahead1();
                    if l.peek::<kw::const_pointer>() {
                        parser.parse::<kw::const_pointer>()?;
                        Ok(TypedefSyntax::ConstPointer(Box::new(parser.parse()?)))
                    } else if l.peek::<kw::pointer>() {
                        parser.parse::<kw::pointer>()?;
                        Ok(TypedefSyntax::Pointer(Box::new(parser.parse()?)))
                    } else if l.peek::<kw::usize>() {
                        parser.parse::<kw::usize>()?;
                        Ok(TypedefSyntax::Builtin(BuiltinType::USize))
                    } else {
                        Err(l.error())
                    }
                } else {
                    Err(l.error())
                }
            })
        } else {
            Err(l.error())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumSyntax<'a> {
    pub repr: BuiltinType,
    pub members: Vec<Documented<'a, wast::Id<'a>>>,
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
pub struct IntSyntax<'a> {
    pub repr: BuiltinType,
    pub consts: Vec<Documented<'a, ConstSyntax<'a>>>,
}

impl<'a> Parse<'a> for IntSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::int>()?;
        let repr = parser.parse()?;
        let mut consts = Vec::new();
        consts.push(parser.parse()?);
        while !parser.is_empty() {
            consts.push(parser.parse()?);
        }
        Ok(IntSyntax { repr, consts })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstSyntax<'a> {
    pub name: wast::Id<'a>,
    pub value: u64,
}

impl<'a> Parse<'a> for ConstSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parens(|p| {
            p.parse::<kw::r#const>()?;
            let name = p.parse()?;
            let value = p.parse()?;
            Ok(ConstSyntax { name, value })
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsSyntax<'a> {
    pub repr: BuiltinType,
    pub flags: Vec<Documented<'a, wast::Id<'a>>>,
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
    pub fields: Vec<Documented<'a, FieldSyntax<'a>>>,
}

impl<'a> Parse<'a> for StructSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::r#struct>()?;
        let mut fields = Vec::new();
        fields.push(parser.parse()?);
        while !parser.is_empty() {
            fields.push(parser.parse()?);
        }
        Ok(StructSyntax { fields })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldSyntax<'a> {
    pub name: wast::Id<'a>,
    pub type_: TypedefSyntax<'a>,
}

impl<'a> Parse<'a> for FieldSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parens(|p| {
            p.parse::<kw::field>()?;
            let name = p.parse()?;
            let type_ = p.parse()?;
            Ok(FieldSyntax { name, type_ })
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariantSyntax<'a> {
    Field(FieldSyntax<'a>),
    Empty(wast::Id<'a>),
}

impl<'a> Parse<'a> for VariantSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parens(|p| {
            let mut l = p.lookahead1();
            if l.peek::<kw::field>() {
                parser.parse::<kw::field>()?;
                let name = p.parse()?;
                let type_ = p.parse()?;
                Ok(VariantSyntax::Field(FieldSyntax { name, type_ }))
            } else if l.peek::<kw::empty>() {
                parser.parse::<kw::empty>()?;
                let name = p.parse()?;
                Ok(VariantSyntax::Empty(name))
            } else {
                Err(l.error())
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionSyntax<'a> {
    pub tag: wast::Id<'a>,
    pub fields: Vec<Documented<'a, VariantSyntax<'a>>>,
}

impl<'a> Parse<'a> for UnionSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::r#union>()?;
        let tag = parser.parse()?;
        let mut fields = Vec::new();
        fields.push(parser.parse()?);
        while !parser.is_empty() {
            fields.push(parser.parse()?);
        }
        Ok(UnionSyntax { tag, fields })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandleSyntax {}

impl<'a> Parse<'a> for HandleSyntax {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::handle>()?;
        Ok(HandleSyntax {})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleSyntax<'a> {
    pub name: wast::Id<'a>,
    pub decls: Vec<Documented<'a, ModuleDeclSyntax<'a>>>,
}

impl<'a> Parse<'a> for ModuleSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::module>()?;
        let name = parser.parse()?;
        let mut decls = Vec::new();
        while !parser.is_empty() {
            decls.push(parser.parse()?);
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
        parser.parens(|p| {
            let mut l = p.lookahead1();
            if l.peek::<kw::import>() {
                Ok(ModuleDeclSyntax::Import(p.parse()?))
            } else if l.peek::<annotation::interface>() {
                Ok(ModuleDeclSyntax::Func(p.parse()?))
            } else {
                Err(l.error())
            }
        })
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
    pub params: Vec<Documented<'a, FieldSyntax<'a>>>,
    pub results: Vec<Documented<'a, FieldSyntax<'a>>>,
    pub noreturn: bool,
}

impl<'a> Parse<'a> for InterfaceFuncSyntax<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<annotation::interface>()?;
        parser.parse::<kw::func>()?;

        let (export_loc, export) = parser.parens(|p| {
            p.parse::<kw::export>()?;
            Ok((p.cur_span(), p.parse()?))
        })?;

        let mut params = Vec::new();
        let mut results = Vec::new();
        let mut noreturn = false;

        while !parser.is_empty() {
            let func_field = parser.parse::<Documented<InterfaceFuncField>>()?;
            match func_field.item {
                InterfaceFuncField::Param(item) => {
                    params.push(Documented {
                        comments: func_field.comments,
                        item,
                    });
                }
                InterfaceFuncField::Result(item) => {
                    results.push(Documented {
                        comments: func_field.comments,
                        item,
                    });
                }
                InterfaceFuncField::Noreturn => {
                    noreturn = true;
                }
            }
        }

        Ok(InterfaceFuncSyntax {
            export,
            export_loc,
            params,
            results,
            noreturn,
        })
    }
}

enum InterfaceFuncField<'a> {
    Param(FieldSyntax<'a>),
    Result(FieldSyntax<'a>),
    Noreturn,
}
impl<'a> Parse<'a> for InterfaceFuncField<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parens(|p| {
            let mut l = p.lookahead1();
            if l.peek::<kw::param>() {
                parser.parse::<kw::param>()?;
                Ok(InterfaceFuncField::Param(FieldSyntax {
                    name: parser.parse()?,
                    type_: parser.parse()?,
                }))
            } else if l.peek::<kw::result>() {
                parser.parse::<kw::result>()?;
                Ok(InterfaceFuncField::Result(FieldSyntax {
                    name: parser.parse()?,
                    type_: parser.parse()?,
                }))
            } else if l.peek::<annotation::witx>() {
                parser.parse::<annotation::witx>()?;
                let mut l = parser.lookahead1();
                if l.peek::<kw::noreturn>() {
                    parser.parse::<kw::noreturn>()?;
                    Ok(InterfaceFuncField::Noreturn)
                } else {
                    Err(l.error())
                }
            } else {
                Err(l.error())
            }
        })
    }
}

impl PartialEq for InterfaceFuncSyntax<'_> {
    fn eq(&self, other: &InterfaceFuncSyntax<'_>) -> bool {
        // skip the `export_loc` field
        self.export == other.export
            && self.params == other.params
            && self.results == other.results
            && self.noreturn == other.noreturn
    }
}

impl Eq for InterfaceFuncSyntax<'_> {}
