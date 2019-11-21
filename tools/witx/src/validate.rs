use crate::{
    io::{Filesystem, WitxIo},
    parser::{
        CommentSyntax, DeclSyntax, Documented, EnumSyntax, FlagsSyntax, HandleSyntax,
        ImportTypeSyntax, ModuleDeclSyntax, StructSyntax, TypedefSyntax, UnionSyntax,
    },
    BuiltinType, Datatype, DatatypePassedBy, DatatypeRef, Definition, Entry, EnumDatatype,
    EnumVariant, FlagsDatatype, FlagsMember, HandleDatatype, Id, IntRepr, InterfaceFunc,
    InterfaceFuncParam, InterfaceFuncParamPosition, Location, Module, ModuleDefinition,
    ModuleEntry, ModuleImport, ModuleImportVariant, NamedDatatype, StructDatatype, StructMember,
    UnionDatatype, UnionVariant,
};
use failure::Fail;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Fail)]
pub enum ValidationError {
    #[fail(display = "Unknown name `{}`", name)]
    UnknownName { name: String, location: Location },
    #[fail(display = "Redefinition of name `{}`", name)]
    NameAlreadyExists {
        name: String,
        at_location: Location,
        previous_location: Location,
    },
    #[fail(
        display = "Wrong kind of name `{}`: expected {}, got {}",
        name, expected, got
    )]
    WrongKindName {
        name: String,
        location: Location,
        expected: &'static str,
        got: &'static str,
    },
    #[fail(display = "Recursive definition of name `{}`", name)]
    Recursive { name: String, location: Location },
    #[fail(display = "Invalid representation `{:?}`", repr)]
    InvalidRepr {
        repr: BuiltinType,
        location: Location,
    },
    #[fail(display = "First result type must be pass-by-value")]
    InvalidFirstResultType { location: Location },
}

