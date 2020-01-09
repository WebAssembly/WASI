mod md;

use crate::ast::*;
use crate::polyfill::*;
use crate::RepEquality;
use md::*;
use std::rc::{Rc, Weak};

pub trait Documentation {
    fn to_md(&self) -> String;
}

trait ToMarkdown {
    fn gen(&self, _parent: Option<Weak<MdElement>>) -> Rc<MdElement>;
}

impl Documentation for Document {
    fn to_md(&self) -> String {
        format!("{}", self.gen(None))
    }
}

impl ToMarkdown for Document {
    fn gen(&self, parent: Option<Weak<MdElement>>) -> Rc<MdElement> {
        let doc = Rc::new(MdSection::new("root", "Root", parent));
        let types = Rc::new(MdSection::new("types", "Types", Some(Rc::downgrade(&doc))));
        for typename in self.typenames() {
            types
                .as_section_mut()
                .elements
                .push(typename.gen(Some(Rc::downgrade(&types))));
        }
        let modules = Rc::new(MdSection::new(
            "modules",
            "Modules",
            Some(Rc::downgrade(&doc)),
        ));
        for module in self.modules() {
            modules
                .as_section_mut()
                .elements
                .push(module.gen(Some(Rc::downgrade(&modules))));
        }
        doc.as_section_mut()
            .elements
            .extend_from_slice(&[types, modules]);
        doc
    }
}

impl BuiltinType {
    pub fn type_name(&self) -> &'static str {
        match self {
            BuiltinType::String => "string",
            BuiltinType::Char8 => "char8",
            BuiltinType::USize => "usize",
            BuiltinType::U8 => "u8",
            BuiltinType::U16 => "u16",
            BuiltinType::U32 => "u32",
            BuiltinType::U64 => "u64",
            BuiltinType::S8 => "s8",
            BuiltinType::S16 => "s16",
            BuiltinType::S32 => "s32",
            BuiltinType::S64 => "s64",
            BuiltinType::F32 => "f32",
            BuiltinType::F64 => "f64",
        }
    }
}

impl ToMarkdown for NamedType {
    fn gen(&self, parent: Option<Weak<MdElement>>) -> Rc<MdElement> {
        match &self.tref {
            TypeRef::Value(v) => {
                let md_type = MdType::from(&**v);
                let mut elements: Vec<_> = match &**v {
                    Type::Enum(a) => a.variants.iter().map(MdBullet::from).collect(),
                    Type::Int(a) => a.consts.iter().map(MdBullet::from).collect(),
                    Type::Flags(a) => a.flags.iter().map(MdBullet::from).collect(),
                    Type::Struct(a) => a.members.iter().map(MdBullet::from).collect(),
                    Type::Union(a) => a.variants.iter().map(MdBullet::from).collect(),
                    Type::Handle(a) => a.supertypes.iter().map(MdBullet::from).collect(),
                    _ => vec![],
                };
                let listing = Rc::new(MdTypeListing::new(self.name.as_str(), md_type, parent));
                listing
                    .as_type_listing_mut()
                    .description
                    .push(MdParagraph::new(&self.docs));
                listing.as_type_listing_mut().elements.append(&mut elements);
                listing
            }
            TypeRef::Name(n) => {
                let sec = Rc::new(MdSection::new(
                    self.name.as_str(),
                    self.name.as_str(),
                    parent,
                ));
                sec.as_section_mut().description.append(&mut vec![
                    MdParagraph::new(&self.docs),
                    MdParagraph::new(format!("Alias to {}", n.name.as_str())),
                ]);
                sec
            }
        }
    }
}

impl From<&Type> for MdType {
    fn from(r#type: &Type) -> Self {
        match r#type {
            Type::Enum(a) => Self::Enum {
                repr: a.repr.type_name().to_owned(),
            },
            Type::Int(a) => Self::Int {
                repr: a.repr.type_name().to_owned(),
            },
            Type::Flags(a) => Self::Flags {
                repr: a.repr.type_name().to_owned(),
            },
            Type::Struct(_) => Self::Struct,
            Type::Union(_) => Self::Union,
            Type::Handle(_) => Self::Handle,
            Type::Array(a) => Self::Array {
                r#type: a.type_name().to_owned(),
            },
            Type::Pointer(a) => Self::Pointer {
                to: a.type_name().to_owned(),
            },
            Type::ConstPointer(a) => Self::ConstPointer {
                to: a.type_name().to_owned(),
            },
            Type::Builtin(a) => Self::Builtin {
                r#type: a.type_name().to_owned(),
            },
        }
    }
}

