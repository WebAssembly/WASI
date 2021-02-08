use crate::{
    io::{Filesystem, WitxIo},
    parser::{
        CommentSyntax, DeclSyntax, Documented, EnumSyntax, FlagsSyntax, HandleSyntax,
        ImportTypeSyntax, ModuleDeclSyntax, RecordSyntax, TypedefSyntax, UnionSyntax,
        VariantSyntax,
    },
    BuiltinType, Case, Constant, Definition, Entry, HandleDatatype, Id, IntRepr, InterfaceFunc,
    InterfaceFuncParam, InterfaceFuncParamPosition, Location, Module, ModuleDefinition,
    ModuleEntry, ModuleImport, ModuleImportVariant, NamedType, RecordDatatype, RecordMember, Type,
    TypePassedBy, TypeRef, Variant,
};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Unknown name `{name}`")]
    UnknownName { name: String, location: Location },
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
    #[error("First result type must be pass-by-value")]
    InvalidFirstResultType { location: Location },
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
            | InvalidFirstResultType { location, .. }
            | AnonymousRecord { location, .. }
            | UnionSizeMismatch { location, .. }
            | InvalidUnionField { location, .. }
            | InvalidUnionTag { location, .. } => {
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

pub struct DocValidation {
    scope: IdentValidation,
    pub entries: HashMap<Id, Entry>,
    constant_scopes: HashMap<Id, IdentValidation>,
}

pub struct DocValidationScope<'a> {
    doc: &'a mut DocValidation,
    text: &'a str,
    path: &'a Path,
}

impl DocValidation {
    pub fn new() -> Self {
        Self {
            scope: IdentValidation::new(),
            entries: HashMap::new(),
            constant_scopes: HashMap::new(),
        }
    }

    pub fn scope<'a>(&'a mut self, text: &'a str, path: &'a Path) -> DocValidationScope<'a> {
        DocValidationScope {
            doc: self,
            text,
            path,
        }
    }
}

