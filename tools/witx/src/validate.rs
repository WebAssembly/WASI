use crate::{
    io::{Filesystem, WitxIo},
    parser::{
        CommentSyntax, DeclSyntax, Documented, EnumSyntax, FlagsSyntax, HandleSyntax,
        ImportTypeSyntax, IntSyntax, ModuleDeclSyntax, StructSyntax, TypedefSyntax, UnionSyntax,
        VariantSyntax,
    },
    BuiltinType, Definition, Entry, EnumDatatype, EnumVariant, FlagsDatatype, FlagsMember,
    HandleDatatype, Id, IntConst, IntDatatype, IntRepr, InterfaceFunc, InterfaceFuncParam,
    InterfaceFuncParamPosition, Location, Module, ModuleDefinition, ModuleEntry, ModuleImport,
    ModuleImportVariant, NamedType, StructDatatype, StructMember, Type, TypePassedBy, TypeRef,
    UnionDatatype, UnionVariant,
};
use std::collections::{hash_map, HashMap};
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
    AnonymousStructure { location: Location },
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
            | AnonymousStructure { location, .. }
            | InvalidUnionField { location, .. } => {
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
                let tref = self.validate_datatype(&decl.def, true, decl.ident.span())?;

                let rc_datatype = Rc::new(NamedType {
                    name: name.clone(),
                    tref,
                    docs,
                });
                self.doc
                    .entries
                    .insert(name, Entry::Typename(Rc::downgrade(&rc_datatype)));
                Ok(Definition::Typename(rc_datatype))
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
            | TypedefSyntax::Int { .. }
            | TypedefSyntax::Flags { .. }
            | TypedefSyntax::Struct { .. }
            | TypedefSyntax::Union { .. }
            | TypedefSyntax::Handle { .. }
                if !named =>
            {
                Err(ValidationError::AnonymousStructure {
                    location: self.location(span),
                })
            }
            other => Ok(TypeRef::Value(Rc::new(match other {
                TypedefSyntax::Enum(syntax) => Type::Enum(self.validate_enum(&syntax, span)?),
                TypedefSyntax::Int(syntax) => Type::Int(self.validate_int(&syntax, span)?),
                TypedefSyntax::Flags(syntax) => Type::Flags(self.validate_flags(&syntax, span)?),
                TypedefSyntax::Struct(syntax) => Type::Struct(self.validate_struct(&syntax, span)?),
                TypedefSyntax::Union(syntax) => Type::Union(self.validate_union(&syntax, span)?),
                TypedefSyntax::Handle(syntax) => Type::Handle(self.validate_handle(syntax, span)?),
                TypedefSyntax::Array(syntax) => {
                    Type::Array(self.validate_datatype(syntax, false, span)?)
                }
                TypedefSyntax::Pointer(syntax) => {
                    Type::Pointer(self.validate_datatype(syntax, false, span)?)
                }
                TypedefSyntax::ConstPointer(syntax) => {
                    Type::ConstPointer(self.validate_datatype(syntax, false, span)?)
                }
                TypedefSyntax::Builtin(builtin) => Type::Builtin(*builtin),
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

    fn validate_int(
        &self,
        syntax: &IntSyntax,
        span: wast::Span,
    ) -> Result<IntDatatype, ValidationError> {
        let mut int_scope = IdentValidation::new();
        let repr = self.validate_int_repr(&syntax.repr, span)?;
        let consts = syntax
            .consts
            .iter()
            .map(|i| {
                let name =
                    int_scope.introduce(i.item.name.name(), self.location(i.item.name.span()))?;
                let value = i.item.value;
                let docs = i.comments.docs();
                Ok(IntConst { name, value, docs })
            })
            .collect::<Result<Vec<IntConst>, _>>()?;

        Ok(IntDatatype { repr, consts })
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
                let tref = self.validate_datatype(&f.item.type_, false, f.item.name.span())?;
                let docs = f.comments.docs();
                Ok(StructMember { name, tref, docs })
            })
            .collect::<Result<Vec<StructMember>, _>>()?;

        Ok(StructDatatype { members })
    }

    fn validate_union(
        &self,
        syntax: &UnionSyntax,
        span: wast::Span,
    ) -> Result<UnionDatatype, ValidationError> {
        let mut variant_scope = IdentValidation::new();
        let tag_id = self.get(&syntax.tag)?;
        let (tag, mut variant_name_uses) = match self.doc.entries.get(&tag_id) {
            Some(Entry::Typename(weak_ref)) => {
                let named_dt = weak_ref.upgrade().expect("weak backref to defined type");
                match &*named_dt.type_() {
                    Type::Enum(e) => {
                        let uses = e
                            .variants
                            .iter()
                            .map(|v| (v.name.clone(), false))
                            .collect::<HashMap<Id, bool>>();
                        Ok((named_dt, uses))
                    }
                    other => Err(ValidationError::WrongKindName {
                        name: syntax.tag.name().to_string(),
                        location: self.location(syntax.tag.span()),
                        expected: "enum",
                        got: other.kind(),
                    }),
                }
            }
            other => Err(ValidationError::WrongKindName {
                name: syntax.tag.name().to_string(),
                location: self.location(syntax.tag.span()),
                expected: "enum",
                got: match other {
                    Some(e) => e.kind(),
                    None => "unknown",
                },
            }),
        }?;

        let variants = syntax
            .fields
            .iter()
            .map(|v| {
                let variant_name = match v.item {
                    VariantSyntax::Field(ref f) => &f.name,
                    VariantSyntax::Empty(ref name) => name,
                };
                let name = variant_scope
                    .introduce(variant_name.name(), self.location(variant_name.span()))?;
                let tref = match &v.item {
                    VariantSyntax::Field(f) => {
                        Some(self.validate_datatype(&f.type_, false, variant_name.span())?)
                    }
                    VariantSyntax::Empty { .. } => None,
                };
                let docs = v.comments.docs();
                match variant_name_uses.entry(name.clone()) {
                    hash_map::Entry::Occupied(mut e) => {
                        if *e.get() {
                            Err(ValidationError::InvalidUnionField {
                                name: variant_name.name().to_string(),
                                reason: "variant already defined".to_owned(),
                                location: self.location(variant_name.span()),
                            })?;
                        } else {
                            e.insert(true);
                        }
                    }
                    hash_map::Entry::Vacant { .. } => Err(ValidationError::InvalidUnionField {
                        name: variant_name.name().to_string(),
                        reason: format!(
                            "does not correspond to variant in tag `{}`",
                            tag.name.as_str()
                        ),
                        location: self.location(variant_name.span()),
                    })?,
                }
                Ok(UnionVariant { name, tref, docs })
            })
            .collect::<Result<Vec<UnionVariant>, _>>()?;

        let unused_variants = variant_name_uses
            .iter()
            .filter(|(_k, used)| **used == false)
            .map(|(k, _)| k.clone())
            .collect::<Vec<Id>>();
        if !unused_variants.is_empty() {
            Err(ValidationError::InvalidUnionField {
                name: unused_variants
                    .iter()
                    .map(|i| i.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                reason: format!("missing variants from tag `{}`", tag.name.as_str()),
                location: self.location(span),
            })?;
        }
        Ok(UnionDatatype { tag, variants })
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
