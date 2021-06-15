use crate::Abi;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(String);

impl Id {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        Id(s.as_ref().to_string())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl PartialEq<&str> for Id {
    fn eq(&self, rhs: &&str) -> bool {
        PartialEq::eq(self.as_ref(), *rhs)
    }
}

impl PartialEq<Id> for &str {
    fn eq(&self, rhs: &Id) -> bool {
        PartialEq::eq(*self, rhs.as_ref())
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    name: Id,
    module_id: ModuleId,
    types: Vec<Rc<NamedType>>,
    type_map: HashMap<Id, Rc<NamedType>>,

    resources: Vec<Rc<Resource>>,
    resource_map: HashMap<Id, Rc<Resource>>,

    funcs: Vec<Rc<Function>>,
    func_map: HashMap<Id, Rc<Function>>,

    constants: Vec<Constant>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ModuleId(pub(crate) Rc<std::path::PathBuf>);

impl Module {
    pub(crate) fn new(name: Id, module_id: ModuleId) -> Module {
        Module {
            name,
            module_id,
            types: Default::default(),
            type_map: Default::default(),
            resources: Default::default(),
            resource_map: Default::default(),
            funcs: Default::default(),
            func_map: Default::default(),
            constants: Default::default(),
        }
    }

    pub fn name(&self) -> &Id {
        &self.name
    }

    pub fn module_id(&self) -> &ModuleId {
        &self.module_id
    }

    pub(crate) fn push_type(&mut self, ty: Rc<NamedType>) {
        assert!(self.type_map.insert(ty.name.clone(), ty.clone()).is_none());
        self.types.push(ty);
    }

    pub(crate) fn push_resource(&mut self, r: Rc<Resource>) {
        assert!(self
            .resource_map
            .insert(r.name.clone(), r.clone())
            .is_none());
        self.resources.push(r);
    }

    pub(crate) fn push_func(&mut self, func: Rc<Function>) {
        assert!(self
            .func_map
            .insert(func.name.clone(), func.clone())
            .is_none());
        self.funcs.push(func);
    }

    pub(crate) fn push_constant(&mut self, constant: Constant) {
        self.constants.push(constant);
    }

    pub fn typename(&self, name: &Id) -> Option<Rc<NamedType>> {
        self.type_map.get(name).cloned()
    }

    pub fn typenames<'a>(&'a self) -> impl Iterator<Item = &'a Rc<NamedType>> + 'a {
        self.types.iter()
    }

    pub fn resource(&self, name: &Id) -> Option<Rc<Resource>> {
        self.resource_map.get(name).cloned()
    }

    pub fn resources<'a>(&'a self) -> impl Iterator<Item = &'a Rc<Resource>> + 'a {
        self.resources.iter()
    }