impl DocValidationScope<'_> {
    fn location(&self, span: wast::Span) -> Location {
        // Wast Span gives 0-indexed lines and columns. Location is 1-indexed.
        let (line, column) = span.linecol_in(self.text);
        Location {
            line: line + 1,
            column: column + 1,
            path: self.path.to_path_buf(),
        }
    }

    fn introduce(&mut self, name: &wast::Id<'_>) -> Result<Id, ValidationError> {
        let loc = self.location(name.span());
        self.doc.scope.introduce(name.name(), loc)
    }

    fn get(&self, name: &wast::Id<'_>) -> Result<Id, ValidationError> {
        let loc = self.location(name.span());
        self.doc.scope.get(name.name(), loc)
    }

    pub fn validate_decl(
        &mut self,
        decl: &DeclSyntax,
        comments: &CommentSyntax,
        definitions: &mut Vec<Definition>,
    ) -> Result<(), ValidationError> {
        match decl {
            DeclSyntax::Typename(decl) => {
                let name = self.introduce(&decl.ident)?;
                let docs = comments.docs();
                let tref = self.validate_datatype(&decl.def, true, decl.ident.span())?;

                let rc_datatype = Rc::new(NamedType {
                    name: name.clone(),
                    tref,
                    docs,
                });
                self.doc
                    .entries
                    .insert(name.clone(), Entry::Typename(Rc::downgrade(&rc_datatype)));
                definitions.push(Definition::Typename(rc_datatype));

                if let TypedefSyntax::Flags(syntax) = &decl.def {
                    if syntax.bitflags_repr.is_some() {
                        let mut flags_scope = IdentValidation::new();
                        let ty = name;
                        for (i, flag) in syntax.flags.iter().enumerate() {
                            let name = flags_scope
                                .introduce(flag.item.name(), self.location(flag.item.span()))?;
                            let docs = flag.comments.docs();
                            definitions.push(Definition::Constant(Constant {
                                ty: ty.clone(),
                                name,
                                value: 1 << i,
                                docs,
                            }));
                        }
                    }
                }
            }

            DeclSyntax::Module(syntax) => {
                let name = self.introduce(&syntax.name)?;
                let mut module_validator = ModuleValidation::new(self);
                let decls = syntax
                    .decls
                    .iter()
                    .map(|d| module_validator.validate_decl(&d))
                    .collect::<Result<Vec<_>, _>>()?;

                let rc_module = Rc::new(Module::new(
                    name.clone(),
                    decls,
                    module_validator.entries,
                    comments.docs(),
                ));
                self.doc
                    .entries
                    .insert(name, Entry::Module(Rc::downgrade(&rc_module)));
                definitions.push(Definition::Module(rc_module));
            }

            DeclSyntax::Const(syntax) => {
                let ty = Id::new(syntax.item.ty.name());
                let loc = self.location(syntax.item.name.span());
                let scope = self
                    .doc
                    .constant_scopes
                    .entry(ty.clone())
                    .or_insert_with(IdentValidation::new);
                let name = scope.introduce(syntax.item.name.name(), loc)?;
                // TODO: validate `ty` is a integer datatype that `syntax.value`
                // fits within.
                definitions.push(Definition::Constant(Constant {
                    ty,
                    name,
                    value: syntax.item.value,
                    docs: syntax.comments.docs(),
                }));
            }
        }
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
                let i = self.get(syntax)?;
                match self.doc.entries.get(&i) {
                    Some(Entry::Typename(weak_ref)) => Ok(TypeRef::Name(
                        weak_ref.upgrade().expect("weak backref to defined type"),
                    )),
                    Some(e) => Err(ValidationError::WrongKindName {
                        name: i.as_str().to_string(),
                        location: self.location(syntax.span()),
                        expected: "datatype",
                        got: e.kind(),
                    }),
                    None => Err(ValidationError::Recursive {
                        name: i.as_str().to_string(),
                        location: self.location(syntax.span()),
                    }),
                }
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
                TypedefSyntax::Flags(syntax) => self.validate_flags(&syntax, span)?,
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

    fn validate_flags(
        &self,
        syntax: &FlagsSyntax,
        span: wast::Span,
    ) -> Result<Type, ValidationError> {
        Ok(match &syntax.bitflags_repr {
            Some(repr) => Type::Builtin(self.validate_int_repr(repr, span)?.to_builtin()),
            None => {
                // TODO: auto-translate to a struct-of-bool-fields
                unimplemented!();
            }
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

        Ok(RecordDatatype { members })
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

        let mut names = names.map(|names| names.into_iter().collect::<HashSet<_>>());

        let cases = syntax
            .cases
            .iter()
            .map(|case| {
                let name = Id::new(case.item.name.name());
                if let Some(names) = &mut names {
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
        match &*ty.type_() {
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
            Type::Builtin(BuiltinType::U8) => return Ok((IntRepr::U8, None)),
            Type::Builtin(BuiltinType::U16) => return Ok((IntRepr::U16, None)),
            Type::Builtin(BuiltinType::U32) => return Ok((IntRepr::U32, None)),
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
        _syntax: &HandleSyntax,
        _span: wast::Span,
    ) -> Result<HandleDatatype, ValidationError> {
        Ok(HandleDatatype {})
    }

    fn validate_int_repr(
        &self,
        type_: &BuiltinType,
        span: wast::Span,
    ) -> Result<IntRepr, ValidationError> {
        match type_ {
            BuiltinType::U8 => Ok(IntRepr::U8),
            BuiltinType::U16 => Ok(IntRepr::U16),
            BuiltinType::U32 => Ok(IntRepr::U32),
            BuiltinType::U64 => Ok(IntRepr::U64),
            _ => Err(ValidationError::InvalidRepr {
                repr: type_.clone(),
                location: self.location(span),
            }),
        }
    }
}

struct ModuleValidation<'a> {
    doc: &'a DocValidationScope<'a>,
    scope: IdentValidation,
    pub entries: HashMap<Id, ModuleEntry>,
}

impl<'a> ModuleValidation<'a> {
    fn new(doc: &'a DocValidationScope<'a>) -> Self {
        Self {
            doc,
            scope: IdentValidation::new(),
            entries: HashMap::new(),
        }
    }

    fn validate_decl(
        &mut self,
        decl: &Documented<ModuleDeclSyntax>,
    ) -> Result<ModuleDefinition, ValidationError> {
        match &decl.item {
            ModuleDeclSyntax::Import(syntax) => {
                let loc = self.doc.location(syntax.name_loc);
                let name = self.scope.introduce(syntax.name, loc)?;
                let variant = match syntax.type_ {
                    ImportTypeSyntax::Memory => ModuleImportVariant::Memory,
                };
                let rc_import = Rc::new(ModuleImport {
                    name: name.clone(),
                    variant,
                    docs: decl.comments.docs(),
                });
                self.entries
                    .insert(name, ModuleEntry::Import(Rc::downgrade(&rc_import)));
                Ok(ModuleDefinition::Import(rc_import))
            }
            ModuleDeclSyntax::Func(syntax) => {
                let loc = self.doc.location(syntax.export_loc);
                let name = self.scope.introduce(syntax.export, loc)?;
                let mut argnames = IdentValidation::new();
                let params = syntax
                    .params
                    .iter()
                    .enumerate()
                    .map(|(ix, f)| {
                        Ok(InterfaceFuncParam {
                            name: argnames.introduce(
                                f.item.name.name(),
                                self.doc.location(f.item.name.span()),
                            )?,
                            tref: self.doc.validate_datatype(
                                &f.item.type_,
                                false,
                                f.item.name.span(),
                            )?,
                            position: InterfaceFuncParamPosition::Param(ix),
                            docs: f.comments.docs(),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let results = syntax
                    .results
                    .iter()
                    .enumerate()
                    .map(|(ix, f)| {
                        let tref =
                            self.doc
                                .validate_datatype(&f.item.type_, false, f.item.name.span())?;
                        if ix == 0 {
                            match tref.type_().passed_by() {
                                TypePassedBy::Value(_) => {}
                                _ => Err(ValidationError::InvalidFirstResultType {
                                    location: self.doc.location(f.item.name.span()),
                                })?,
                            }
                        }
                        Ok(InterfaceFuncParam {
                            name: argnames.introduce(
                                f.item.name.name(),
                                self.doc.location(f.item.name.span()),
                            )?,
                            tref,
                            position: InterfaceFuncParamPosition::Result(ix),
                            docs: f.comments.docs(),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let noreturn = syntax.noreturn;

                let rc_func = Rc::new(InterfaceFunc {
                    name: name.clone(),
                    params,
                    results,
                    noreturn,
                    docs: decl.comments.docs(),
                });
                self.entries
                    .insert(name, ModuleEntry::Func(Rc::downgrade(&rc_func)));
                Ok(ModuleDefinition::Func(rc_func))
            }
        }
    }
}