macro_rules! impl_mdbullet {
    ($from:ty) => {
        impl From<$from> for MdBullet {
            fn from(f: $from) -> Self {
                Self {
                    id: f.name.as_str().to_owned(),
                    description: vec![MdParagraph::new(&f.docs)],
                }
            }
        }
    };
}

impl_mdbullet!(&EnumVariant);
impl_mdbullet!(&IntConst);
impl_mdbullet!(&FlagsMember);
impl_mdbullet!(&StructMember);
impl_mdbullet!(&UnionVariant);

impl From<&TypeRef> for MdBullet {
    fn from(t: &TypeRef) -> Self {
        Self {
            id: t.type_name().to_owned(),
            description: vec![],
        }
    }
}

impl TypeRef {
    pub fn type_name(&self) -> String {
        match self {
            TypeRef::Name(n) => n.name.as_str().to_string(),
            TypeRef::Value(ref v) => match &**v {
                Type::Array(a) => format!("Array<{}>", a.type_name()),
                Type::Pointer(p) => format!("Pointer<{}>", p.type_name()),
                Type::ConstPointer(p) => format!("ConstPointer<{}>", p.type_name()),
                Type::Builtin(b) => b.type_name().to_string(),
                Type::Enum { .. }
                | Type::Int { .. }
                | Type::Flags { .. }
                | Type::Struct { .. }
                | Type::Union { .. }
                | Type::Handle { .. } => {
                    unimplemented!("type_name of anonymous compound datatypes")
                }
            },
        }
    }
}

impl IntRepr {
    fn type_name(&self) -> &'static str {
        match self {
            IntRepr::U8 => "u8",
            IntRepr::U16 => "u16",
            IntRepr::U32 => "u32",
            IntRepr::U64 => "u64",
        }
    }
}

impl ToMarkdown for Module {
    fn gen(&self, parent: Option<Weak<MdElement>>) -> Rc<MdElement> {
        let sec = Rc::new(MdSection::new(
            self.name.as_str(),
            &format!("`{}`", self.name.as_str()),
            parent,
        ));
        let imports = Rc::new(MdSection::new(
            "imports",
            "Imports",
            Some(Rc::downgrade(&sec)),
        ));
        // TODO
        // This should probably be done using something more specific
        // than a generic MdSection
        for import in self.imports() {
            let desc = match import.variant {
                ModuleImportVariant::Memory => format!("* {}: Memory", import.name.as_str()),
            };
            imports
                .as_section_mut()
                .description
                .push(MdParagraph::new(desc));
        }
        sec.as_section_mut().elements.push(imports);
        let funcs = Rc::new(MdSection::new(
            "functions",
            "Functions",
            Some(Rc::downgrade(&sec)),
        ));
        funcs
            .as_section_mut()
            .elements
            .extend(self.funcs().map(|f| f.gen(Some(Rc::downgrade(&funcs)))));
        sec.as_section_mut().elements.push(funcs);
        sec
    }
}

impl ToMarkdown for InterfaceFunc {
    fn gen(&self, parent: Option<Weak<MdElement>>) -> Rc<MdElement> {
        let func = Rc::new(MdInterfaceFunc::new(self.name.as_str(), parent));
        func.as_interface_func_mut()
            .description
            .push(MdParagraph::new(&self.docs));
        func.as_interface_func_mut()
            .parameters
            .extend(self.params.iter().map(MdBullet::from));
        func.as_interface_func_mut()
            .results
            .extend(self.results.iter().map(MdBullet::from));
        func
    }
}

