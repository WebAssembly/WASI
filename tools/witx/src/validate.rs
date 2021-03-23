use crate::{
    io::{Filesystem, WitxIo},
    parser::{
        CommentSyntax, DeclSyntax, EnumSyntax, ExpectedSyntax, FlagsSyntax, FunctionSyntax,
        HandleSyntax, RecordSyntax, TupleSyntax, TypedefSyntax, UnionSyntax, UseSyntax, UsedNames,
        VariantSyntax,
    },
    Abi, BuiltinType, Case, Constant, Function, HandleDatatype, Id, IntRepr, Location, Module,
    ModuleId, NamedType, Param, RecordDatatype, RecordKind, RecordMember, Resource, ResourceId,
    Type, TypeRef, Variant,
};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Unknown name `{name}`")]
    UnknownName { name: String, location: Location },
    #[error("module definition cycle detected")]
    CyclicModule { location: Location },
    #[error("Redefinition of name `{name}`")]
    NameAlreadyExists {
        name: String,
        at_location: Location,
        previous_location: Location,
    },
    #[error("Wrong kind of name `{name}`: expected {expected}, got {got}")]
    WrongKindName {
        name: String,
        location: Location,
        expected: &'static str,
        got: &'static str,
    },
    #[error("Recursive definition of name `{name}`")]
    Recursive { name: String, location: Location },
    #[error("Invalid representation `{repr:?}`")]
    InvalidRepr {
        repr: BuiltinType,
        location: Location,
    },
    #[error("ABI error: {reason}")]
    Abi { reason: String, location: Location },
    #[error("Anonymous structured types (struct, union, enum, flags, handle) are not permitted")]
    AnonymousRecord { location: Location },
    #[error("Union expected {expected} variants, found {found}")]
    UnionSizeMismatch {
        expected: usize,
        found: usize,
        location: Location,
    },
    #[error("Invalid union tag: {reason}")]
    InvalidUnionTag { reason: String, location: Location },
    #[error("Invalid union field `{name}`: {reason}")]
    InvalidUnionField {
        name: String,
        reason: String,
        location: Location,
    },
}

impl ValidationError {
    pub fn report_with(&self, witxio: &dyn WitxIo) -> String {
        use ValidationError::*;
        match self {
            UnknownName { location, .. }
            | WrongKindName { location, .. }
            | Recursive { location, .. }
            | InvalidRepr { location, .. }
            | Abi { location, .. }
            | AnonymousRecord { location, .. }
            | UnionSizeMismatch { location, .. }
            | InvalidUnionField { location, .. }
            | InvalidUnionTag { location, .. }
            | CyclicModule { location } => {
                format!("{}\n{}", location.highlight_source_with(witxio), &self)
            }
            NameAlreadyExists {
                at_location,
                previous_location,
                ..
            } => format!(
                "{}\n{}\nOriginally defined at:\n{}",
                at_location.highlight_source_with(witxio),
                &self,
                previous_location.highlight_source_with(witxio),
            ),
        }
    }
    pub fn report(&self) -> String {
        self.report_with(&Filesystem)
    }
}

struct IdentValidation {
    names: HashMap<String, Location>,
}

impl IdentValidation {
    fn new() -> Self {
        Self {
            names: HashMap::new(),
        }
    }

    fn introduce(&mut self, syntax: &str, location: Location) -> Result<Id, ValidationError> {
        if let Some(introduced) = self.names.get(syntax) {
            Err(ValidationError::NameAlreadyExists {
                name: syntax.to_string(),
                at_location: location,
                previous_location: introduced.clone(),
            })
        } else {
            self.names.insert(syntax.to_string(), location);
            Ok(Id::new(syntax))
        }
    }

    fn get(&self, syntax: &str, location: Location) -> Result<Id, ValidationError> {
        if self.names.get(syntax).is_some() {
            Ok(Id::new(syntax))
        } else {
            Err(ValidationError::UnknownName {
                name: syntax.to_string(),
                location,
            })
        }
    }
}

pub struct ModuleValidation<'a> {
    module: Module,
    type_ns: IdentValidation,
    resource_ns: IdentValidation,
    func_ns: IdentValidation,
    constant_ns: HashMap<Id, IdentValidation>,
    bool_ty: TypeRef,
    text: &'a str,
    path: &'a Path,
}

