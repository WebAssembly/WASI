use crate::ast::*;

pub trait Documentation {
    fn to_md(&self) -> String;
}

impl Documentation for Document {
    fn to_md(&self) -> String {
        let mut ret = "# Types\n".to_string();
        for d in self.typenames() {
            ret += &d.to_md();
        }

        ret += "\n# Modules\n";
        for m in self.modules() {
            ret += &m.to_md();
        }
        ret
    }
}

impl BuiltinType {
    pub fn type_name(&self) -> &'static str {
        match self {
            BuiltinType::String => "string",
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
    fn to_md(&self) -> String {
        let body = match &self.tref {
            TypeRef::Value(v) => match &**v {
                Type::Enum(a) => a.to_md(),
                Type::Flags(a) => a.to_md(),
                Type::Struct(a) => a.to_md(),
                Type::Union(a) => a.to_md(),
                Type::Handle(a) => a.to_md(),
                Type::Array(a) => format!("Array of {}", a.type_name()),
                Type::Pointer(a) => format!("Pointer to {}", a.type_name()),
                Type::ConstPointer(a) => format!("Constant Pointer to {}", a.type_name()),
                Type::Builtin(a) => format!("Builtin type {}", a.type_name()),
            },
            TypeRef::Name(n) => format!("Alias to {}", n.name.as_str()),
        };
        format!("## `{}`\n{}\n{}\n", self.name.as_str(), self.docs, body,)
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

impl Documentation for EnumDatatype {
    fn to_md(&self) -> String {
        let variants = self
            .variants
            .iter()
            .map(|v| format!("#### `{}`\n{}", v.name.as_str(), v.docs))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "Enum represented by `{}`\n\n### Variants:\n{}\n",
            self.repr.type_name(),
            variants
        )
    }
}

impl Documentation for FlagsDatatype {
    fn to_md(&self) -> String {
        let flags = self
            .flags
            .iter()
            .map(|f| format!("#### `{}`\n{}\n", f.name.as_str(), f.docs))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "Flags represented by `{}`\n\n### Flags:\n{}",
            self.repr.type_name(),
            flags
        )
    }
}

impl Documentation for StructDatatype {
    fn to_md(&self) -> String {
        let members = self
            .members
            .iter()
            .map(|m| {
                format!(
                    "#### `{}`\nMember type: `{}`\n{}",
                    m.name.as_str(),
                    m.tref.type_name(),
                    m.docs,
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("### Struct members:\n{}", members)
    }
}

impl Documentation for UnionDatatype {
    fn to_md(&self) -> String {
        let variants = self
            .variants
            .iter()
            .map(|v| {
                format!(
                    "#### `{}`\nVariant type: `{}`\n{}",
                    v.name.as_str(),
                    v.tref.type_name(),
                    v.docs,
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("### Union variants:\n{}\n", variants)
    }
}

impl Documentation for HandleDatatype {
    fn to_md(&self) -> String {
        let supertypes = self
            .supertypes
            .iter()
            .map(|s| format!("* {}", s.type_name()))
            .collect::<Vec<String>>()
            .join("\n");
        format!("### Handle supertypes:\n{}\n", supertypes)
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
    fn to_md(&self) -> String {
        let imports = self
            .imports()
            .map(|i| i.to_md())
            .collect::<Vec<String>>()
            .join("\n");
        let funcs = self
            .funcs()
            .map(|i| i.to_md())
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "## `{}`\n### Imports\n{}\n### Functions\n{}",
            self.name.as_str(),
            imports,
            funcs,
        )
    }
}

impl Documentation for ModuleImport {
    fn to_md(&self) -> String {
        match self.variant {
            ModuleImportVariant::Memory => format!("* {}: Memory", self.name.as_str()),
        }
    }
}

impl Documentation for InterfaceFunc {
    fn to_md(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|f| {
                format!(
                    "##### `{name}`\n`{name}` has type `{type_}`\n{docs}",
                    name = f.name.as_str(),
                    type_ = f.tref.type_name(),
                    docs = f.docs
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        let results = self
            .results
            .iter()
            .map(|f| {
                format!(
                    "##### `{name}`\n`{name}` has type `{type_}`\n{docs}",
                    name = f.name.as_str(),
                    type_ = f.tref.type_name(),
                    docs = f.docs
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "### {}\n{}\n#### Parameters:\n{}#### Results:\n{}",
            self.name.as_str(),
            self.docs,
            params,
            results,
        )
    }
}
