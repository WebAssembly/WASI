use crate::{
    parser::{
        DatatypeIdentSyntax, DeclSyntax, EnumSyntax, FlagsSyntax, IdentSyntax, ImportTypeSyntax,
        ModuleDeclSyntax, StructSyntax, TypedefSyntax, UnionSyntax,
    },
    AliasDatatype, BuiltinType, Datatype, DatatypeIdent, DatatypeVariant, Definition, Document,
    Entry, EnumDatatype, FlagsDatatype, Id, IntRepr, InterfaceFunc, InterfaceFuncParam, Location,
    Module, ModuleDefinition, ModuleEntry, ModuleImport, ModuleImportVariant, StructDatatype,
    StructMember, UnionDatatype, UnionVariant,
};
use failure::Fail;
use std::collections::HashMap;
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
}

pub fn validate_document(decls: &[DeclSyntax]) -> Result<Document, ValidationError> {
    let mut validator = DocValidation::new();
    let mut definitions = Vec::new();
    for d in decls {
        definitions.push(validator.validate_decl(&d)?);
    }

    Ok(Document {
        entries: validator.entries,
        definitions,
    })
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
    fn introduce(&mut self, syntax: &IdentSyntax) -> Result<Id, ValidationError> {
        if let Some(introduced) = self.names.get(&syntax.name) {
            Err(ValidationError::NameAlreadyExists {
                name: syntax.name.clone(),
                at_location: syntax.location.clone(),
                previous_location: introduced.clone(),
            })
        } else {
            self.names
                .insert(syntax.name.clone(), syntax.location.clone());
            Ok(Id::new(&syntax.name))
        }
    }

    fn get(&self, syntax: &IdentSyntax) -> Result<Id, ValidationError> {
        if self.names.get(&syntax.name).is_some() {
            Ok(Id::new(&syntax.name))
        } else {
            Err(ValidationError::UnknownName {
                name: syntax.name.clone(),
                location: syntax.location.clone(),
            })
        }
    }
}

struct DocValidation {
    scope: IdentValidation,
    pub entries: HashMap<Id, Entry>,
}

impl DocValidation {
    fn new() -> Self {
        Self {
            scope: IdentValidation::new(),
            entries: HashMap::new(),
        }
    }

