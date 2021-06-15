use super::md::{MdFunc, MdHeading, MdNamedType, MdNodeRef, MdSection, ToMarkdown};
use crate::{ast::*, layout::Layout};
use std::collections::{btree_map::Entry, BTreeMap, HashMap};

fn heading_from_node(node: &MdNodeRef, levels_down: usize) -> MdHeading {
    MdHeading::new_header(node.borrow().ancestors().len() + levels_down)
}

pub(super) fn modules(node: MdNodeRef, all_modules: &[&Module]) {
    // Generate a set, transitively, of all types/constants used by these modules.
    let mut all_types = BTreeMap::new();
    let mut constants_by_name = HashMap::new();
    for module in all_modules {
        for ty in module.typenames() {
            all_types.insert((&ty.name, &ty.module), &**ty);
            add_types(&mut all_types, &ty.tref);
        }

        for c in module.constants() {
            constants_by_name
                .entry((&c.ty, module.module_id()))
                .or_insert(Vec::new())
                .push(c);
        }
    }

    // Then render the information for all of the types...

    let heading = heading_from_node(&node, 1);
    let types = node.new_child(MdSection::new(heading, "Types"));

    for (key, d) in all_types {
        let name = d.name.as_str();
        let child = types.new_child(MdNamedType::new(
            heading.new_level_down(),
            name,
            name,
            format!(
                "{}\nSize: {}\n\nAlignment: {}\n",
                &d.docs,
                &d.mem_size(false),
                &d.mem_align(false)
            )
            .as_str(),
        ));
        if let Some(constants) = constants_by_name.remove(&key) {
            let heading = heading_from_node(&child, 1);
            child.new_child(MdSection::new(heading, "Constants"));
            for constant in constants {
                child.new_child(MdNamedType::new(
                    MdHeading::new_bullet(),
                    format!("{}.{}", name, constant.name.as_str()).as_str(),
                    constant.name.as_str(),
                    &constant.docs,
                ));
            }
        }
        d.generate(child.clone());
    }

    let modules = node.new_child(MdSection::new(heading, "Modules"));
    for d in all_modules {
        let heading = heading.new_level_down();
        let mut content = MdSection::new(heading, d.name().as_str());
        content.id = Some(d.name().as_str().to_owned());
        let child = modules.new_child(content);

        // d.generate(child.clone());
        let heading = heading.new_level_down();
        let funcs = child
            .clone()
            .new_child(MdSection::new(heading, "Functions"));
        for func in d.funcs() {
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

    fn add_types<'a>(types: &mut BTreeMap<(&'a Id, &'a ModuleId), &'a NamedType>, ty: &'a TypeRef) {
        let ty = match ty {
            TypeRef::Name(name) => {
                match types.entry((&name.name, &name.module)) {
                    Entry::Occupied(_) => return,
                    Entry::Vacant(v) => {
                        v.insert(name);
                    }
                }
                return add_types(types, &name.tref);
            }
            TypeRef::Value(ty) => ty,
        };

        match &**ty {
            Type::Record(r) => {
                for member in r.members.iter() {
                    add_types(types, &member.tref);
                }
            }
            Type::Variant(v) => {
                for ty in v.cases.iter().filter_map(|c| c.tref.as_ref()) {
                    add_types(types, ty);
                }
            }
            Type::List(t) | Type::ConstPointer(t) | Type::Pointer(t) => add_types(types, t),
            Type::Buffer(b) => add_types(types, &b.tref),
            Type::Handle(_) | Type::Builtin(_) => {}
        }
    }
}

impl ToMarkdown for TypeRef {
    fn generate(&self, node: MdNodeRef) {
        match self {
            TypeRef::Value(v) => {
                v.generate(node.clone());
                node.content_ref_mut::<MdNamedType>().ty = Some(format!("`{}`", self.type_name()));
            }
            TypeRef::Name(n) => {
                node.content_ref_mut::<MdNamedType>().ty =
                    Some(format!("[`{0}`](#{0})", n.name.as_str().to_owned()));
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
            Self::Record(a) => a.generate(node.clone()),
            Self::Variant(a) => a.generate(node.clone()),
            Self::Handle(a) => a.generate(node.clone()),
            Self::List(_) => {}
            Self::Pointer(_) => {}
            Self::ConstPointer(_) => {}
            Self::Buffer(_) => {}
            Self::Builtin(_) => {}
        }
    }
}

impl ToMarkdown for RecordDatatype {
    fn generate(&self, node: MdNodeRef) {
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Record members"));

        for member_layout in &self.member_layout(false) {
            let member = member_layout.member;
            let offset = member_layout.offset;
            let name = member.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let offset_desc = if self.bitflags_repr().is_some() {
                "Bit"
            } else {
                "Offset"
            };
            let n = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                format!("{}\n{}: {}\n", &member.docs, offset_desc, offset).as_str(),
            ));
            member.tref.generate(n.clone());
        }
    }
}

