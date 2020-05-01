use super::{
    md::{MdFunc, MdHeading, MdNamedType, MdNodeRef, MdSection, MdType, ToMarkdown},
    Documentation,
};
use crate::{
    ast::{
        BuiltinType, Document, EnumDatatype, FlagsDatatype, HandleDatatype, IntDatatype, IntRepr,
        InterfaceFunc, InterfaceFuncParam, Module, ModuleImport, ModuleImportVariant, NamedType,
        StructDatatype, Type, TypeRef, UnionDatatype,
    },
    layout::Layout,
    polyfill::{FuncPolyfill, ModulePolyfill, ParamPolyfill, Polyfill, TypePolyfill},
    RepEquality,
};

fn heading_from_node(node: &MdNodeRef, levels_down: usize) -> MdHeading {
    MdHeading::new_header(node.borrow().ancestors().len() + levels_down)
}

impl ToMarkdown for Document {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        let types = node.new_child(MdSection::new(heading, "Types"));
        for d in self.typenames() {
            let name = d.name.as_str();
            let child = types.new_child(MdNamedType::new(
                heading.new_level_down(),
                name,
                name,
                format!(
                    "{}\nSize: {}\n\nAlignment: {}\n",
                    &d.docs,
                    &d.mem_size(),
                    &d.mem_align()
                )
                .as_str(),
            ));
            d.generate(child.clone());
        }

        let modules = node.new_child(MdSection::new(heading, "Modules"));
        for d in self.modules() {
            let mut content = MdSection::new(heading.new_level_down(), d.name.as_str());
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
                node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Alias {
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
                node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Array {
                    r#type: a.type_name().to_owned(),
                })
            }
            Self::Pointer(a) => {
                node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Pointer {
                    r#type: a.type_name().to_owned(),
                })
            }
            Self::ConstPointer(a) => {
                node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::ConstPointer {
                    r#type: a.type_name().to_owned(),
                })
            }
            Self::Builtin(a) => {
                node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Builtin {
                    repr: a.type_name().to_owned(),
                })
            }
        }
    }
}

impl ToMarkdown for EnumDatatype {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Variants"));

        for variant in &self.variants {
            let name = variant.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                &variant.docs,
            ));
        }

        node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Enum {
            repr: self.repr.type_name().to_owned(),
        });
    }
}

impl ToMarkdown for IntDatatype {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Consts"));

        for r#const in &self.consts {
            let name = r#const.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let tt = MdNamedType::new(MdHeading::new_bullet(), id.as_str(), name, &r#const.docs);
            // TODO handle r#const.value
            node.new_child(tt);
        }

        node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Int {
            repr: self.repr.type_name().to_owned(),
        });
    }
}

impl ToMarkdown for FlagsDatatype {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Flags"));

        for flag in &self.flags {
            let name = flag.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                &flag.docs,
            ));
        }

        node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Flags {
            repr: self.repr.type_name().to_owned(),
        });
    }
}

impl ToMarkdown for StructDatatype {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Struct members"));

        for member_layout in &self.member_layout() {
            let member = member_layout.member;
            let offset = member_layout.offset;
            let name = member.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let n = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                format!("{}\nOffset: {}\n", &member.docs, &offset).as_str(),
            ));
            member.tref.generate(n.clone());
        }

        node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Struct);
    }
}

impl ToMarkdown for UnionDatatype {
    fn generate(&self, node: MdNodeRef) {
        // Sizes & Alignments
        let sizes_heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(sizes_heading, "Union Layout"));
        let union_layout = &self.union_layout();
        node.new_child(MdSection::new(
            MdHeading::new_bullet(),
            format!("tag_size: {}", union_layout.tag_size).as_str(),
        ));
        node.new_child(MdSection::new(
            MdHeading::new_bullet(),
            format!("tag_align: {}", union_layout.tag_align).as_str(),
        ));
        node.new_child(MdSection::new(
            MdHeading::new_bullet(),
            format!("contents_offset: {}", union_layout.contents_offset).as_str(),
        ));
        node.new_child(MdSection::new(
            MdHeading::new_bullet(),
            format!("contents_size: {}", union_layout.contents_size).as_str(),
        ));
        node.new_child(MdSection::new(
            MdHeading::new_bullet(),
            format!("contents_align: {}", union_layout.contents_align).as_str(),
        ));

        // Variants
        let variants_heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(variants_heading, "Union variants"));

        for variant in &self.variants {
            let name = variant.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let n = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                &variant.docs,
            ));
            if let Some(ref tref) = variant.tref {
                tref.generate(n.clone());
            } else {
                n.content_ref_mut::<MdNamedType>().r#type = None;
            }
        }

        node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Union);
    }
}

impl ToMarkdown for HandleDatatype {
    fn generate(&self, node: MdNodeRef) {
        // TODO this needs more work
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Supertypes"));
        node.content_ref_mut::<MdNamedType>().r#type = Some(MdType::Handle);
    }
}

impl ToMarkdown for Module {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        let imports = node.new_child(MdSection::new(heading, "Imports"));
        for import in self.imports() {
            let child = imports.new_child(MdSection::new(heading.new_level_down(), ""));
            import.generate(child.clone());
        }

        let funcs = node.new_child(MdSection::new(heading, "Functions"));
        for func in self.funcs() {
            let name = func.name.as_str();
            let child = funcs.new_child(MdFunc::new(
                heading.new_level_down(),
                name,
                name,
                &func.docs,
            ));
            func.generate(child.clone());
        }
    }
}

impl ToMarkdown for ModuleImport {
    fn generate(&self, node: MdNodeRef) {
        match self.variant {
            ModuleImportVariant::Memory => {
                node.content_ref_mut::<MdSection>().title = "Memory".to_owned();
            }
        }
    }
}

impl ToMarkdown for InterfaceFunc {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Params"));
        for param in &self.params {
            let name = param.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let child = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                param.name.as_str(),
            ));
            param.generate(child.clone());
            // TODO should this be expanded recursively instead of using flattened type names?
            node.content_ref_mut::<MdFunc>()
                .inputs
                .push((param.name.as_str().to_owned(), param.tref.type_name()));
        }

        node.new_child(MdSection::new(heading, "Results"));
        for result in &self.results {
            let name = result.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let child = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                result.name.as_str(),
            ));
            result.generate(child.clone());
            // TODO should this be expanded recursively instead of using flattened type names?
            node.content_ref_mut::<MdFunc>()
                .outputs
                .push(result.tref.type_name());
        }
    }
}

impl ToMarkdown for InterfaceFuncParam {
    fn generate(&self, node: MdNodeRef) {
        self.tref.generate(node.clone());
        node.content_ref_mut::<MdNamedType>().docs = self.docs.clone();
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
