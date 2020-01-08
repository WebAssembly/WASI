use crate::ast::*;
use crate::polyfill::*;
use crate::RepEquality;
use std::fmt;

#[derive(Debug)]
pub enum MdBlockElement {
    Section(MdSection),
    TypeListing(MdTypeListing),
    InterfaceFunc(MdInterfaceFunc),
    Root(MdRoot),
}

impl fmt::Display for MdBlockElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Section(sec) => sec.fmt(f),
            Self::TypeListing(listing) => listing.fmt(f),
            Self::InterfaceFunc(func) => func.fmt(f),
            Self::Root(root) => root.fmt(f),
        }
    }
}

#[derive(Debug, Default)]
pub struct MdRoot {
    blocks: Vec<MdBlockElement>,
}

impl fmt::Display for MdRoot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in &self.blocks {
            block.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct MdParagraph {
    para: String,
}

impl MdParagraph {
    fn new<S: AsRef<str>>(para: S) -> Self {
        Self {
            para: para.as_ref().to_owned(),
        }
    }

    fn with_links_parsed<S: AsRef<str>>(para: S) -> Self {
        let to_parse = para.as_ref();
        let mut parsed = String::with_capacity(to_parse.len());
        let mut temp = String::with_capacity(to_parse.len());
        let mut is_link = false;
        let mut eaten = None;
        for ch in to_parse.chars() {
            match (ch, is_link) {
                ('`', false) => {
                    // found the beginning of a link
                    is_link = true;
                }
                ('`', true) => {
                    // reached the end, expand into a link!
                    let expanded = format!("[`{}`](#{})", temp, temp);
                    parsed.push_str(&expanded);
                    temp.drain(..);
                    is_link = false;
                }
                (':', true) => {
                    if let Some(_) = eaten {
                        // swap for '.'
                        temp.push('.');
                        eaten = None;
                    } else {
                        eaten = Some(':');
                    }
                }
                (ch, false) => parsed.push(ch),
                (ch, true) => temp.push(ch),
            }
        }
        Self::new(parsed)
    }
}

impl fmt::Display for MdParagraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}\n", &self.para))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum MdHeading {
    Header {
        id: String,
        title: String,
        doc_level: usize,
    },
    Bullet {
        id: String,
        title: String,
    },
}

impl MdHeading {
    fn new_header<S: AsRef<str>>(title: S, doc_level: usize) -> Self {
        let id = title.as_ref().replace(" ", "_");
        let title = title.as_ref().to_owned();
        Self::Header {
            id,
            title,
            doc_level,
        }
    }

    fn new_bullet<S: AsRef<str>>(title: S) -> Self {
        let id = title.as_ref().replace(" ", "_");
        let title = title.as_ref().to_owned();
        Self::Bullet { id, title }
    }

    fn id(&self) -> &str {
        match self {
            Self::Header { ref id, .. } => id,
            Self::Bullet { ref id, .. } => id,
        }
    }

    fn set_id<S: AsRef<str>>(&mut self, id: S) {
        let s_id = match self {
            Self::Header { ref mut id, .. } => id,
            Self::Bullet { ref mut id, .. } => id,
        };
        *s_id = id.as_ref().to_owned();
    }
}

impl fmt::Display for MdHeading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (id, title, heading) = match self {
            Self::Header {
                id,
                title,
                doc_level,
            } => (id, title, "#".repeat(*doc_level)),
            Self::Bullet { id, title } => (id, title, "-".to_owned()),
        };
        f.write_fmt(format_args!(
            "{heading} <a href=\"{id}\" name=\"{id}\"></a> {title}\n",
            heading = heading,
            id = id,
            title = title
        ))
    }
}

#[derive(Debug)]
pub struct MdSection {
    heading: MdHeading,
    description: Vec<MdParagraph>,
    blocks: Vec<MdBlockElement>,
}

impl MdSection {
    fn new(heading: MdHeading) -> Self {
        Self {
            heading,
            description: vec![],
            blocks: vec![],
        }
    }
}