impl ToMarkdown for Variant {
    fn generate(&self, node: MdNodeRef) {
        if self.is_bool() {
            return;
        }
        if self.cases.iter().any(|c| c.tref.is_some()) {
            let heading = heading_from_node(&node, 1);
            node.new_child(MdSection::new(heading, "Variant Layout"));

            let whole = self.mem_size_align(false);
            node.new_child(MdSection::new(
                MdHeading::new_bullet(),
                format!("size: {}", whole.size),
            ));
            node.new_child(MdSection::new(
                MdHeading::new_bullet(),
                format!("align: {}", whole.align),
            ));

            let tag = self.tag_repr.mem_size_align(false);
            node.new_child(MdSection::new(
                MdHeading::new_bullet(),
                format!("tag_size: {}", tag.size),
            ));
        }

        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Variant cases"));

        for case in self.cases.iter() {
            let name = case.name.as_str();
            let id = if let Some(id) = node.any_ref().id() {
                format!("{}.{}", id, name)
            } else {
                name.to_owned()
            };
            let n = node.new_child(MdNamedType::new(
                MdHeading::new_bullet(),
                id.as_str(),
                name,
                &case.docs,
            ));
            if let Some(ty) = &case.tref {
                ty.generate(n.clone());
            }
        }
    }
}

impl ToMarkdown for HandleDatatype {
    fn generate(&self, node: MdNodeRef) {
        // TODO this needs more work
        let heading = heading_from_node(&node, 1);
        node.new_child(MdSection::new(heading, "Supertypes"));
    }
}

impl ToMarkdown for Function {
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

impl ToMarkdown for Param {
    fn generate(&self, node: MdNodeRef) {
        self.tref.generate(node.clone());
        node.content_ref_mut::<MdNamedType>().docs = self.docs.clone();
    }
}

impl BuiltinType {
    pub fn type_name(&self) -> &'static str {
        match self {
            BuiltinType::Char => "char",
            BuiltinType::U8 { .. } => "u8",
            BuiltinType::U16 => "u16",
            BuiltinType::U32 {
                lang_ptr_size: false,
            } => "u32",
            BuiltinType::U32 {
                lang_ptr_size: true,
            } => "usize",
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
            TypeRef::Value(v) => match &**v {
                Type::List(a) => match &**a.type_() {
                    Type::Builtin(BuiltinType::Char) => "string".to_string(),
                    _ => format!("List<{}>", a.type_name()),
                },
                Type::Pointer(p) => format!("Pointer<{}>", p.type_name()),
                Type::ConstPointer(p) => format!("ConstPointer<{}>", p.type_name()),
                Type::Builtin(b) => b.type_name().to_string(),
                Type::Record(RecordDatatype {
                    kind: RecordKind::Tuple,
                    members,
                }) => {
                    let mut ret = "(".to_string();
                    for (i, member) in members.iter().enumerate() {
                        if i > 0 {
                            ret.push_str(", ");
                        }
                        ret.push_str(&member.tref.type_name());
                    }
                    ret.push_str(")");
                    ret
                }
                Type::Record { .. } => {
                    format!("Record")
                }
                Type::Handle { .. } => {
                    format!("Handle")
                }
                Type::Variant(v) => {
                    if let Some((ok, err)) = v.as_expected() {
                        let ok = match ok {
                            Some(ty) => ty.type_name(),
                            None => "()".to_string(),
                        };
                        let err = match err {
                            Some(ty) => ty.type_name(),
                            None => "()".to_string(),
                        };
                        format!("Result<{}, {}>", ok, err)
                    } else if v.is_bool() {
                        format!("bool")
                    } else {
                        format!("Variant")
                    }
                }
                Type::Buffer(_) => format!("buffer"),
            },
        }
    }
}
