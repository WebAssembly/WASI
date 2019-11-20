use crate::ast::*;

pub trait Documentation {
    fn to_md(&self) -> String;
}

impl Documentation for Document {
    fn to_md(&self) -> String {
        let mut ret = "# Types\n".to_string();
        for d in self.datatypes() {
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
    fn type_name(&self) -> &'static str {
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

impl Documentation for Datatype {
    fn to_md(&self) -> String {
        if let Some(name) = &self.name {
            format!(
                "## `{}`\n{}\n{}\n",
                name.as_str(),
                self.docs,
                self.variant.to_md()
            )
        } else {
            unreachable!("should only produce Documentation for named datatypes")
        }
    }
}

impl Datatype {
    fn type_name(&self) -> String {
        self.name
            .clone()
            .map(|n| n.as_str().to_string())
            .unwrap_or_else(|| match &self.variant {
                DatatypeVariant::Array(a) => format!("Array<{}>", a.type_name()),
                DatatypeVariant::Pointer(p) => format!("Pointer<{}>", p.type_name()),
                DatatypeVariant::ConstPointer(p) => format!("ConstPointer<{}>", p.type_name()),
                DatatypeVariant::Builtin(b) => b.type_name().to_string(),
                DatatypeVariant::Alias { .. }
                | DatatypeVariant::Enum { .. }
                | DatatypeVariant::Flags { .. }
                | DatatypeVariant::Struct { .. }
                | DatatypeVariant::Union { .. }
                | DatatypeVariant::Handle { .. } => {
                    unreachable!("these variants should always be named")
                }
            })
    }
}

impl Documentation for DatatypeVariant {
    fn to_md(&self) -> String {
        match self {
            DatatypeVariant::Alias(a) => a.to_md(),
            DatatypeVariant::Enum(a) => a.to_md(),
            DatatypeVariant::Flags(a) => a.to_md(),
            DatatypeVariant::Struct(a) => a.to_md(),
            DatatypeVariant::Union(a) => a.to_md(),
            DatatypeVariant::Handle(a) => a.to_md(),
            DatatypeVariant::Array{..} |
            DatatypeVariant::Pointer{..} |
            DatatypeVariant::ConstPointer{..} |
            DatatypeVariant::Builtin{..} => unreachable!("should only produce Documentation for other variants"),
        }
    }
}

impl Documentation for AliasDatatype {
    fn to_md(&self) -> String {
        format!("Alias to `{}`", self.to.type_name())
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
                    m.type_.type_name(),
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
                    v.type_.type_name(),
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
                    type_ = f.type_.type_name(),
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
                    type_ = f.type_.type_name(),
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