impl fmt::Display for MdSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.heading.fmt(f)?;
        for para in &self.description {
            para.fmt(f)?;
        }
        for el in &self.blocks {
            el.fmt(f)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum MdType {
    Enum { repr: String },
    Int { repr: String },
    Flags { repr: String },
    Struct,
    Union,
    Handle,
    Array { r#type: String },
    Pointer { to: String },
    ConstPointer { to: String },
    Builtin { r#type: String },
}

impl fmt::Display for MdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_specific = match self {
            MdType::Enum { repr } => format!("Enum represented by `{}`\n\n**Variants:**", repr),
            MdType::Int { repr } => format!("Int represented by `{}`\n\n**Const:**", repr),
            MdType::Flags { repr } => format!("Flags represented by `{}`\n\n**Flags:**", repr),
            MdType::Struct => "\n**Struct members:**".to_owned(),
            MdType::Union => "\n**Union variants:**".to_owned(),
            MdType::Handle => "\n**Supertypes:**".to_owned(),
            MdType::Array { r#type } => format!("Array of `{}`", r#type),
            MdType::Pointer { to } => format!("Pointer to `{}`", to),
            MdType::ConstPointer { to } => format!("Const pointer to `{}`", to),
            MdType::Builtin { r#type } => format!("Builtin type `{}`", r#type),
        };
        f.write_fmt(format_args!("{}\n", type_specific))
    }
}

#[derive(Debug, Clone)]
struct MdListingEntry {
    heading: MdHeading,
    description: Vec<MdParagraph>,
}

impl fmt::Display for MdListingEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.heading.fmt(f)?;
        for para in &self.description {
            f.write_fmt(format_args!("\n\t{}\n", para))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct MdTypeListing {
    heading: MdHeading,
    r#type: MdType,
    description: Vec<MdParagraph>,
    entries: Vec<MdListingEntry>,
}

impl MdTypeListing {
    fn new(heading: MdHeading, r#type: MdType) -> Self {
        Self {
            heading,
            r#type,
            description: vec![],
            entries: vec![],
        }
    }
}

impl fmt::Display for MdTypeListing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.heading.fmt(f)?;
        for para in &self.description {
            para.fmt(f)?;
        }
        self.r#type.fmt(f)?;
        for el in &self.entries {
            // prepend id of this MdTypeListing in order to generate scoped
            // references of list entries
            let id = format!("{}.{}", self.heading.id(), el.heading.id());
            let mut new_el = el.clone();
            new_el.heading.set_id(id);
            new_el.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct MdInterfaceFunc {
    heading: MdHeading,
    description: Vec<MdParagraph>,
    parameters: Vec<MdListingEntry>,
    results: Vec<MdListingEntry>,
}

impl MdInterfaceFunc {
    fn new(heading: MdHeading) -> Self {
        Self {
            heading,
            description: vec![],
            parameters: vec![],
            results: vec![],
        }
    }
}

impl fmt::Display for MdInterfaceFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.heading.fmt(f)?;
        for desc in &self.description {
            desc.fmt(f)?;
        }
        f.write_str("\n**Parameters:**\n\n")?;
        for el in &self.parameters {
            // prepend id of this MdTypeListing in order to generate scoped
            // references of list entries
            let id = format!("{}.{}", self.heading.id(), el.heading.id());
            let mut new_el = el.clone();
            new_el.heading.set_id(id);
            new_el.fmt(f)?;
        }
        f.write_str("\n**Results:**\n\n")?;
        for el in &self.results {
            // prepend id of this MdTypeListing in order to generate scoped
            // references of list entries
            let id = format!("{}.{}", self.heading.id(), el.heading.id());
            let mut new_el = el.clone();
            new_el.heading.set_id(id);
            new_el.fmt(f)?;
        }
        Ok(())
    }
}

pub trait Documentation {
    fn to_md(&self) -> String {
        format!("{}", self.gen(0))
    }

    fn gen(&self, _level: usize) -> MdBlockElement {
        MdBlockElement::Root(MdRoot::default())
    }
}

impl Documentation for Document {
    fn gen(&self, level: usize) -> MdBlockElement {
        let mut doc = MdRoot::default();
        let mut types = MdSection::new(MdHeading::new_header("Types", level + 1));
        for typename in self.typenames() {
            types.blocks.push(typename.gen(level + 2));
        }
        doc.blocks.push(MdBlockElement::Section(types));
        let mut modules = MdSection::new(MdHeading::new_header("Modules", level + 1));
        for module in self.modules() {
            modules.blocks.push(module.gen(level + 2));
        }
        doc.blocks.push(MdBlockElement::Section(modules));
        MdBlockElement::Root(doc)
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

impl Documentation for NamedType {
    fn gen(&self, level: usize) -> MdBlockElement {
        match &self.tref {
            TypeRef::Value(v) => {
                let md_type = MdType::from(&**v);
                let mut entries: Vec<_> = match &**v {
                    Type::Enum(a) => a.variants.iter().map(MdListingEntry::from).collect(),
                    Type::Int(a) => a.consts.iter().map(MdListingEntry::from).collect(),
                    Type::Flags(a) => a.flags.iter().map(MdListingEntry::from).collect(),
                    Type::Struct(a) => a.members.iter().map(MdListingEntry::from).collect(),
                    Type::Union(a) => a.variants.iter().map(MdListingEntry::from).collect(),
                    Type::Handle(a) => a.supertypes.iter().map(MdListingEntry::from).collect(),
                    _ => vec![],
                };
                let mut heading = MdHeading::new_header(format!("`{}`", self.name.as_str()), level);
                heading.set_id(self.name.as_str());
                let mut listing = MdTypeListing::new(heading, md_type);
                listing
                    .description
                    .push(MdParagraph::with_links_parsed(&self.docs));
                listing.entries.append(&mut entries);
                MdBlockElement::TypeListing(listing)
            }
            TypeRef::Name(n) => {
                let mut heading = MdHeading::new_header(self.name.as_str(), level);
                heading.set_id(self.name.as_str());
                let mut sec = MdSection::new(heading);
                sec.description
                    .push(MdParagraph::with_links_parsed(&self.docs));
                sec.description.push(MdParagraph::with_links_parsed(format!(
                    "Alias to {}",
                    n.name.as_str()
                )));
                MdBlockElement::Section(sec)
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

macro_rules! impl_mdlistingentry {
    ($from:ty) => {
        impl From<$from> for MdListingEntry {
            fn from(f: $from) -> Self {
                let mut heading = MdHeading::new_bullet(format!("`{}`", f.name.as_str()));
                heading.set_id(f.name.as_str());
                Self {
                    heading,
                    description: vec![MdParagraph::with_links_parsed(&f.docs)],
                }
            }
        }
    };
}

impl_mdlistingentry!(&EnumVariant);
impl_mdlistingentry!(&IntConst);
impl_mdlistingentry!(&FlagsMember);
impl_mdlistingentry!(&StructMember);
impl_mdlistingentry!(&UnionVariant);

impl From<&TypeRef> for MdListingEntry {
    fn from(t: &TypeRef) -> Self {
        let mut heading = MdHeading::new_bullet(format!("`{}`", t.type_name()));
        heading.set_id(t.type_name());
        Self {
            heading,
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

impl Documentation for Module {
    fn gen(&self, level: usize) -> MdBlockElement {
        let mut heading = MdHeading::new_header(format!("`{}`", self.name.as_str()), level);
        heading.set_id(self.name.as_str());
        let mut sec = MdSection::new(heading);
        let mut imports = MdSection::new(MdHeading::new_header("Imports", level + 1));
        // FIXME
        for import in self.imports() {
            let desc = match import.variant {
                ModuleImportVariant::Memory => {
                    MdParagraph::with_links_parsed(format!("* {}: Memory", import.name.as_str()))
                }
            };
            imports.description.push(desc);
        }
        sec.blocks.push(MdBlockElement::Section(imports));
        let mut funcs = MdSection::new(MdHeading::new_header("Functions", level + 1));
        funcs.blocks.extend(self.funcs().map(|f| f.gen(level + 2)));
        sec.blocks.push(MdBlockElement::Section(funcs));
        MdBlockElement::Section(sec)
    }
}

impl Documentation for InterfaceFunc {
    fn gen(&self, level: usize) -> MdBlockElement {
        let mut heading = MdHeading::new_header(format!("`{}`", self.name.as_str()), level);
        heading.set_id(self.name.as_str());
        let mut func = MdInterfaceFunc::new(heading);
        func.description
            .push(MdParagraph::with_links_parsed(&self.docs));
        func.parameters
            .extend(self.params.iter().map(MdListingEntry::from));
        func.results
            .extend(self.results.iter().map(MdListingEntry::from));
        MdBlockElement::InterfaceFunc(func)
    }
}

impl From<&InterfaceFuncParam> for MdListingEntry {
    fn from(param: &InterfaceFuncParam) -> Self {
        let mut heading = MdHeading::new_bullet(format!("`{}`", param.name.as_str()));
        heading.set_id(param.name.as_str());
        let mut description = vec![];
        description.push(MdParagraph::with_links_parsed(format!(
            "`{}` has type `{}`.",
            param.name.as_str(),
            param.tref.type_name()
        )));
        description.push(MdParagraph::with_links_parsed(&param.docs));
        Self {
            heading,
            description,
        }
    }
}

// TODO
// Generate nicely-formatted docs for polyfill
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
