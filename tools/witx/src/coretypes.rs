use crate::{BuiltinType, Datatype, DatatypeVariant, IntRepr, InterfaceFunc, InterfaceFuncParam};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Enumerates the types permitted for function arguments in the WebAssembly spec
pub enum AtomType {
    I32,
    I64,
    F32,
    F64,
}

impl From<IntRepr> for AtomType {
    fn from(i: IntRepr) -> AtomType {
        match i {
            IntRepr::U8 | IntRepr::U16 | IntRepr::U32 => AtomType::I32,
            IntRepr::U64 => AtomType::I64,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Enumerates the strategies which may be used to pass a datatype as an argument
pub enum DatatypePassedBy {
    /// Pass by value specifies the AtomType used to represent that value
    Value(AtomType),
    /// Pass by a pointer into linear memory
    Pointer,
    /// Pass by a pointer and length pair, into linear memory
    PointerLengthPair,
}

impl Datatype {
    /// Determine the simplest strategy by which a type may be passed. Value always preferred over
    /// Pointer.
    pub fn passed_by(&self) -> DatatypePassedBy {
        match &self.variant {
            DatatypeVariant::Builtin(b) => match b {
                BuiltinType::String => DatatypePassedBy::PointerLengthPair,
                BuiltinType::U8
                | BuiltinType::U16
                | BuiltinType::U32
                | BuiltinType::S8
                | BuiltinType::S16
                | BuiltinType::S32 => DatatypePassedBy::Value(AtomType::I32),
                BuiltinType::U64 | BuiltinType::S64 => DatatypePassedBy::Value(AtomType::I64),
                BuiltinType::F32 => DatatypePassedBy::Value(AtomType::F32),
                BuiltinType::F64 => DatatypePassedBy::Value(AtomType::F64),
            },
            DatatypeVariant::Array { .. } => DatatypePassedBy::PointerLengthPair,
            DatatypeVariant::Pointer { .. } | DatatypeVariant::ConstPointer { .. } => {
                DatatypePassedBy::Value(AtomType::I32)
            }
            DatatypeVariant::Alias(a) => a.to.passed_by(),
            DatatypeVariant::Enum(e) => DatatypePassedBy::Value(e.repr.into()),
            DatatypeVariant::Flags(f) => DatatypePassedBy::Value(f.repr.into()),
            DatatypeVariant::Struct { .. } | DatatypeVariant::Union { .. } => {
                DatatypePassedBy::Pointer
            }
            DatatypeVariant::Handle { .. } => DatatypePassedBy::Value(AtomType::I32),
        }
    }
}

/// A parameter in the WebAssembly type of a function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreParamType {
    /// The interface function parameter to which this
    pub param: InterfaceFuncParam,
    /// The relationship of the WebAssembly parameter to the function interface parameter
    pub signifies: CoreParamSignifies,
}

impl CoreParamType {
    /// Representation of the WebAssembly parameter. This is the type that will appear
    /// in the function's WebAssembly type signature.
    pub fn repr(&self) -> AtomType {
        self.signifies.repr()
    }
}

/// Enumerates the sort of relationship an WebAssembly parameter to an interface function
/// parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreParamSignifies {
    /// Core type represents the value using an AtomType
    Value(AtomType),
    /// Core type represents a pointer into linear memory
    PointerTo,
    /// Core type represents a length of a region of linear memory
    LengthOf,
}

impl CoreParamSignifies {
    /// Representation of the WebAssembly parameter.
    pub fn repr(&self) -> AtomType {
        match self {
            CoreParamSignifies::Value(a) => *a,
            CoreParamSignifies::PointerTo | CoreParamSignifies::LengthOf => AtomType::I32,
        }
    }
}

impl InterfaceFuncParam {
    /// Gives the WebAssembly type that corresponds to passing this interface func parameter by value.
    /// Not all types can be passed by value: those which cannot return None
    pub fn pass_by_value(&self) -> Option<CoreParamType> {
        match self.type_.passed_by() {
            DatatypePassedBy::Value(atom) => Some(CoreParamType {
                signifies: CoreParamSignifies::Value(atom),
                param: self.clone(),
            }),
            DatatypePassedBy::Pointer | DatatypePassedBy::PointerLengthPair => None,
        }
    }

    /// Gives the WebAssembly types that correspond to passing this interface func parameter
    /// by reference. Some types are passed by reference using a single pointer, others
    /// require both a pointer and length.
    pub fn pass_by_reference(&self) -> Vec<CoreParamType> {
        match self.type_.passed_by() {
            DatatypePassedBy::Value(_) | DatatypePassedBy::Pointer => vec![CoreParamType {
                signifies: CoreParamSignifies::PointerTo,
                param: self.clone(),
            }],
            DatatypePassedBy::PointerLengthPair => vec![
                CoreParamType {
                    signifies: CoreParamSignifies::PointerTo,
                    param: self.clone(),
                },
                CoreParamType {
                    signifies: CoreParamSignifies::LengthOf,
                    param: self.clone(),
                },
            ],
        }
    }
}

/// Describes the WebAssembly signature of a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreFuncType {
    pub args: Vec<CoreParamType>,
    pub ret: Option<CoreParamType>,
}

impl InterfaceFunc {
    /// Get the WebAssembly type signature for this interface function
    pub fn core_type(&self) -> CoreFuncType {
        let mut results = self.results.iter();
        // The ret value is the first result (if there is one), passed
        // by value.
        let ret = results.next().map(|param| {
            param
                .pass_by_value()
                .expect("validation ensures first result can be passed by value")
        });
        let args = self
            .params
            .iter()
            .flat_map(|param| {
                // interface function parameters are passed by value if possible,
                // and fall back on passing by reference.
                param
                    .pass_by_value()
                    .map(|ptype| vec![ptype])
                    .unwrap_or_else(|| param.pass_by_reference())
            })
            // Then, the remaining results are passed by reference.
            .chain(results.flat_map(|param| param.pass_by_reference()))
            .collect();
        CoreFuncType { args, ret }
    }
}
