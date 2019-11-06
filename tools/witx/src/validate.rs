use crate::{
    io::{Filesystem, WitxIo},
    parser::{
        DatatypeIdentSyntax, DeclSyntax, EnumSyntax, FlagsSyntax, ImportTypeSyntax,
        ModuleDeclSyntax, StructSyntax, TypedefSyntax, UnionSyntax,
    },
    AliasDatatype, BuiltinType, Datatype, DatatypeIdent, DatatypePassedBy, DatatypeVariant,
    Definition, Entry, EnumDatatype, FlagsDatatype, Id, IntRepr, InterfaceFunc, InterfaceFuncParam,
    InterfaceFuncParamPosition, Location, Module, ModuleDefinition, ModuleEntry, ModuleImport,
    ModuleImportVariant, StructDatatype, StructMember, UnionDatatype, UnionVariant,
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
            column: column,
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

    pub fn validate_decl(&mut self, decl: &DeclSyntax) -> Result<Definition, ValidationError> {
        match decl {
            DeclSyntax::Typename(decl) => {
                let name = self.introduce(&decl.ident)?;
                let variant =
                    match &decl.def {
                        TypedefSyntax::Ident(syntax) => DatatypeVariant::Alias(AliasDatatype {
                            name: name.clone(),
                            to: self.validate_datatype_ident(&syntax)?,
                        }),
                        TypedefSyntax::Enum(syntax) => DatatypeVariant::Enum(self.validate_enum(
                            &name,
                            &syntax,
                            decl.ident.span(),
                        )?),
                        TypedefSyntax::Flags(syntax) => DatatypeVariant::Flags(
                            self.validate_flags(&name, &syntax, decl.ident.span())?,
                        ),
                        TypedefSyntax::Struct(syntax) => DatatypeVariant::Struct(
                            self.validate_struct(&name, &syntax, decl.ident.span())?,
                        ),
                        TypedefSyntax::Union(syntax) => DatatypeVariant::Union(
                            self.validate_union(&name, &syntax, decl.ident.span())?,
                        ),
                    };
                let rc_datatype = Rc::new(Datatype {
                    name: name.clone(),
                    variant,
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
                ));
                self.doc
                    .entries
                    .insert(name, Entry::Module(Rc::downgrade(&rc_module)));
                Ok(Definition::Module(rc_module))
            }
        }
    }

    fn validate_datatype_ident(
        &self,
        syntax: &DatatypeIdentSyntax,
    ) -> Result<DatatypeIdent, ValidationError> {
        match syntax {
            DatatypeIdentSyntax::Builtin(b) => Ok(DatatypeIdent::Builtin(*b)),
            DatatypeIdentSyntax::Array(a) => Ok(DatatypeIdent::Array(Box::new(
                self.validate_datatype_ident(&a)?,
            ))),
            DatatypeIdentSyntax::Pointer(a) => Ok(DatatypeIdent::Pointer(Box::new(
                self.validate_datatype_ident(&a)?,
            ))),
            DatatypeIdentSyntax::ConstPointer(a) => Ok(DatatypeIdent::ConstPointer(Box::new(
                self.validate_datatype_ident(&a)?,
            ))),
            DatatypeIdentSyntax::Ident(i) => {
                let id = self.get(i)?;
                match self.doc.entries.get(&id) {
                    Some(Entry::Datatype(weak_d)) => Ok(DatatypeIdent::Ident(
                        weak_d.upgrade().expect("weak backref to defined type"),
                    )),
                    Some(e) => Err(ValidationError::WrongKindName {
                        name: i.name().to_string(),
                        location: self.location(i.span()),
                        expected: "datatype",
                        got: e.kind(),
                    }),
                    None => Err(ValidationError::Recursive {
                        name: i.name().to_string(),
                        location: self.location(i.span()),
                    }),
                }
            }
        }
    }

    fn validate_enum(
        &self,
        name: &Id,
        syntax: &EnumSyntax,
        span: wast::Span,
    ) -> Result<EnumDatatype, ValidationError> {
        let mut enum_scope = IdentValidation::new();
        let repr = self.validate_int_repr(&syntax.repr, span)?;
        let variants = syntax
            .members
            .iter()
            .map(|i| enum_scope.introduce(i.name(), self.location(i.span())))
            .collect::<Result<Vec<Id>, _>>()?;

        Ok(EnumDatatype {
            name: name.clone(),
            repr,
            variants,
        })
    }

    fn validate_flags(
        &self,
        name: &Id,
        syntax: &FlagsSyntax,
        span: wast::Span,
    ) -> Result<FlagsDatatype, ValidationError> {
        let mut flags_scope = IdentValidation::new();
        let repr = self.validate_int_repr(&syntax.repr, span)?;
        let flags = syntax
            .flags
            .iter()
            .map(|i| flags_scope.introduce(i.name(), self.location(i.span())))
            .collect::<Result<Vec<Id>, _>>()?;

        Ok(FlagsDatatype {
            name: name.clone(),
            repr,
            flags,
        })
    }

    fn validate_struct(
        &self,
        name: &Id,
        syntax: &StructSyntax,
        _span: wast::Span,
    ) -> Result<StructDatatype, ValidationError> {
        let mut member_scope = IdentValidation::new();
        let members = syntax
            .fields
            .iter()
            .map(|f| {
                Ok(StructMember {
                    name: member_scope.introduce(f.name.name(), self.location(f.name.span()))?,
                    type_: self.validate_datatype_ident(&f.type_)?,
                })
            })
            .collect::<Result<Vec<StructMember>, _>>()?;

        Ok(StructDatatype {
            name: name.clone(),
            members,
        })
    }

    fn validate_union(
        &self,
        name: &Id,
        syntax: &UnionSyntax,
        _span: wast::Span,
    ) -> Result<UnionDatatype, ValidationError> {
        let mut variant_scope = IdentValidation::new();
        let variants = syntax
            .fields
            .iter()
            .map(|f| {
                Ok(UnionVariant {
                    name: variant_scope.introduce(f.name.name(), self.location(f.name.span()))?,
                    type_: self.validate_datatype_ident(&f.type_)?,
                })
            })
            .collect::<Result<Vec<UnionVariant>, _>>()?;

        Ok(UnionDatatype {
            name: name.clone(),
            variants,
        })
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
        decl: &ModuleDeclSyntax,
    ) -> Result<ModuleDefinition, ValidationError> {
        match decl {
            ModuleDeclSyntax::Import(syntax) => {
                let loc = self.doc.location(syntax.name_loc);
                let name = self.scope.introduce(syntax.name, loc)?;
                let variant = match syntax.type_ {
                    ImportTypeSyntax::Memory => ModuleImportVariant::Memory,
                };
                let rc_import = Rc::new(ModuleImport {
                    name: name.clone(),
                    variant,
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
                            name: argnames
                                .introduce(f.name.name(), self.doc.location(f.name.span()))?,
                            type_: self.doc.validate_datatype_ident(&f.type_)?,
                            position: InterfaceFuncParamPosition::Param(ix),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let results = syntax
                    .results
                    .iter()
                    .enumerate()
                    .map(|(ix, f)| {
                        let type_ = self.doc.validate_datatype_ident(&f.type_)?;
                        if ix == 0 {
                            match type_.passed_by() {
                                DatatypePassedBy::Value(_) => {}
                                _ => Err(ValidationError::InvalidFirstResultType {
                                    location: self.doc.location(f.name.span()),
                                })?,
                            }
                        }
                        Ok(InterfaceFuncParam {
                            name: argnames
                                .introduce(f.name.name(), self.doc.location(f.name.span()))?,
                            type_,
                            position: InterfaceFuncParamPosition::Result(ix),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let rc_func = Rc::new(InterfaceFunc {
                    name: name.clone(),
                    params,
                    results,
                });
                self.entries
                    .insert(name, ModuleEntry::Func(Rc::downgrade(&rc_func)));
                Ok(ModuleDefinition::Func(rc_func))
            }
        }
    }
}