    /// All of the (unique) types used as "err" variant of results returned from
    /// functions.
    pub fn error_types<'a>(&'a self) -> impl Iterator<Item = TypeRef> + 'a {
        let errors: HashSet<TypeRef> = self
            .funcs()
            .filter_map(|f| {
                if f.results.len() == 1 {
                    Some(f.results[0].tref.type_().clone())
                } else {
                    None
                }
            })
            .filter_map(|t| match &*t {
                Type::Variant(v) => {
                    let (_ok, err) = v.as_expected()?;
                    Some(err?.clone())
                }
                _ => None,
            })
            .collect::<HashSet<TypeRef>>();
        errors.into_iter()
    }

    pub fn func(&self, name: &Id) -> Option<Rc<Function>> {
        self.func_map.get(&name).cloned()
    }

    pub fn funcs<'a>(&'a self) -> impl Iterator<Item = Rc<Function>> + 'a {
        self.funcs.iter().cloned()
    }

    pub fn constants<'a>(&'a self) -> impl Iterator<Item = &'a Constant> + 'a {
        self.constants.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeRef {
    Name(Rc<NamedType>),
    Value(Rc<Type>),
}

impl TypeRef {
    pub fn type_(&self) -> &Rc<Type> {
        match self {
            TypeRef::Name(named) => named.type_(),
            TypeRef::Value(v) => v,
        }
    }

    pub fn name(&self) -> Option<&NamedType> {
        match self {
            TypeRef::Name(n) => Some(n),
            TypeRef::Value(_) => None,
        }
    }

    pub fn named(&self) -> bool {
        match self {
            TypeRef::Name(_) => true,
            TypeRef::Value(_) => false,
        }
    }

    pub fn type_equal(&self, other: &TypeRef) -> bool {
        self.type_().type_equal(other.type_())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedType {
    pub name: Id,
    pub module: ModuleId,
    pub tref: TypeRef,
    pub docs: String,
}

impl NamedType {
    pub fn type_(&self) -> &Rc<Type> {
        self.tref.type_()
    }
}

/// Structure of all possible interface types.
///
/// Note that this is intended to match the interface types proposal itself.
/// Currently this is relatively close to that with just a few `*.witx`
/// extensions for now.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// A structure with named field.
    Record(RecordDatatype),
    /// An enumeration where a value is one of a number of variants.
    Variant(Variant),
    /// A "handle" which is an un-forgeable reference. Today this is an `i32`
    /// where a module can't forge and use integers it was not already given
    /// access to.
    Handle(HandleDatatype),
    /// A list of a type, stored in linear memory.
    ///
    /// Note that lists of `char` are specialized to indicate strings.
    List(TypeRef),
    /// A `witx`-specific type representing a raw mutable pointer into linear
    /// memory
    Pointer(TypeRef),
    /// A `witx`-specific type representing a raw const pointer into linear
    /// memory
    ConstPointer(TypeRef),
    /// A buffer type representing a window in memory
    Buffer(Buffer),
    /// A builtin base-case type.
    Builtin(BuiltinType),
}

impl Type {
    /// Returns a human-readable string to describe this type.
    pub fn kind(&self) -> &'static str {
        use Type::*;
        match self {
            Record(_) => "record",
            Variant(_) => "variant",
            Handle(_) => "handle",
            List(_) => "list",
            Pointer(_) => "pointer",
            ConstPointer(_) => "constpointer",
            Buffer(_) => "buffer",
            Builtin(_) => "builtin",
        }
    }

    /// Returns whether the in-memory representation of this type will always be
    /// valid regardless of the value of all the bits in memory.
    ///
    /// This is only true for numerical types, pointers, and records of these
    /// values. This is used for canonical lifting/lowering of lists.
    pub fn all_bits_valid(&self) -> bool {
        match self {
            Type::Record(r) => r.members.iter().all(|t| t.tref.type_().all_bits_valid()),

            Type::Builtin(BuiltinType::Char)
            | Type::Variant(_)
            | Type::Handle(_)
            | Type::Buffer(_)
            | Type::List(_) => false,

            Type::Builtin(BuiltinType::U8 { .. })
            | Type::Builtin(BuiltinType::S8)
            | Type::Builtin(BuiltinType::U16)
            | Type::Builtin(BuiltinType::S16)
            | Type::Builtin(BuiltinType::U32 { .. })
            | Type::Builtin(BuiltinType::S32)
            | Type::Builtin(BuiltinType::U64)
            | Type::Builtin(BuiltinType::S64)
            | Type::Builtin(BuiltinType::F32)
            | Type::Builtin(BuiltinType::F64)
            | Type::Pointer(_)
            | Type::ConstPointer(_) => true,
        }
    }

    pub fn type_equal(&self, other: &Type) -> bool {
        match self {
            Type::Record(a) => match other {
                Type::Record(b) => a.type_equal(b),
                _ => false,
            },
            Type::Variant(a) => match other {
                Type::Variant(b) => a.type_equal(b),
                _ => false,
            },
            Type::Handle(a) => match other {
                Type::Handle(b) => a.type_equal(b),
                _ => false,
            },
            Type::List(a) => match other {
                Type::List(b) => a.type_equal(b),
                _ => false,
            },
            Type::Pointer(a) => match other {
                Type::Pointer(b) => a.type_equal(b),
                _ => false,
            },
            Type::ConstPointer(a) => match other {
                Type::ConstPointer(b) => a.type_equal(b),
                _ => false,
            },
            Type::Builtin(a) => match other {
                Type::Builtin(b) => a == b,
                _ => false,
            },
            Type::Buffer(a) => match other {
                Type::Buffer(b) => a.type_equal(b),
                _ => false,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    /// This is a 32-bit unicode scalar value, not a code point.
    ///
    /// Same as the Rust language's `char` type.
    Char,
    /// An 8-bit unsigned integer.
    U8 {
        /// Indicates whether this type is intended to represent the `char`
        /// type in the C language. The C `char` type is often unsigned, but
        /// it's language-specific. At an interface-types level this is an
        /// unsigned byte but binding generators may wish to bind this as the
        /// language-specific representation for a C character instead.
        ///
        /// This is also currently used exclusively in conjunction with `@witx
        /// pointer` to hint that it's pointing to unicode string data as well.
        lang_c_char: bool,
    },
    /// A 16-bit unsigned integer.
    U16,
    /// A 32-bit unsigned integer.
    U32 {
        /// Indicates that this 32-bit value should actually be considered a
        /// pointer-like value in language bindings. At the interface types
        /// layer this is always a 32-bit unsigned value, but binding
        /// generators may wish to instead bind this as the equivalent of C's
        /// `size_t` for convenience with other APIs.
        ///
        /// This allows witx authors to communicate the intent that the
        /// argument or return-value is pointer-like.
        lang_ptr_size: bool,
    },
    /// A 64-bit unsigned integer.
    U64,
    /// An 8-bit signed integer
    S8,
    /// A 16-bit signed integer
    S16,
    /// A 32-bit signed integer
    S32,
    /// A 64-bit signed integer
    S64,
    /// A 32-bit floating point value.
    F32,
    /// A 64-bit floating point value.
    F64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IntRepr {
    U8,
    U16,
    U32,
    U64,
}

impl IntRepr {
    pub fn to_builtin(&self) -> BuiltinType {
        match self {
            IntRepr::U8 => BuiltinType::U8 { lang_c_char: false },
            IntRepr::U16 => BuiltinType::U16,
            IntRepr::U32 => BuiltinType::U32 {
                lang_ptr_size: false,
            },
            IntRepr::U64 => BuiltinType::U64,
        }
    }
}

/// A struct-like value with named fields.
///
/// Records map to `struct`s in most languages where this is a type with a
/// number of named fields that all have their own particular type. Field order
/// dictates layout in memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordDatatype {
    /// A hint as to what this record might be.
    ///
    /// Note that in the future this will only be a hint, not a control of the
    /// actual representation itself. At this time though the record layout of
    /// bitflags is different from other types.
    pub kind: RecordKind,

    /// A list of named fields for this record.
    pub members: Vec<RecordMember>,
}

/// Different kinds of records used for hinting various language-specific types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecordKind {
    /// A tuple where the name of all fields are consecutive integers starting
    /// at "0".
    Tuple,
    /// A record where all fields are `bool`s. Currently represented as an
    /// integer with bits set or not set.
    Bitflags(IntRepr),
    /// All other structures.
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordMember {
    pub name: Id,
    pub tref: TypeRef,
    pub docs: String,
}

impl RecordDatatype {
    pub fn is_tuple(&self) -> bool {
        match self.kind {
            RecordKind::Tuple => true,
            _ => false,
        }
    }

    pub fn bitflags_repr(&self) -> Option<IntRepr> {
        match self.kind {
            RecordKind::Bitflags(i) => Some(i),
            _ => None,
        }
    }

    pub fn type_equal(&self, other: &RecordDatatype) -> bool {
        // Note that eventually we'll probably want to ignore ABI-style
        // differences where the fields are reordered but have the same types.
        // That's more of a subtyping-style check, however, and would require a
        // bit more infrastructure so we just go for strict equality for now.
        self.members.len() == other.members.len()
            && self
                .members
                .iter()
                .zip(&other.members)
                .all(|(a, b)| a.type_equal(b))
    }
}

impl RecordMember {
    pub fn type_equal(&self, other: &RecordMember) -> bool {
        self.name == other.name && self.tref.type_equal(&other.tref)
    }
}

impl RecordKind {
    pub fn infer(members: &[RecordMember]) -> RecordKind {
        if members.len() == 0 {
            return RecordKind::Other;
        }

        // Structs-of-bools are classified to get represented as bitflags.
        if members.iter().all(|t| is_bool(&t.tref)) {
            match members.len() {
                n if n <= 8 => return RecordKind::Bitflags(IntRepr::U8),
                n if n <= 16 => return RecordKind::Bitflags(IntRepr::U16),
                n if n <= 32 => return RecordKind::Bitflags(IntRepr::U32),
                n if n <= 64 => return RecordKind::Bitflags(IntRepr::U64),
                _ => {}
            }
        }

        // Members with consecutive integer names get represented as tuples.
        if members
            .iter()
            .enumerate()
            .all(|(i, m)| m.name.as_str().parse().ok() == Some(i))
        {
            return RecordKind::Tuple;
        }

        return RecordKind::Other;

        fn is_bool(t: &TypeRef) -> bool {
            match &**t.type_() {
                Type::Variant(v) => v.is_bool(),
                _ => false,
            }
        }
    }
}

/// A type which represents how values can be one of a set of possible cases.
///
/// This type maps to an `enum` in languages like Rust, but doesn't have an
/// equivalent in languages like JS or C. The closest analog in C is a tagged
/// union, but a `Variant` is always consistent whereas a tagged union in C
/// could be mis-tagged or such.
///
/// Variants are used to represent one of a possible set of types. For example
/// an enum-like variant, a result that is either success or failure, or even a
/// simple `bool`. Variants are primarily used heavily with various kinds of
/// shorthands in the `*.witx` format to represent idioms in languages.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    /// The bit representation of the width of this variant's tag when the
    /// variant is stored in memory.
    pub tag_repr: IntRepr,
    /// The possible cases that values of this variant type can take.
    pub cases: Vec<Case>,
}

impl Variant {
    pub fn infer_repr(cases: usize) -> IntRepr {
        match cases {
            n if n < u8::max_value() as usize => IntRepr::U8,
            n if n < u16::max_value() as usize => IntRepr::U16,
            n if n < u32::max_value() as usize => IntRepr::U32,
            n if n < u64::max_value() as usize => IntRepr::U64,
            _ => panic!("too many cases to fit in a repr"),
        }
    }

    /// If this variant looks like an `option` shorthand, return the type
    /// associated with option.
    ///
    /// Only matches variants fo the form:
    ///
    /// ```text
    /// (variant
    ///     (case "none")
    ///     (case "some" ty))
    /// ```
    pub fn as_option(&self) -> Option<&TypeRef> {
        if self.cases.len() != 2 {
            return None;
        }
        if self.cases[0].name != "none" || self.cases[0].tref.is_some() {
            return None;
        }
        if self.cases[1].name != "some" {
            return None;
        }
        self.cases[1].tref.as_ref()
    }

    /// If this variant looks like an `expected` shorthand, return the ok/err
    /// types associated with this result.
    ///
    /// Only matches variants fo the form:
    ///
    /// ```text
    /// (variant
    ///     (case "ok" ok?)
    ///     (case "err" err?))
    /// ```
    pub fn as_expected(&self) -> Option<(Option<&TypeRef>, Option<&TypeRef>)> {
        if self.cases.len() != 2 {
            return None;
        }
        if self.cases[0].name != "ok" {
            return None;
        }
        if self.cases[1].name != "err" {
            return None;
        }
        Some((self.cases[0].tref.as_ref(), self.cases[1].tref.as_ref()))
    }

    /// Returns whether this variant type is "bool-like" meaning that it matches
    /// this type:
    ///
    /// ```text
    /// (variant
    ///     (case "false")
    ///     (case "true"))
    /// ```
    pub fn is_bool(&self) -> bool {
        self.cases.len() == 2
            && self.cases[0].name == "false"
            && self.cases[1].name == "true"
            && self.cases[0].tref.is_none()
            && self.cases[1].tref.is_none()
    }

    /// Returns whether this variant type is "enum-like" meaning that all of its
    /// cases have no payload associated with them.
    pub fn is_enum(&self) -> bool {
        self.cases.iter().all(|c| c.tref.is_none())
    }

    pub fn type_equal(&self, other: &Variant) -> bool {
        // See the comment in `RecordDatatype::type_equal` for why strict
        // positional equality is required here
        self.tag_repr == other.tag_repr
            && self.cases.len() == other.cases.len()
            && self
                .cases
                .iter()
                .zip(&other.cases)
                .all(|(a, b)| a.type_equal(b))
    }
}

/// One of a number of possible types that a `Variant` can take.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Case {
    /// The name of this case and how to identify it.
    pub name: Id,
    /// An optional payload type for this case and data that can be associated
    /// with it.
    pub tref: Option<TypeRef>,
    /// Documentation for this case.
    pub docs: String,
}

impl Case {
    pub fn type_equal(&self, other: &Case) -> bool {
        self.name == other.name
            && match (&self.tref, &other.tref) {
                (Some(a), Some(b)) => a.type_equal(b),
                (None, None) => true,
                _ => false,
            }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    /// The local name within the module this resource is defined within. This
    /// may differ from the id of the resource itself.
    pub name: Id,
    /// The unique id assigned to this resource.
    pub resource_id: ResourceId,
    /// Documentation in the defining module, if any.
    pub docs: String,
}

/// A unique id used to determine whether two handles are nominally referring
/// to the same resource.
///
/// An id is composed of the definition location (a module id) and the original
/// name within that module.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceId {
    pub name: Id,
    pub module_id: ModuleId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HandleDatatype {
    /// The resource that this handle references, used for determining if two
    /// handle types are nominally equal to one another.
    pub resource_id: ResourceId,
}

impl HandleDatatype {
    pub fn type_equal(&self, other: &HandleDatatype) -> bool {
        self.resource_id == other.resource_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    pub abi: Abi,
    pub name: Id,
    pub params: Vec<Param>,
    pub results: Vec<Param>,
    pub noreturn: bool,
    pub docs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub name: Id,
    pub tref: TypeRef,
    pub docs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constant {
    pub ty: Id,
    pub name: Id,
    pub value: u64,
    pub docs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Buffer {
    /// Whether or not this is an `out` buffer (`true`) or an `in` buffer
    /// (`false`)
    pub out: bool,

    /// The type of items this buffer contains
    pub tref: TypeRef,
}

impl Buffer {
    pub fn type_equal(&self, other: &Buffer) -> bool {
        self.out == other.out && self.tref.type_equal(&other.tref)
    }
}
