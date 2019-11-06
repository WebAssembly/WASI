use crate::ast::*;
use std::fmt;

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in self.datatypes() {
            write!(f, "{}\n", d.to_sexpr())?;
        }
        for m in self.modules() {
            write!(f, "{}\n", m.to_sexpr())?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExpr {
    Vec(Vec<SExpr>),
    Word(String),
    Ident(String),
    Quote(String),
    /// Short for Annotation
    Annot(String),
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExpr::Vec(vs) => {
                write!(f, "(")?;
                let mut vss = Vec::new();
                for v in vs {
                    vss.push(format!("{}", v));
                }
                f.write_str(&vss.join(" "))?;
                write!(f, ")")
            }
            SExpr::Word(w) => write!(f, "{}", w),
            SExpr::Ident(i) => write!(f, "${}", i),
            SExpr::Quote(q) => write!(f, "\"{}\"", q),
            SExpr::Annot(a) => write!(f, "@{}", a),
        }
    }
}

impl SExpr {
    fn word(s: &str) -> SExpr {
        SExpr::Word(s.to_string())
    }
    fn ident(s: &str) -> SExpr {
        SExpr::Ident(s.to_string())
    }
    fn quote(s: &str) -> SExpr {
        SExpr::Quote(s.to_string())
    }
    fn annot(s: &str) -> SExpr {
        SExpr::Annot(s.to_string())
    }
}

pub trait Render {
    fn to_sexpr(&self) -> SExpr;
}

impl Render for Id {
    fn to_sexpr(&self) -> SExpr {
        SExpr::ident(self.as_str())
    }
}

impl Render for BuiltinType {
    fn to_sexpr(&self) -> SExpr {
        match self {
            BuiltinType::String => SExpr::word("string"),
            BuiltinType::U8 => SExpr::word("u8"),
            BuiltinType::U16 => SExpr::word("u16"),
            BuiltinType::U32 => SExpr::word("u32"),
            BuiltinType::U64 => SExpr::word("u64"),
            BuiltinType::S8 => SExpr::word("s8"),
            BuiltinType::S16 => SExpr::word("s16"),
            BuiltinType::S32 => SExpr::word("s32"),
            BuiltinType::S64 => SExpr::word("s64"),
            BuiltinType::F32 => SExpr::word("f32"),
            BuiltinType::F64 => SExpr::word("f64"),
        }
    }
}

impl Render for DatatypeIdent {
    fn to_sexpr(&self) -> SExpr {
        match self {
            DatatypeIdent::Builtin(b) => b.to_sexpr(),
            DatatypeIdent::Array(a) => SExpr::Vec(vec![SExpr::word("array"), a.to_sexpr()]),
            DatatypeIdent::Pointer(p) => SExpr::Vec(vec![
                SExpr::annot("witx"),
                SExpr::word("pointer"),
                p.to_sexpr(),
            ]),
            DatatypeIdent::ConstPointer(p) => SExpr::Vec(vec![
                SExpr::annot("witx"),
                SExpr::word("const_pointer"),
                p.to_sexpr(),
            ]),
            DatatypeIdent::Ident(i) => i.name.to_sexpr(),
        }
    }
}

impl Render for Datatype {
    fn to_sexpr(&self) -> SExpr {
        let name = self.name.to_sexpr();
        let body = self.variant.to_sexpr();
        SExpr::Vec(vec![SExpr::word("typename"), name, body])
    }
}

impl Render for DatatypeVariant {
    fn to_sexpr(&self) -> SExpr {
        match self {
            DatatypeVariant::Alias(a) => a.to_sexpr(),
            DatatypeVariant::Enum(a) => a.to_sexpr(),
            DatatypeVariant::Flags(a) => a.to_sexpr(),
            DatatypeVariant::Struct(a) => a.to_sexpr(),
            DatatypeVariant::Union(a) => a.to_sexpr(),
        }
    }
}

impl Render for AliasDatatype {
    fn to_sexpr(&self) -> SExpr {
        self.to.to_sexpr()
    }
}

impl Render for EnumDatatype {
    fn to_sexpr(&self) -> SExpr {
        let header = vec![SExpr::word("enum"), self.repr.to_sexpr()];
        let variants = self
            .variants
            .iter()
            .map(|v| v.name.to_sexpr())
            .collect::<Vec<SExpr>>();
        SExpr::Vec([header, variants].concat())
    }
}

impl Render for FlagsDatatype {
    fn to_sexpr(&self) -> SExpr {
        let header = vec![SExpr::word("flags"), self.repr.to_sexpr()];
        let flags = self
            .flags
            .iter()
            .map(|f| f.to_sexpr())
            .collect::<Vec<SExpr>>();
        SExpr::Vec([header, flags].concat())
    }
}

impl Render for StructDatatype {
    fn to_sexpr(&self) -> SExpr {
        let header = vec![SExpr::word("struct")];
        let members = self
            .members
            .iter()
            .map(|m| {
                SExpr::Vec(vec![
                    SExpr::word("field"),
                    m.name.to_sexpr(),
                    m.type_.to_sexpr(),
                ])
            })
            .collect::<Vec<SExpr>>();
        SExpr::Vec([header, members].concat())
    }
}

impl Render for UnionDatatype {
    fn to_sexpr(&self) -> SExpr {
        let header = vec![SExpr::word("union")];
        let variants = self
            .variants
            .iter()
            .map(|v| {
                SExpr::Vec(vec![
                    SExpr::word("field"),
                    v.name.to_sexpr(),
                    v.type_.to_sexpr(),
                ])
            })
            .collect::<Vec<SExpr>>();
        SExpr::Vec([header, variants].concat())
    }
}

impl Render for IntRepr {
    fn to_sexpr(&self) -> SExpr {
        match self {
            IntRepr::U8 => SExpr::word("u8"),
            IntRepr::U16 => SExpr::word("u16"),
            IntRepr::U32 => SExpr::word("u32"),
            IntRepr::U64 => SExpr::word("u64"),
        }
    }
}

impl Render for Module {
    fn to_sexpr(&self) -> SExpr {
        let header = vec![SExpr::word("module"), self.name.to_sexpr()];
        let definitions = self
            .imports()
            .map(|i| i.to_sexpr())
            .chain(self.funcs().map(|f| f.to_sexpr()))
            .collect::<Vec<SExpr>>();
        SExpr::Vec([header, definitions].concat())
    }
}

impl Render for ModuleImport {
    fn to_sexpr(&self) -> SExpr {
        let variant = match self.variant {
            ModuleImportVariant::Memory => SExpr::Vec(vec![SExpr::word("memory")]),
        };
        SExpr::Vec(vec![
            SExpr::word("import"),
            SExpr::quote(self.name.as_str()),
            variant,
        ])
    }
}

impl Render for InterfaceFunc {
    fn to_sexpr(&self) -> SExpr {
        let header = vec![
            SExpr::annot("interface"),
            SExpr::word("func"),
            SExpr::Vec(vec![
                SExpr::word("export"),
                SExpr::quote(self.name.as_str()),
            ]),
        ];
        let params = self
            .params
            .iter()
            .map(|f| {
                SExpr::Vec(vec![
                    SExpr::word("param"),
                    f.name.to_sexpr(),
                    f.type_.to_sexpr(),
                ])
            })
            .collect();
        let results = self
            .results
            .iter()
            .map(|f| {
                SExpr::Vec(vec![
                    SExpr::word("result"),
                    f.name.to_sexpr(),
                    f.type_.to_sexpr(),
                ])
            })
            .collect();
        SExpr::Vec([header, params, results].concat())
    }
}