impl ValidationError {
    pub fn report_with(&self, witxio: &dyn WitxIo) -> String {
        use ValidationError::*;
        match self {
            UnknownName { location, .. }
            | WrongKindName { location, .. }
            | Recursive { location, .. }
            | InvalidRepr { location, .. }
            | InvalidFirstResultType { location, .. } => {
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
    ) -> Result<Definition, ValidationError> {
        match decl {
            DeclSyntax::Typename(decl) => {
                let name = self.introduce(&decl.ident)?;
                let docs = comments.docs();
                let dt = self.validate_datatype(&decl.def, decl.ident.span())?;

                let rc_datatype = Rc::new(NamedDatatype {
                    name: name.clone(),
                    dt,
                    docs,
                });
                self.doc
                    .entries
                    .insert(name, Entry::Datatype(Rc::downgrade(&rc_datatype)));
                Ok(Definition::Datatype(rc_datatype))
            }
            DeclSyntax::Module(syntax) => {
                let name = self.introduce(&syntax.name)?;
                let mut module_validator = ModuleValidation::new(self);
                let definitions = syntax
                    .decls
                    .iter()
                    .map(|d| module_validator.validate_decl(&d))
                    .collect::<Result<Vec<_>, _>>()?;

                let rc_module = Rc::new(Module::new(
                    name.clone(),
                    definitions,
                    module_validator.entries,
                    comments.docs(),
                ));
                self.doc
                    .entries
                    .insert(name, Entry::Module(Rc::downgrade(&rc_module)));
                Ok(Definition::Module(rc_module))
            }
        }
    }

    fn validate_datatype(
        &self,
        syntax: &TypedefSyntax,
        span: wast::Span,
    ) -> Result<DatatypeRef, ValidationError> {
        match syntax {
            TypedefSyntax::Ident(syntax) => {
                let i = self.get(syntax)?;
                match self.doc.entries.get(&i) {
                    Some(Entry::Datatype(weak_ref)) => Ok(DatatypeRef::Name(
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
            other => Ok(DatatypeRef::Value(Rc::new(match other {
                TypedefSyntax::Enum(syntax) => Datatype::Enum(self.validate_enum(&syntax, span)?),
                TypedefSyntax::Flags(syntax) => {
                    Datatype::Flags(self.validate_flags(&syntax, span)?)
                }
                TypedefSyntax::Struct(syntax) => {
                    Datatype::Struct(self.validate_struct(&syntax, span)?)
                }
                TypedefSyntax::Union(syntax) => {
                    Datatype::Union(self.validate_union(&syntax, span)?)
                }
                TypedefSyntax::Handle(syntax) => {
                    Datatype::Handle(self.validate_handle(syntax, span)?)
                }
                TypedefSyntax::Array(syntax) => {
                    Datatype::Array(self.validate_datatype(syntax, span)?)
                }
                TypedefSyntax::Pointer(syntax) => {
                    Datatype::Pointer(self.validate_datatype(syntax, span)?)
                }
                TypedefSyntax::ConstPointer(syntax) => {
                    Datatype::ConstPointer(self.validate_datatype(syntax, span)?)
                }
                TypedefSyntax::Builtin(builtin) => Datatype::Builtin(*builtin),
                TypedefSyntax::Ident { .. } => unreachable!(),
            }))),
        }
    }

    fn validate_enum(
        &self,
        syntax: &EnumSyntax,
        span: wast::Span,
    ) -> Result<EnumDatatype, ValidationError> {
        let mut enum_scope = IdentValidation::new();
        let repr = self.validate_int_repr(&syntax.repr, span)?;
        let variants = syntax
            .members
            .iter()
            .map(|i| {
                let name = enum_scope.introduce(i.item.name(), self.location(i.item.span()))?;
                let docs = i.comments.docs();
                Ok(EnumVariant { name, docs })
            })
            .collect::<Result<Vec<EnumVariant>, _>>()?;

        Ok(EnumDatatype { repr, variants })
    }

    fn validate_flags(
        &self,
        syntax: &FlagsSyntax,
        span: wast::Span,
    ) -> Result<FlagsDatatype, ValidationError> {
        let mut flags_scope = IdentValidation::new();
        let repr = self.validate_int_repr(&syntax.repr, span)?;
        let flags = syntax
            .flags
            .iter()
            .map(|i| {
                let name = flags_scope.introduce(i.item.name(), self.location(i.item.span()))?;
                let docs = i.comments.docs();
                Ok(FlagsMember { name, docs })
            })
            .collect::<Result<Vec<FlagsMember>, _>>()?;

        Ok(FlagsDatatype { repr, flags })
    }

    fn validate_struct(
        &self,
        syntax: &StructSyntax,
        _span: wast::Span,
    ) -> Result<StructDatatype, ValidationError> {
        let mut member_scope = IdentValidation::new();
        let members = syntax
            .fields
            .iter()
            .map(|f| {
                let name = member_scope
                    .introduce(f.item.name.name(), self.location(f.item.name.span()))?;
                let type_ = self.validate_datatype(&f.item.type_, f.item.name.span())?;
                let docs = f.comments.docs();
                Ok(StructMember { name, type_, docs })
            })
            .collect::<Result<Vec<StructMember>, _>>()?;

        Ok(StructDatatype { members })
    }

    fn validate_union(
        &self,
        syntax: &UnionSyntax,
        _span: wast::Span,
    ) -> Result<UnionDatatype, ValidationError> {
        let mut variant_scope = IdentValidation::new();
        let variants = syntax
            .fields
            .iter()
            .map(|f| {
                let name = variant_scope
                    .introduce(f.item.name.name(), self.location(f.item.name.span()))?;
                let type_ = self.validate_datatype(&f.item.type_, f.item.name.span())?;
                let docs = f.comments.docs();
                Ok(UnionVariant { name, type_, docs })
            })
            .collect::<Result<Vec<UnionVariant>, _>>()?;

        Ok(UnionDatatype { variants })
    }

    fn validate_handle(
        &self,
        syntax: &HandleSyntax,
        _span: wast::Span,
    ) -> Result<HandleDatatype, ValidationError> {
        let supertypes = syntax
            .supertypes
            .iter()
            .map(|id_syntax| {
                let id = self.get(&id_syntax)?;
                match self.doc.entries.get(&id) {
                    Some(Entry::Datatype(weak_ref)) => {
                        let named_dt = weak_ref.upgrade().expect("weak backref to defined type");
                        match &*named_dt.datatype() {
                            Datatype::Handle { .. } => Ok(DatatypeRef::Name(named_dt)),
                            other => Err(ValidationError::WrongKindName {
                                name: id_syntax.name().to_string(),
                                location: self.location(id_syntax.span()),
                                expected: "handle",
                                got: other.kind(),
                            }),
                        }
                    }
                    Some(entry) => Err(ValidationError::WrongKindName {
                        name: id_syntax.name().to_string(),
                        location: self.location(id_syntax.span()),
                        expected: "handle",
                        got: entry.kind(),
                    }),
                    None => Err(ValidationError::Recursive {
                        name: id_syntax.name().to_string(),
                        location: self.location(id_syntax.span()),
                    }),
                }
            })
            .collect::<Result<Vec<DatatypeRef>, _>>()?;

        Ok(HandleDatatype { supertypes })
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
                            type_: self
                                .doc
                                .validate_datatype(&f.item.type_, f.item.name.span())?,
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
                        let type_ = self
                            .doc
                            .validate_datatype(&f.item.type_, f.item.name.span())?;
                        if ix == 0 {
                            match type_.datatype().passed_by() {
                                DatatypePassedBy::Value(_) => {}
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
                            type_,
                            position: InterfaceFuncParamPosition::Result(ix),
                            docs: f.comments.docs(),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let rc_func = Rc::new(InterfaceFunc {
                    name: name.clone(),
                    params,
                    results,
                    docs: decl.comments.docs(),
                });
                self.entries
                    .insert(name, ModuleEntry::Func(Rc::downgrade(&rc_func)));
                Ok(ModuleDefinition::Func(rc_func))
            }
        }
    }
}