impl<'a> ModuleValidation<'a> {
    pub fn new(text: &'a str, path: &'a Path) -> Self {
        let name = Id::new(path.file_stem().unwrap().to_str().unwrap());
        let module_id = ModuleId(Rc::new(path.to_path_buf()));
        Self {
            module: Module::new(name, module_id),
            type_ns: IdentValidation::new(),
            resource_ns: IdentValidation::new(),
            func_ns: IdentValidation::new(),
            constant_ns: HashMap::new(),
            bool_ty: TypeRef::Value(Rc::new(Type::Variant(Variant {
                tag_repr: IntRepr::U32,
                cases: vec![
                    Case {
                        name: Id::new("false"),
                        tref: None,
                        docs: String::new(),
                    },
                    Case {
                        name: Id::new("true"),
                        tref: None,
                        docs: String::new(),
                    },
                ],
            }))),
            text,
            path,
        }
    }

    pub fn into_module(self) -> Module {
        self.module
    }

    pub fn location(&self, span: wast::Span) -> Location {
        // Wast Span gives 0-indexed lines and columns. Location is 1-indexed.
        let (line, column) = span.linecol_in(self.text);
        Location {
            line: line + 1,
            column: column + 1,
            path: self.path.to_path_buf(),
        }
    }

    pub fn validate_use(
        &mut self,
        use_: UseSyntax<'_>,
        module: &Module,
    ) -> Result<(), ValidationError> {
        match use_.names {
            UsedNames::All(span) => {
                for ty in module.typenames() {
                    let loc = self.location(span);
                    self.type_ns.introduce(ty.name.as_str(), loc)?;
                    self.module.push_type(ty.clone());
                }
                for r in module.resources() {
                    let loc = self.location(span);
                    self.resource_ns.introduce(r.name.as_str(), loc)?;
                    self.module.push_resource(r.clone());
                }
            }
            UsedNames::List(names) => {
                for name in names {
                    let mut used = false;
                    let id = Id::new(name.other_name.name());
                    let other_loc = self.location(name.other_name.span());
                    let our_loc = self.location(name.our_name.span());

                    if let Some(ty) = module.typename(&id) {
                        let id = self
                            .type_ns
                            .introduce(name.our_name.name(), our_loc.clone())?;
                        let ty = if name.other_name.name() == name.our_name.name() {
                            ty
                        } else {
                            Rc::new(NamedType {
                                name: id,
                                module: self.module.module_id().clone(),
                                tref: TypeRef::Name(ty),
                                docs: String::new(),
                            })
                        };
                        self.module.push_type(ty);
                        used = true;
                    }

                    if let Some(r) = module.resource(&id) {
                        let id = self.resource_ns.introduce(name.our_name.name(), our_loc)?;
                        let r = if name.other_name.name() == name.our_name.name() {
                            r
                        } else {
                            Rc::new(Resource {
                                name: id,
                                resource_id: r.resource_id.clone(),
                                docs: String::new(),
                            })
                        };
                        self.module.push_resource(r);
                        used = true;
                    }

                    if !used {
                        return Err(ValidationError::UnknownName {
                            name: name.other_name.name().to_string(),
                            location: other_loc,
                        }
                        .into());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn validate_decl(
        &mut self,
        decl: &DeclSyntax,
        comments: &CommentSyntax,
    ) -> Result<(), ValidationError> {
        match decl {
            DeclSyntax::Typename(decl) => {
                let loc = self.location(decl.ident.span());
                let name = self.type_ns.introduce(decl.ident.name(), loc)?;
                let docs = comments.docs();
                let tref = self.validate_datatype(&decl.def, true, decl.ident.span())?;

                self.module.push_type(Rc::new(NamedType {
                    name,
                    module: self.module.module_id().clone(),
                    tref,
                    docs,
                }));
            }

            DeclSyntax::Resource(decl) => {
                let loc = self.location(decl.ident.span());
                let name = self.resource_ns.introduce(decl.ident.name(), loc)?;
                let docs = comments.docs();

                self.module.push_resource(Rc::new(Resource {
                    name: name.clone(),
                    resource_id: ResourceId {
                        name,
                        module_id: self.module.module_id().clone(),
                    },
                    docs,
                }));
            }

            DeclSyntax::Const(syntax) => {
                let ty = Id::new(syntax.item.ty.name());
                let loc = self.location(syntax.item.name.span());
                let scope = self
                    .constant_ns
                    .entry(ty.clone())
                    .or_insert_with(IdentValidation::new);
                let name = scope.introduce(syntax.item.name.name(), loc)?;
                // TODO: validate `ty` is a integer datatype that `syntax.value`
                // fits within.
                self.module.push_constant(Constant {
                    ty,
                    name,
                    value: syntax.item.value,
                    docs: syntax.comments.docs(),
                });
            }
        }
        Ok(())
    }

    pub fn validate_function(
        &mut self,
        syntax: &FunctionSyntax,
        comments: &CommentSyntax,
    ) -> Result<(), ValidationError> {
        let loc = self.location(syntax.export_loc);
        let name = self.func_ns.introduce(syntax.export, loc)?;
        let mut argnames = IdentValidation::new();
        let params = syntax
            .params
            .iter()
            .map(|f| {
                Ok(Param {
                    name: argnames
                        .introduce(f.item.name.name(), self.location(f.item.name.span()))?,
                    tref: self.validate_datatype(&f.item.type_, false, f.item.name.span())?,
                    docs: f.comments.docs(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let results = syntax
            .results
            .iter()
            .map(|f| {
                let tref = self.validate_datatype(&f.item.type_, false, f.item.name.span())?;
                Ok(Param {
                    name: argnames
                        .introduce(f.item.name.name(), self.location(f.item.name.span()))?,
                    tref,
                    docs: f.comments.docs(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let noreturn = syntax.noreturn;
        let abi = Abi::Preview1;
        abi.validate(&params, &results)
            .map_err(|reason| ValidationError::Abi {
                reason,
                location: self.location(syntax.export_loc),
            })?;
        self.module.push_func(Rc::new(Function {
            abi,
            name: name.clone(),
            params,
            results,
            noreturn,
            docs: comments.docs(),
        }));
        Ok(())
    }

    fn validate_datatype(
        &self,
        syntax: &TypedefSyntax,
        named: bool,
        span: wast::Span,
    ) -> Result<TypeRef, ValidationError> {
        match syntax {
            TypedefSyntax::Ident(syntax) => {
                let loc = self.location(syntax.span());
                let i = self.type_ns.get(syntax.name(), loc)?;
                let ty = self.module.typename(&i).unwrap();
                Ok(TypeRef::Name(ty))
            }
            TypedefSyntax::Enum { .. }
            | TypedefSyntax::Flags { .. }
            | TypedefSyntax::Record { .. }
            | TypedefSyntax::Union { .. }
            | TypedefSyntax::Handle { .. }
                if !named =>
            {
                Err(ValidationError::AnonymousRecord {
                    location: self.location(span),
                })
            }
            other => Ok(TypeRef::Value(Rc::new(match other {
                TypedefSyntax::Enum(syntax) => Type::Variant(self.validate_enum(&syntax, span)?),
                TypedefSyntax::Tuple(syntax) => Type::Record(self.validate_tuple(&syntax, span)?),
                TypedefSyntax::Expected(syntax) => {
                    Type::Variant(self.validate_expected(&syntax, span)?)
                }
                TypedefSyntax::Flags(syntax) => Type::Record(self.validate_flags(&syntax, span)?),
                TypedefSyntax::Record(syntax) => Type::Record(self.validate_record(&syntax, span)?),
                TypedefSyntax::Union(syntax) => Type::Variant(self.validate_union(&syntax, span)?),
                TypedefSyntax::Variant(syntax) => {
                    Type::Variant(self.validate_variant(&syntax, span)?)
                }
                TypedefSyntax::Handle(syntax) => Type::Handle(self.validate_handle(syntax, span)?),
                TypedefSyntax::List(syntax) => {
                    Type::List(self.validate_datatype(syntax, false, span)?)
                }
                TypedefSyntax::Pointer(syntax) => {
                    Type::Pointer(self.validate_datatype(syntax, false, span)?)
                }
                TypedefSyntax::ConstPointer(syntax) => {
                    Type::ConstPointer(self.validate_datatype(syntax, false, span)?)
                }
                TypedefSyntax::Builtin(builtin) => Type::Builtin(*builtin),
                TypedefSyntax::String => {
                    Type::List(TypeRef::Value(Rc::new(Type::Builtin(BuiltinType::Char))))
                }
                TypedefSyntax::Bool => return Ok(self.bool_ty.clone()),
                TypedefSyntax::Ident { .. } => unreachable!(),
            }))),
        }
    }

    fn validate_enum(
        &self,
        syntax: &EnumSyntax,
        span: wast::Span,
    ) -> Result<Variant, ValidationError> {
        let mut enum_scope = IdentValidation::new();
        let tag_repr = match &syntax.repr {
            Some(repr) => self.validate_int_repr(repr, span)?,
            None => IntRepr::U32,
        };
        let cases = syntax
            .members
            .iter()
            .map(|i| {
                let name = enum_scope.introduce(i.item.name(), self.location(i.item.span()))?;
                let docs = i.comments.docs();
                Ok(Case {
                    name,
                    tref: None,
                    docs,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Variant { tag_repr, cases })
    }

    fn validate_tuple(
        &self,
        syntax: &TupleSyntax,
        span: wast::Span,
    ) -> Result<RecordDatatype, ValidationError> {
        let members = syntax
            .types
            .iter()
            .enumerate()
            .map(|(i, ty)| {
                Ok(RecordMember {
                    name: Id::new(i.to_string()),
                    tref: self.validate_datatype(ty, false, span)?,
                    docs: String::new(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(RecordDatatype {
            kind: RecordKind::Tuple,
            members,
        })
    }

    fn validate_expected(
        &self,
        syntax: &ExpectedSyntax,
        span: wast::Span,
    ) -> Result<Variant, ValidationError> {
        let ok_ty = match &syntax.ok {
            Some(ok) => Some(self.validate_datatype(ok, false, span)?),
            None => None,
        };
        let err_ty = match &syntax.err {
            Some(err) => Some(self.validate_datatype(err, false, span)?),
            None => None,
        };
        Ok(Variant {
            tag_repr: IntRepr::U32,
            cases: vec![
                Case {
                    name: Id::new("ok"),
                    tref: ok_ty,
                    docs: String::new(),
                },
                Case {
                    name: Id::new("err"),
                    tref: err_ty,
                    docs: String::new(),
                },
            ],
        })
    }

    fn validate_flags(
        &self,
        syntax: &FlagsSyntax,
        span: wast::Span,
    ) -> Result<RecordDatatype, ValidationError> {
        let repr = match syntax.repr {
            Some(ty) => self.validate_int_repr(&ty, span)?,
            None => IntRepr::U32,
        };
        let mut flags_scope = IdentValidation::new();
        let mut members = Vec::new();
        for flag in syntax.flags.iter() {
            let name = flags_scope.introduce(flag.item.name(), self.location(flag.item.span()))?;
            let docs = flag.comments.docs();
            members.push(RecordMember {
                name,
                docs,
                tref: self.bool_ty.clone(),
            });
        }
        Ok(RecordDatatype {
            kind: RecordKind::Bitflags(repr),
            members,
        })
    }

    fn validate_record(
        &self,
        syntax: &RecordSyntax,
        _span: wast::Span,
    ) -> Result<RecordDatatype, ValidationError> {
        let mut member_scope = IdentValidation::new();
        let members = syntax
            .fields
            .iter()
            .map(|f| {
                let name = member_scope
                    .introduce(f.item.name.name(), self.location(f.item.name.span()))?;
                let tref = self.validate_datatype(&f.item.type_, false, f.item.name.span())?;
                let docs = f.comments.docs();
                Ok(RecordMember { name, tref, docs })
            })
            .collect::<Result<Vec<RecordMember>, _>>()?;

        Ok(RecordDatatype {
            kind: RecordKind::Other,
            members,
        })
    }

    fn validate_union(
        &self,
        syntax: &UnionSyntax,
        span: wast::Span,
    ) -> Result<Variant, ValidationError> {
        let (tag_repr, names) = self.union_tag_repr(&syntax.tag, span)?;

        if let Some(names) = &names {
            if names.len() != syntax.fields.len() {
                return Err(ValidationError::UnionSizeMismatch {
                    expected: names.len(),
                    found: syntax.fields.len(),
                    location: self.location(span),
                });
            }
        }

        let cases = syntax
            .fields
            .iter()
            .enumerate()
            .map(|(i, case)| {
                Ok(Case {
                    name: match &names {
                        Some(names) => names[i].clone(),
                        None => Id::new(i.to_string()),
                    },
                    tref: Some(self.validate_datatype(&case.item, false, span)?),
                    docs: case.comments.docs(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Variant { tag_repr, cases })
    }

    fn validate_variant(
        &self,
        syntax: &VariantSyntax,
        span: wast::Span,
    ) -> Result<Variant, ValidationError> {
        let (tag_repr, names) = self.union_tag_repr(&syntax.tag, span)?;

        if let Some(names) = &names {
            if names.len() != syntax.cases.len() {
                return Err(ValidationError::UnionSizeMismatch {
                    expected: names.len(),
                    found: syntax.cases.len(),
                    location: self.location(span),
                });
            }
        }

        let mut name_set = names
            .as_ref()
            .map(|names| names.iter().collect::<HashSet<_>>());

        let mut cases = syntax
            .cases
            .iter()
            .map(|case| {
                let name = Id::new(case.item.name.name());
                if let Some(names) = &mut name_set {
                    if !names.remove(&name) {
                        return Err(ValidationError::InvalidUnionField {
                            name: name.as_str().to_string(),
                            location: self.location(case.item.name.span()),
                            reason: format!("does not correspond to variant in tag `tag`"),
                        });
                    }
                }
                Ok(Case {
                    name: Id::new(case.item.name.name()),
                    tref: match &case.item.ty {
                        Some(ty) => {
                            Some(self.validate_datatype(ty, false, case.item.name.span())?)
                        }
                        None => None,
                    },
                    docs: case.comments.docs(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // If we have an explicit tag with an enum then that's instructing us to
        // reorder cases based on the order of the enum itself, so do that here.
        if let Some(names) = names {
            let name_pos = names
                .iter()
                .enumerate()
                .map(|(i, name)| (name, i))
                .collect::<HashMap<_, _>>();
            cases.sort_by_key(|c| name_pos[&&c.name]);
        }

        Ok(Variant { tag_repr, cases })
    }

    fn union_tag_repr(
        &self,
        tag: &Option<Box<TypedefSyntax<'_>>>,
        span: wast::Span,
    ) -> Result<(IntRepr, Option<Vec<Id>>), ValidationError> {
        let ty = match tag {
            Some(tag) => self.validate_datatype(tag, false, span)?,
            None => return Ok((IntRepr::U32, None)),
        };
        match &**ty.type_() {
            Type::Variant(e) => {
                let mut names = Vec::new();
                for c in e.cases.iter() {
                    if c.tref.is_some() {
                        return Err(ValidationError::InvalidUnionTag {
                            location: self.location(span),
                            reason: format!("all variant cases should have empty payloads"),
                        });
                    }
                    names.push(c.name.clone());
                }
                return Ok((e.tag_repr, Some(names)));
            }
            Type::Builtin(BuiltinType::U8 { .. }) => return Ok((IntRepr::U8, None)),
            Type::Builtin(BuiltinType::U16) => return Ok((IntRepr::U16, None)),
            Type::Builtin(BuiltinType::U32 { .. }) => return Ok((IntRepr::U32, None)),
            Type::Builtin(BuiltinType::U64) => return Ok((IntRepr::U64, None)),
            _ => {}
        }

        Err(ValidationError::WrongKindName {
            name: "tag".to_string(),
            location: self.location(span),
            expected: "enum or builtin",
            got: ty.type_().kind(),
        })
    }

    fn validate_handle(
        &self,
        syntax: &HandleSyntax,
        _span: wast::Span,
    ) -> Result<HandleDatatype, ValidationError> {
        let loc = self.location(syntax.resource.span());
        let name = self.resource_ns.get(syntax.resource.name(), loc)?;
        let resource = self.module.resource(&name).unwrap();
        Ok(HandleDatatype {
            resource_id: resource.resource_id.clone(),
        })
    }

    fn validate_int_repr(
        &self,
        type_: &BuiltinType,
        span: wast::Span,
    ) -> Result<IntRepr, ValidationError> {
        match type_ {
            BuiltinType::U8 { .. } => Ok(IntRepr::U8),
            BuiltinType::U16 => Ok(IntRepr::U16),
            BuiltinType::U32 { .. } => Ok(IntRepr::U32),
            BuiltinType::U64 => Ok(IntRepr::U64),
            _ => Err(ValidationError::InvalidRepr {
                repr: type_.clone(),
                location: self.location(span),
            }),
        }
    }
}
