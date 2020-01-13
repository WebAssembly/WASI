use super::md::*;
use super::Documentation;
use crate::ast::*;
use crate::polyfill::*;
use crate::RepEquality;

impl ToMarkdown for Document {
    fn generate(&self, node: MdNodeRef) {
        let types = node.new_child(MdSection::new("Types"));
        for d in self.typenames() {
            let child = types.new_child(MdNamedType::new(d.name.as_str(), &d.docs));
            d.generate(child.clone());
        }

        let modules = node.new_child(MdSection::new("Modules"));
        for d in self.modules() {
            let mut content = MdSection::new(d.name.as_str());
            content.id = Some(d.name.as_str().to_owned());
            let child = modules.new_child(content);
            d.generate(child.clone());
        }
    }
}

impl ToMarkdown for TypeRef {
    fn generate(&self, node: MdNodeRef) {
        match self {
            TypeRef::Value(v) => v.generate(node.clone()),
            TypeRef::Name(n) => {
                node.content_mut::<MdNamedType>().r#type = Some(MdType::Alias {
                    r#type: n.name.as_str().to_owned(),
                })
            }
        }
    }
}

impl ToMarkdown for NamedType {
    fn generate(&self, node: MdNodeRef) {
        self.tref.generate(node.clone());
    }
}

impl ToMarkdown for Type {
    fn generate(&self, node: MdNodeRef) {
        match self {
            Self::Enum(a) => a.generate(node.clone()),
            Self::Int(a) => a.generate(node.clone()),
            Self::Flags(a) => a.generate(node.clone()),
            Self::Struct(a) => a.generate(node.clone()),
            Self::Union(a) => a.generate(node.clone()),
            Self::Handle(a) => a.generate(node.clone()),
            Self::Array(a) => {
                node.content_mut::<MdNamedType>().r#type = Some(MdType::Array {
                    r#type: a.type_name().to_owned(),
                })
            }
            Self::Pointer(a) => {
                node.content_mut::<MdNamedType>().r#type = Some(MdType::Pointer {
                    r#type: a.type_name().to_owned(),
                })
            }
            Self::ConstPointer(a) => {
                node.content_mut::<MdNamedType>().r#type = Some(MdType::ConstPointer {
                    r#type: a.type_name().to_owned(),
                })
            }
            Self::Builtin(a) => {
                node.content_mut::<MdNamedType>().r#type = Some(MdType::Builtin {
                    repr: a.type_name().to_owned(),
                })
            }
        }
    }
}

impl ToMarkdown for EnumDatatype {
    fn generate(&self, node: MdNodeRef) {
        node.new_child(MdSection::new("Variants"));

        for variant in &self.variants {
            node.new_child(MdNamedType::new(variant.name.as_str(), &variant.docs));
        }

        node.content_mut::<MdNamedType>().r#type = Some(MdType::Enum {
            repr: self.repr.type_name().to_owned(),
        });
    }
}

impl ToMarkdown for IntDatatype {
    fn generate(&self, node: MdNodeRef) {
        node.new_child(MdSection::new("Consts"));

        for r#const in &self.consts {
            let tt = MdNamedType::new(r#const.name.as_str(), &r#const.docs);
            // TODO handle r#const.value
            node.new_child(tt);
        }

        node.content_mut::<MdNamedType>().r#type = Some(MdType::Int {
            repr: self.repr.type_name().to_owned(),
        });
    }
}

impl ToMarkdown for FlagsDatatype {
    fn generate(&self, node: MdNodeRef) {
        node.new_child(MdSection::new("Flags"));

        for flag in &self.flags {
            node.new_child(MdNamedType::new(flag.name.as_str(), &flag.docs));
        }

        node.content_mut::<MdNamedType>().r#type = Some(MdType::Flags {
            repr: self.repr.type_name().to_owned(),
        });
    }
}

impl ToMarkdown for StructDatatype {
    fn generate(&self, node: MdNodeRef) {
        node.new_child(MdSection::new("Struct members"));

        for member in &self.members {
            let n = node.new_child(MdNamedType::new(member.name.as_str(), &member.docs));
            member.tref.generate(n.clone());
        }

        node.content_mut::<MdNamedType>().r#type = Some(MdType::Struct);
    }
}

impl ToMarkdown for UnionDatatype {
    fn generate(&self, node: MdNodeRef) {
        node.new_child(MdSection::new("Union variants"));

        for variant in &self.variants {
            let n = node.new_child(MdNamedType::new(variant.name.as_str(), &variant.docs));
            variant.tref.generate(n.clone());
        }

        node.content_mut::<MdNamedType>().r#type = Some(MdType::Union);
    }
}

impl ToMarkdown for HandleDatatype {
    fn generate(&self, node: MdNodeRef) {
        // TODO this needs more work
        node.new_child(MdSection::new("Supertypes"));
        node.content_mut::<MdNamedType>().r#type = Some(MdType::Handle);
    }
}

impl ToMarkdown for Module {
    fn generate(&self, node: MdNodeRef) {
        let imports = node.new_child(MdSection::new("Imports"));
        for import in self.imports() {
            let child = imports.new_child(MdSection::default());
            import.generate(child.clone());
        }

        let funcs = node.new_child(MdSection::new("Functions"));
        for func in self.funcs() {
            let child = funcs.new_child(MdFunc::new(func.name.as_str(), &func.docs));
            func.generate(child.clone());
        }
    }
}

impl ToMarkdown for ModuleImport {
    fn generate(&self, node: MdNodeRef) {
        match self.variant {
            ModuleImportVariant::Memory => {
                node.content_mut::<MdSection>().title = "Memory".to_owned();
            }
        }
    }
}

impl ToMarkdown for InterfaceFunc {
    fn generate(&self, node: MdNodeRef) {
        node.new_child(MdSection::new("Params"));
        for param in &self.params {
            let child = node.new_child(MdNamedType::new(param.name.as_str(), param.name.as_str()));
            param.generate(child.clone());
            // TODO should this be expanded recursively instead of using flattened type names?
            node.content_mut::<MdFunc>()
                .inputs
                .push((param.name.as_str().to_owned(), param.tref.type_name()));
        }

        node.new_child(MdSection::new("Results"));
        for result in &self.results {
            let child =
                node.new_child(MdNamedType::new(result.name.as_str(), result.name.as_str()));
            result.generate(child.clone());
            // TODO should this be expanded recursively instead of using flattened type names?
            node.content_mut::<MdFunc>()
                .outputs
                .push(result.tref.type_name());
        }
    }
}

impl ToMarkdown for InterfaceFuncParam {
    fn generate(&self, node: MdNodeRef) {
        self.tref.generate(node.clone());
        node.content_mut::<MdNamedType>().docs = self.docs.clone();
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

// TODO
// Generate Markdown tree for the polyfill
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