impl From<&InterfaceFuncParam> for MdBullet {
    fn from(param: &InterfaceFuncParam) -> Self {
        Self {
            id: param.name.as_str().to_owned(),
            description: vec![
                MdParagraph::new(format!(
                    "`{}` has type `{}`",
                    param.name.as_str(),
                    param.tref.type_name(),
                )),
                MdParagraph::new(format!("{}", param.docs)),
            ],
        }
    }
}

// TODO
// Implement ToMarkdown for Polyfill
impl Documentation for Polyfill {
    fn to_md(&self) -> String {
        let module_docs = self
            .modules
            .iter()
            .map(|m| m.to_md())
            .collect::<Vec<String>>()
            .join("\n");
        let type_docs = self
            .type_polyfills()
            .iter()
            .filter_map(|t| {
                if t.repeq() == RepEquality::Eq {
                    None
                } else {
                    Some(t.to_md())
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "# Modules\n{}\n# Type Conversions\n{}\n",
            module_docs, type_docs
        )
    }
}

impl Documentation for ModulePolyfill {
    fn to_md(&self) -> String {
        format!(
            "## `{}` in terms of `{}`\n{}",
            self.new.name.as_str(),
            self.old.name.as_str(),
            self.funcs
                .iter()
                .map(|f| f.to_md())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl Documentation for FuncPolyfill {
    fn to_md(&self) -> String {
        if self.full_compat() {
            format!("* `{}`: full compatibility", self.new.name.as_str())
        } else {
            let name = if self.new.name != self.old.name {
                format!(
                    "* `{}` => `{}`",
                    self.old.name.as_str(),
                    self.new.name.as_str()
                )
            } else {
                format!("* `{}`", self.new.name.as_str())
            };
            let mut contents = Vec::new();
            for p in self.mapped_params.iter() {
                contents.push(if !p.full_compat() {
                    format!("param {}", p.to_md())
                } else {
                    format!("param `{}`: compatible", p.new.name.as_str())
                })
            }
            for u in self.unknown_params.iter() {
                contents.push(format!(
                    "{} param `{}`: no corresponding result!",
                    u.which(),
                    u.param().name.as_str()
                ))
            }
            for r in self.mapped_results.iter() {
                contents.push(if !r.full_compat() {
                    format!("result {}", r.to_md())
                } else {
                    format!("result `{}`: compatible", r.new.name.as_str())
                })
            }
            for u in self.unknown_results.iter() {
                contents.push(format!(
                    "{} result `{}`: no corresponding result!",
                    u.which(),
                    u.param().name.as_str()
                ))
            }
            let contents = if contents.is_empty() {
                String::new()
            } else {
                format!(":\n    - {}", contents.join("\n    - "))
            };
            format!("{}{}", name, contents)
        }
    }
}

impl Documentation for ParamPolyfill {
    fn to_md(&self) -> String {
        let name = if self.new.name != self.old.name {
            format!(
                "`{}` => `{}`",
                self.old.name.as_str(),
                self.new.name.as_str()
            )
        } else {
            format!("`{}`", self.new.name.as_str())
        };
        let repr = match self.repeq() {
            RepEquality::Eq => "compatible types".to_string(),
            RepEquality::Superset => format!(
                "`{}` is superset-compatible with `{}`",
                self.old.tref.type_name(),
                self.new.tref.type_name()
            ),
            RepEquality::NotEq => format!(
                "`{}` is incompatible with new `{}`",
                self.old.tref.type_name(),
                self.new.tref.type_name()
            ),
        };
        format!("{}: {}", name, repr)
    }
}

impl Documentation for TypePolyfill {
    fn to_md(&self) -> String {
        fn repeq_name(r: RepEquality) -> &'static str {
            match r {
                RepEquality::Eq => ": compatible",
                RepEquality::Superset => ": superset",
                RepEquality::NotEq => "",
            }
        }
        match self {
            TypePolyfill::OldToNew(o, n) => format!(
                "* old `{}` => new `{}`{}",
                o.type_name(),
                n.type_name(),
                repeq_name(self.repeq())
            ),
            TypePolyfill::NewToOld(n, o) => format!(
                "* new `{}` => old `{}`{}",
                n.type_name(),
                o.type_name(),
                repeq_name(self.repeq())
            ),
        }
    }
}