    fn validate_decl(&mut self, decl: &DeclSyntax) -> Result<Definition, ValidationError> {
        match decl {
            DeclSyntax::Typename(decl) => {
                let name = self.scope.introduce(&decl.ident)?;
                let variant =
                    match &decl.def {
                        TypedefSyntax::Ident(syntax) => DatatypeVariant::Alias(AliasDatatype {
                            name: name.clone(),
                            to: self.validate_datatype_ident(&syntax)?,
                        }),
                        TypedefSyntax::Enum(syntax) => DatatypeVariant::Enum(self.validate_enum(
                            &name,
                            &syntax,
                            &decl.ident.location,
                        )?),
                        TypedefSyntax::Flags(syntax) => DatatypeVariant::Flags(
                            self.validate_flags(&name, &syntax, &decl.ident.location)?,
                        ),
                        TypedefSyntax::Struct(syntax) => DatatypeVariant::Struct(
                            self.validate_struct(&name, &syntax, &decl.ident.location)?,
                        ),
                        TypedefSyntax::Union(syntax) => DatatypeVariant::Union(
                            self.validate_union(&name, &syntax, &decl.ident.location)?,
                        ),
                    };
                let rc_datatype = Rc::new(Datatype {
                    name: name.clone(),
                    variant,
                });
                self.entries
                    .insert(name, Entry::Datatype(Rc::downgrade(&rc_datatype)));
                Ok(Definition::Datatype(rc_datatype))
            }
            DeclSyntax::Module(syntax) => {
                let name = self.scope.introduce(&syntax.name)?;
                let mut module_validator = ModuleValidation::new(self);
                let definitions = syntax
                    .decls
                    .iter()
                    .map(|d| module_validator.validate_decl(&d))
                    .collect::<Result<Vec<_>, _>>()?;

                let rc_module = Rc::new(Module {
                    name: name.clone(),
                    definitions,
                    entries: module_validator.entries,
                });
                self.entries
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
            DatatypeIdentSyntax::Ident(i) => {
                let id = self.scope.get(i)?;
                match self.entries.get(&id) {
                    Some(Entry::Datatype(weak_d)) => Ok(DatatypeIdent::Ident(
                        weak_d.upgrade().expect("weak backref to defined type"),
                    )),
                    Some(e) => Err(ValidationError::WrongKindName {
                        name: i.name.clone(),
                        location: i.location.clone(),
                        expected: "datatype",
                        got: e.kind(),
                    }),
                    None => Err(ValidationError::Recursive {
                        name: i.name.clone(),
                        location: i.location.clone(),
                    }),
                }
            }
        }
    }

    fn validate_enum(
        &self,
        name: &Id,
        syntax: &EnumSyntax,
        location: &Location,
    ) -> Result<EnumDatatype, ValidationError> {
        let mut enum_scope = IdentValidation::new();
        let repr = validate_int_repr(&syntax.repr, location)?;
        let variants = syntax
            .members
            .iter()
            .map(|i| enum_scope.introduce(i))
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
        location: &Location,
    ) -> Result<FlagsDatatype, ValidationError> {
        let mut flags_scope = IdentValidation::new();
        let repr = validate_int_repr(&syntax.repr, location)?;
        let flags = syntax
            .flags
            .iter()
            .map(|i| flags_scope.introduce(i))
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
        _location: &Location,
    ) -> Result<StructDatatype, ValidationError> {
        let mut member_scope = IdentValidation::new();
        let members = syntax
            .fields
            .iter()
            .map(|f| {
                Ok(StructMember {
                    name: member_scope.introduce(&f.name)?,
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
        _location: &Location,
    ) -> Result<UnionDatatype, ValidationError> {
        let mut variant_scope = IdentValidation::new();
        let variants = syntax
            .fields
            .iter()
            .map(|f| {
                Ok(UnionVariant {
                    name: variant_scope.introduce(&f.name)?,
                    type_: self.validate_datatype_ident(&f.type_)?,
                })
            })
            .collect::<Result<Vec<UnionVariant>, _>>()?;

        Ok(UnionDatatype {
            name: name.clone(),
            variants,
        })
    }
}

fn validate_int_repr(type_: &BuiltinType, location: &Location) -> Result<IntRepr, ValidationError> {
    match type_ {
        BuiltinType::U8 => Ok(IntRepr::I8),
        BuiltinType::U16 => Ok(IntRepr::I16),
        BuiltinType::U32 => Ok(IntRepr::I32),
        BuiltinType::U64 => Ok(IntRepr::I64),
        _ => Err(ValidationError::InvalidRepr {
            repr: type_.clone(),
            location: location.clone(),
        }),
    }
}

struct ModuleValidation<'a> {
    doc: &'a DocValidation,
    scope: IdentValidation,
    pub entries: HashMap<Id, ModuleEntry>,
}

impl<'a> ModuleValidation<'a> {
    fn new(doc: &'a DocValidation) -> Self {
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
                let name = self.scope.introduce(&syntax.name)?;
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
                let name = self.scope.introduce(&syntax.export)?;
                let mut argnames = IdentValidation::new();
                let params = syntax
                    .params
                    .iter()
                    .map(|f| {
                        Ok(InterfaceFuncParam {
                            name: argnames.introduce(&f.name)?,
                            type_: self.doc.validate_datatype_ident(&f.type_)?,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let results = syntax
                    .results
                    .iter()
                    .map(|f| {
                        Ok(InterfaceFuncParam {
                            name: argnames.introduce(&f.name)?,
                            type_: self.doc.validate_datatype_ident(&f.type_)?,
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
