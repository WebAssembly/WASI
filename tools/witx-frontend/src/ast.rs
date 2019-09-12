#![allow(dead_code)]
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub use crate::parser::BuiltinType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

impl Id {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        Id(s.as_ref().to_string())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub definitions: Vec<Definition>,
    pub entries: HashMap<Id, Entry>,
}

#[derive(Debug, Clone)]
pub enum Definition {
    Datatype(Rc<Datatype>),
    Module(Rc<Module>),
}

#[derive(Debug, Clone)]
pub enum Entry {
    Datatype(Weak<Datatype>),
    Module(Weak<Module>),
}

impl Entry {
    pub fn kind(&self) -> &'static str {
        match self {
            Entry::Datatype { .. } => "datatype",
            Entry::Module { .. } => "module",
        }
    }
}

#[derive(Debug, Clone)]
pub enum DatatypeIdent {
    Builtin(BuiltinType),
    Array(Box<DatatypeIdent>),
    Ident(Rc<Datatype>),
}

#[derive(Debug, Clone)]
pub struct Datatype {
    pub name: Id,
    pub variant: DatatypeVariant,
}

#[derive(Debug, Clone)]
pub enum DatatypeVariant {
    Alias(AliasDatatype),
    Enum(EnumDatatype),
    Flags(FlagsDatatype),
    Struct(StructDatatype),
    Union(UnionDatatype),
}

#[derive(Debug, Clone)]
pub struct AliasDatatype {
    pub name: Id,
    pub to: DatatypeIdent,
}

#[derive(Debug, Clone)]
pub enum IntRepr {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Clone)]
pub struct EnumDatatype {
    pub name: Id,
    pub repr: IntRepr,
    pub variants: Vec<Id>,
}

#[derive(Debug, Clone)]
pub struct FlagsDatatype {
    pub name: Id,
    pub repr: IntRepr,
    pub flags: Vec<Id>,
}

#[derive(Debug, Clone)]
pub struct StructDatatype {
    pub name: Id,
    pub members: Vec<StructMember>,
}

#[derive(Debug, Clone)]
pub struct StructMember {
    pub name: Id,
    pub type_: DatatypeIdent,
}

#[derive(Debug, Clone)]
pub struct UnionDatatype {
    pub name: Id,
    pub variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone)]
pub struct UnionVariant {
    pub name: Id,
    pub type_: DatatypeIdent,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Id,
    pub definitions: Vec<ModuleDefinition>,
    pub entries: HashMap<Id, ModuleEntry>,
}

#[derive(Debug, Clone)]
pub enum ModuleDefinition {
    Import(Rc<ModuleImport>),
    Func(Rc<InterfaceFunc>),
}

#[derive(Debug, Clone)]
pub enum ModuleEntry {
    Import(Weak<ModuleImport>),
    Func(Weak<InterfaceFunc>),
}

#[derive(Debug, Clone)]
pub struct ModuleImport {
    pub name: Id,
    pub variant: ModuleImportVariant,
}

#[derive(Debug, Clone)]
pub enum ModuleImportVariant {
    Memory,
}

#[derive(Debug, Clone)]
pub struct InterfaceFunc {
    pub name: Id,
    pub params: Vec<InterfaceFuncParam>,
    pub results: Vec<InterfaceFuncParam>,
}

#[derive(Debug, Clone)]
pub struct InterfaceFuncParam {
    pub name: Id,
    pub type_: DatatypeIdent,
}
