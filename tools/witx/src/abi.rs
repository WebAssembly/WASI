//! Definition of the ABI of witx functions
//!
//! This module is intended to assist with code generators which are binding or
//! implementing APIs defined by `*.witx` files. THis module contains all
//! details necessary to implement the actual ABI of these functions so wasm
//! modules and hosts can communicate with one another.
//!
//! Each interface types function (a function defined in `*.witx`) currently has
//! a well-known wasm signature associated with it. There's then also a standard
//! way to convert from interface-types values (whose representation is defined
//! per-language) into this wasm API. This module is intended to assist with
//! this definition.
//!
//! Contained within are two primary functions, [`InterfaceFunc::call_wasm`] and
//! [`InterfaceFunc::call_interface`]. These functions implement the two ways to
//! interact with an interface types function, namely calling the raw wasm
//! version and calling the high-level version with interface types. These two
//! functions are fed a structure that implements [`Bindgen`]. An instance of
//! [`Bindgen`] receives instructions which are low-level implementation details
//! of how to convert to and from wasm types and interface types. Code
//! generators will need to implement the various instructions to support APIs.

use crate::{
    BuiltinType, Id, IntRepr, InterfaceFunc, InterfaceFuncParam, NamedType, RecordDatatype,
    RecordKind, Type, TypeRef, Variant,
};
use std::mem;

/// A raw WebAssembly signature with params and results.
#[derive(Debug)]
pub struct WasmSignature {
    /// The WebAssembly parameters of this function.
    pub params: Vec<WasmType>,
    /// The WebAssembly results of this function.
    pub results: Vec<WasmType>,
    /// The raw types, if needed, returned through return pointer located in
    /// `params`.
    pub retptr: Option<Vec<WasmType>>,
}

/// Enumerates wasm types used by interface types when lowering/lifting.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    // NOTE: we don't lower interface types to any other Wasm type,
    // e.g. externref, so we don't need to define them here.
}

impl From<IntRepr> for WasmType {
    fn from(i: IntRepr) -> WasmType {
        match i {
            IntRepr::U8 | IntRepr::U16 | IntRepr::U32 => WasmType::I32,
            IntRepr::U64 => WasmType::I64,
        }
    }
}

/// Possible ABIs for interface functions to have.
///
/// Note that this is a stopgap until we have more of interface types. Interface
/// types functions do not have ABIs, they have APIs. For the meantime, however,
/// we mandate ABIs to ensure we can all talk to each other.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Abi {
    /// Only stable ABI currently, and is the historical WASI ABI since it was
    /// first created.
    ///
    /// Note that this ABI is limited notably in its return values where it can
    /// only return 0 results or one `Result<T, enum>` lookalike.
    Preview1,

    /// TODO
    Next,
}

// Helper macro for defining instructions without having to have tons of
// exhaustive `match` statements to update
macro_rules! def_instruction {
    (
        $( #[$enum_attr:meta] )*
        pub enum $name:ident<'a> {
            $(
                $( #[$attr:meta] )*
                $variant:ident $( {
                    $($field:ident : $field_ty:ty $(,)* )*
                } )?
                    :
                [$num_popped:expr] => [$num_pushed:expr],
            )*
        }
    ) => {
        $( #[$enum_attr] )*
        pub enum $name<'a> {
            $(
                $( #[$attr] )*
                $variant $( {
                    $(
                        $field : $field_ty,
                    )*
                } )? ,
            )*
        }

        impl $name<'_> {
            /// How many operands does this instruction pop from the stack?
            #[allow(unused_variables)]
            pub fn operands_len(&self) -> usize {
                match self {
                    $(
                        Self::$variant $( {
                            $(
                                $field,
                            )*
                        } )? => $num_popped,
                    )*
                }
            }

            /// How many results does this instruction push onto the stack?
            #[allow(unused_variables)]
            pub fn results_len(&self) -> usize {
                match self {
                    $(
                        Self::$variant $( {
                            $(
                                $field,
                            )*
                        } )? => $num_pushed,
                    )*
                }
            }
        }
    };
}

def_instruction! {
    #[derive(Debug)]
    pub enum Instruction<'a> {
        /// Acquires the specified parameter and places it on the stack.
        /// Depending on the context this may refer to wasm parameters or
        /// interface types parameters.
        GetArg { nth: usize } : [0] => [1],
        /// Pushes the constant `val` onto the stack.
        I32Const { val: i32 } : [0] => [1],
        /// Converts an interface type `char` value to a 32-bit integer
        /// representing the unicode scalar value.
        I32FromChar : [1] => [1],
        /// Converts an interface type `u64` value to a wasm `i64`.
        I64FromU64 : [1] => [1],
        /// Converts an interface type `s64` value to a wasm `i64`.
        I64FromS64 : [1] => [1],
        /// Converts an interface type `u32` value to a wasm `i32`.
        I32FromU32 : [1] => [1],
        /// Converts an interface type `s32` value to a wasm `i32`.
        I32FromS32 : [1] => [1],
        /// Converts an interface type `u16` value to a wasm `i32`.
        I32FromU16 : [1] => [1],
        /// Converts an interface type `s16` value to a wasm `i32`.
        I32FromS16 : [1] => [1],
        /// Converts an interface type `u8` value to a wasm `i32`.
        I32FromU8 : [1] => [1],
        /// Converts an interface type `s8` value to a wasm `i32`.
        I32FromS8 : [1] => [1],
        /// Converts a language-specific `usize` value to a wasm `i32`.
        I32FromUsize : [1] => [1],
        /// Converts a language-specific C `char` value to a wasm `i32`.
        I32FromChar8 : [1] => [1],
        /// Converts a language-specific handle value to a wasm `i32`.
        I32FromHandle { ty: &'a NamedType } : [1] => [1],
        /// Lowers a list whose elements can be directly copied and interpreted
        /// with no validation whatsoever.
        ///
        /// Pops a list value from the stack and pushes the pointer/length onto
        /// the stack. Note that this consumes the list and the list must be an
        /// owned allocation.
        ListCanonLower {
            element: &'a TypeRef,
            malloc: String,
        } : [1] => [2],
        /// Lowers a list whose elements are copied one-by-one into a new list.
        ///
        /// Pops a list value from the stack and pushes the pointer/length onto
        /// the stack. Note that this consumes the list as an owned allocation
        /// and must produce an owned allocation.
        ///
        /// This operation also pops a block from the block stack which is used
        /// as the iteration body of writing each element of the list consumed.
        ListLower {
            element: &'a TypeRef,
            malloc: String,
        } : [1] => [2],
        /// Lifts a list which has a canonical representation into an interface
        /// types value.
        ///
        /// This will consume two `i32` values from the stack, a pointer and a
        /// length, and then produces an interface value list. Note that the
        /// pointer/length popped are **owned** and need to be deallocated when
        /// the interface type is dropped.
        ListCanonLift {
            element: &'a TypeRef,
            free: String,
        } : [2] => [1],
        /// Lifts a list which into an interface types value.
        ///
        /// This will consume two `i32` values from the stack, a pointer and a
        /// length, and then produces an interface value list. Note that the
        /// pointer/length popped are **owned** and need to be deallocated when
        /// the interface type is dropped.
        ///
        /// This will also pop a block from the block stack which is how to
        /// read each individual element from the list.
        ListLift {
            element: &'a TypeRef,
            free: String,
        } : [2] => [1],
        /// Pushes an operand onto the stack representing the list item from
        /// each iteration of the list.
        ///
        /// This is only used inside of blocks related to lowering lists.
        IterElem : [0] => [1],
        /// Pushes an operand onto the stack representing the base pointer of
        /// the next element in a list.
        ///
        /// This is sused for both lifting and lowering lists.
        IterBasePointer : [0] => [1],
        /// Conversion an interface type `f32` value to a wasm `f32`.
        ///
        /// This may be a noop for some implementations, but it's here in case the
        /// native language representation of `f32` is different than the wasm
        /// representation of `f32`.
        F32FromIf32 : [1] => [1],
        /// Conversion an interface type `f64` value to a wasm `f64`.
        ///
        /// This may be a noop for some implementations, but it's here in case the
        /// native language representation of `f64` is different than the wasm
        /// representation of `f64`.
        F64FromIf64 : [1] => [1],
        /// Converts a native wasm `i32` to a language-specific C `char`.
        ///
        /// This will truncate the upper bits of the `i32`.
        Char8FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to a language-specific `usize`.
        UsizeFromI32 : [1] => [1],

        /// Represents a call to a raw WebAssembly API. The module/name are
        /// provided inline as well as the types if necessary.
        CallWasm {
            module: &'a str,
            name: &'a str,
            params: &'a [WasmType],
            results: &'a [WasmType],
        } : [params.len()] => [results.len()],

        /// Same as `CallWasm`, except the dual where an interface is being
        /// called rather than a raw wasm function.
        CallInterface {
            module: &'a str,
            func: &'a InterfaceFunc,
        } : [func.params.len()] => [func.results.len()],

        /// Converts a native wasm `i32` to an interface type `s8`.
        ///
        /// This will truncate the upper bits of the `i32`.
        S8FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `u8`.
        ///
        /// This will truncate the upper bits of the `i32`.
        U8FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `s16`.
        ///
        /// This will truncate the upper bits of the `i32`.
        S16FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `u16`.
        ///
        /// This will truncate the upper bits of the `i32`.
        U16FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `s32`.
        S32FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `u32`.
        U32FromI32 : [1] => [1],
        /// Converts a native wasm `i64` to an interface type `s64`.
        S64FromI64 : [1] => [1],
        /// Converts a native wasm `i64` to an interface type `u64`.
        U64FromI64 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `char`.
        ///
        /// It's safe to assume that the `i32` is indeed a valid unicode code point.
        CharFromI32 : [1] => [1],
        /// Converts a native wasm `f32` to an interface type `f32`.
        If32FromF32 : [1] => [1],
        /// Converts a native wasm `f64` to an interface type `f64`.
        If64FromF64 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `handle`.
        HandleFromI32 { ty: &'a NamedType } : [1] => [1],
        /// Returns `amt` values on the stack. This is always the last
        /// instruction.
        Return { amt: usize } : [*amt] => [0],

        /// Pops a record value off the stack, decomposes the record to all of
        /// its fields, and then pushes the fields onto the stack.
        RecordLower {
            ty: &'a RecordDatatype,
            name: Option<&'a NamedType>,
        } : [1] => [ty.members.len()],
        /// Pops all fields for a record off the stack and then composes them
        /// into a record.
        RecordLift {
            ty: &'a RecordDatatype,
            name: Option<&'a NamedType>,
        } : [ty.members.len()] => [1],

        /// This is a special instruction used at the entry of blocks used as
        /// part of `ResultLower`, representing that the payload of that variant
        /// being matched on should be pushed onto the stack.
        VariantPayload : [0] => [1],

        /// Pops a variant off the stack as well as `ty.cases.len()` blocks
        /// from the code generator. Uses each of those blocks and the value
        /// from the stack to produce `nresults` of items.
        VariantLower {
            ty: &'a Variant,
            name: Option<&'a NamedType>,
            nresults: usize,
        } : [1] => [*nresults],

        /// Pops an `i32` off the stack as well as `ty.cases.len()` blocks
        /// from the code generator. Uses each of those blocks and the value
        /// from the stack to produce a final variant.
        VariantLift {
            ty: &'a Variant,
            name: Option<&'a NamedType>,
        } : [1] => [1],

        /// Casts the top N items on the stack using the `Bitcast` enum
        /// provided. Consumes the same number of operands that this produces.
        Bitcasts { casts: &'a [Bitcast] } : [casts.len()] => [casts.len()],

        /// Pushes a number of constant zeros for each wasm type on the stack.
        ConstZero { tys: &'a [WasmType] } : [0] => [tys.len()],

        /// Pops an `i32` from the stack and loads a little-endian `i32` from
        /// it, using the specified constant offset.
        I32Load { offset: i32 } : [1] => [1],
        /// Pops an `i32` from the stack and loads a little-endian `i8` from
        /// it, using the specified constant offset. The value loaded is the
        /// zero-extended to 32-bits
        I32Load8U { offset: i32 } : [1] => [1],
        /// Pops an `i32` from the stack and loads a little-endian `i8` from
        /// it, using the specified constant offset. The value loaded is the
        /// sign-extended to 32-bits
        I32Load8S { offset: i32 } : [1] => [1],
        /// Pops an `i32` from the stack and loads a little-endian `i16` from
        /// it, using the specified constant offset. The value loaded is the
        /// zero-extended to 32-bits
        I32Load16U { offset: i32 } : [1] => [1],
        /// Pops an `i32` from the stack and loads a little-endian `i16` from
        /// it, using the specified constant offset. The value loaded is the
        /// sign-extended to 32-bits
        I32Load16S { offset: i32 } : [1] => [1],
        /// Pops an `i32` from the stack and loads a little-endian `i64` from
        /// it, using the specified constant offset.
        I64Load { offset: i32 } : [1] => [1],
        /// Pops an `i32` from the stack and loads a little-endian `f32` from
        /// it, using the specified constant offset.
        F32Load { offset: i32 } : [1] => [1],
        /// Pops an `i32` from the stack and loads a little-endian `f64` from
        /// it, using the specified constant offset.
        F64Load { offset: i32 } : [1] => [1],

        /// Pops an `i32` address from the stack and then an `i32` value.
        /// Stores the value in little-endian at the pointer specified plus the
        /// constant `offset`.
        I32Store { offset: i32 } : [2] => [0],
        /// Pops an `i32` address from the stack and then an `i32` value.
        /// Stores the low 8 bits of the value in little-endian at the pointer
        /// specified plus the constant `offset`.
        I32Store8 { offset: i32 } : [2] => [0],
        /// Pops an `i32` address from the stack and then an `i32` value.
        /// Stores the low 16 bits of the value in little-endian at the pointer
        /// specified plus the constant `offset`.
        I32Store16 { offset: i32 } : [2] => [0],
        /// Pops an `i32` address from the stack and then an `i64` value.
        /// Stores the value in little-endian at the pointer specified plus the
        /// constant `offset`.
        I64Store { offset: i32 } : [2] => [0],
        /// Pops an `i32` address from the stack and then an `f32` value.
        /// Stores the value in little-endian at the pointer specified plus the
        /// constant `offset`.
        F32Store { offset: i32 } : [2] => [0],
        /// Pops an `i32` address from the stack and then an `f64` value.
        /// Stores the value in little-endian at the pointer specified plus the
        /// constant `offset`.
        F64Store { offset: i32 } : [2] => [0],

        /// An instruction from an extended instruction set that's specific to
        /// `*.witx` and the "Preview1" ABI.
        Witx {
            instr: &'a WitxInstruction<'a>,
        } : [instr.operands_len()] => [instr.results_len()],
    }
}

#[derive(Debug, PartialEq)]
pub enum Bitcast {
    // Upcasts
    F32ToF64,
    F32ToI32,
    F64ToI64,
    I32ToI64,
    F32ToI64,

    // Downcasts
    F64ToF32,
    I32ToF32,
    I64ToF64,
    I64ToI32,
    I64ToF32,

    None,
}

def_instruction! {
    #[derive(Debug)]
    pub enum WitxInstruction<'a> {
        /// Takes the value off the top of the stack and writes it into linear
        /// memory. Pushes the address in linear memory as an `i32`.
        AddrOf : [1] => [1],
        /// Converts a language-specific pointer value to a wasm `i32`.
        I32FromPointer : [1] => [1],
        /// Converts a language-specific pointer value to a wasm `i32`.
        I32FromConstPointer : [1] => [1],
        /// Converts a language-specific record-of-bools to the packed
        /// representation as an `i32`.
        I32FromBitflags { ty: &'a NamedType } : [1] => [1],
        /// Converts a language-specific record-of-bools to the packed
        /// representation as an `i64`.
        I64FromBitflags { ty: &'a NamedType } : [1] => [1],
        /// Pops two `i32` values from the stack and creates a list from them of
        /// the specified type. The first operand is the pointer in linear
        /// memory to the start of the list and the second operand is the
        /// length.
        ListFromPointerLength { ty: &'a TypeRef } : [2] => [1],
        /// Pushes the pointer/length of a list as two `i32` parameters.
        ListPointerLength : [1] => [2],

        /// Converts a native wasm `i32` to a language-specific pointer.
        PointerFromI32 { ty: &'a TypeRef }: [1] => [1],
        /// Converts a native wasm `i32` to a language-specific pointer.
        ConstPointerFromI32 { ty: &'a TypeRef } : [1] => [1],
        /// Converts a native wasm `i32` to a language-specific record-of-bools.
        BitflagsFromI32 { ty: &'a NamedType } : [1] => [1],
        /// Converts a native wasm `i64` to a language-specific record-of-bools.
        BitflagsFromI64 { ty: &'a NamedType } : [1] => [1],
        /// Loads the interface types value from an `i32` pointer popped from
        /// the stack.
        Load { ty: &'a NamedType } : [1] => [1],
        /// Stores an interface types value into linear memory. The first
        /// operand is the value to store and the second operand is the pointer
        /// in linear memory to store it at.
        Store { ty: &'a NamedType } : [2] => [0],
        /// Pops a native wasm `i32` from the stack, as well as two blocks
        /// internally from the code generator.
        ///
        /// If the value is 0 then the first "ok" block value should be used.
        /// If the value is anything else then the second "err" block value
        /// should be used, and the value is used as the error enum.
        ///
        /// Note that this is a special instruction matching the current ABI of
        /// WASI and intentionally differs from the type-level grammar of
        /// interface types results.
        ResultLift : [1] => [1],
        /// Pops a native interface value from the stack as well as two blocks
        /// internally from the code generator.
        ///
        /// A `match` is performed on the value popped and the corresponding
        /// block for ok/err is used depending on value. This pushes a single
        /// `i32` onto the stack representing the error code for this result.
        ///
        /// Note that like `ResultLift` this is specialized to the current WASI
        /// ABI.
        ResultLower {
            ok: Option<&'a TypeRef>,
            err: Option<&'a TypeRef>,
        } : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `enum` value.
        ///
        /// It's guaranteed that the interface type integer value is within
        /// range for this enum's type. Additionally `ty` is guaranteed to be
        /// enum-like as a `Variant` where all `case` arms have no associated
        /// type with them. The purpose of this instruction is to convert a
        /// native wasm integer into the enum type for the interface.
        EnumLift { ty: &'a NamedType } : [1] => [1],
        /// Converts an interface types enum value into a wasm `i32`.
        EnumLower { ty: &'a NamedType } : [1] => [1],
        /// Creates a tuple from the top `n` elements on the stack, pushing the
        /// tuple onto the stack.
        TupleLift { amt: usize } : [*amt] => [1],
        /// Splits a tuple at the top of the stack into its `n` components,
        /// pushing them all onto the stack.
        TupleLower { amt: usize } : [1] => [*amt],
        /// This is a special instruction specifically for the original ABI of
        /// WASI.  The raw return `i32` of a function is re-pushed onto the
        /// stack for reuse.
        ReuseReturn : [0] => [1],
    }
}

impl Abi {
    /// Validates the parameters/results are representable in this ABI.
    ///
    /// Returns an error string if they're not representable or returns `Ok` if
    /// they're indeed representable.
    pub fn validate(
        &self,
        params: &[InterfaceFuncParam],
        results: &[InterfaceFuncParam],
    ) -> Result<(), String> {
        match self {
            Abi::Preview1 => {
                // validated below...
            }
            Abi::Next => {
                for ty in params.iter().chain(results) {
                    validate_no_witx(ty.tref.type_())?;
                }
                return Ok(());
            }
        }
        assert_eq!(*self, Abi::Preview1);
        match results.len() {
            0 => {}
            1 => match &**results[0].tref.type_() {
                Type::Handle(_) | Type::Builtin(_) | Type::ConstPointer(_) | Type::Pointer(_) => {}
                Type::Variant(v) => {
                    let (ok, err) = match v.as_expected() {
                        Some(pair) => pair,
                        None => return Err("invalid return type".to_string()),
                    };
                    if let Some(ty) = ok {
                        match &**ty.type_() {
                            Type::Record(r) if r.is_tuple() => {
                                for member in r.members.iter() {
                                    if !member.tref.named() {
                                        return Err(
                                            "only named types are allowed in results".to_string()
                                        );
                                    }
                                }
                            }
                            _ => {
                                if !ty.named() {
                                    return Err(
                                        "only named types are allowed in results".to_string()
                                    );
                                }
                            }
                        }
                    }
                    if let Some(ty) = err {
                        if !ty.named() {
                            return Err("only named types are allowed in results".to_string());
                        }
                        if let Type::Variant(v) = &**ty.type_() {
                            if v.is_enum() {
                                return Ok(());
                            }
                        }
                    }
                }
                Type::Record(r) if r.bitflags_repr().is_some() => {}
                Type::Record(_) | Type::List(_) => return Err("invalid return type".to_string()),
            },
            _ => return Err("more than one result".to_string()),
        }
        Ok(())
    }
}

fn validate_no_witx(ty: &Type) -> Result<(), String> {
    match ty {
        Type::Record(r) => {
            match r.kind {
                RecordKind::Bitflags(_) => {
                    return Err("cannot use `(@witx bitflags)` in this ABI".to_string())
                }
                RecordKind::Tuple | RecordKind::Other => {}
            }
            for r in r.members.iter() {
                validate_no_witx(r.tref.type_())?;
            }
            Ok(())
        }
        Type::Variant(v) => {
            if v.tag_repr != IntRepr::U32 {
                return Err("cannot use `(@witx tag)` in this ABI".to_string());
            }
            for case in v.cases.iter() {
                if let Some(ty) = &case.tref {
                    validate_no_witx(ty.type_())?;
                }
            }
            Ok(())
        }
        Type::Handle(_) => Ok(()),
        Type::List(t) => validate_no_witx(t.type_()),
        Type::Pointer(_) => return Err("cannot use `(@witx pointer)` in this ABI".to_string()),
        Type::ConstPointer(_) => {
            return Err("cannot use `(@witx const_pointer)` in this ABI".to_string())
        }
        Type::Builtin(BuiltinType::U8 { lang_c_char: true }) => {
            return Err("cannot use `(@witx char8)` in this ABI".to_string());
        }
        Type::Builtin(BuiltinType::U32 {
            lang_ptr_size: true,
        }) => {
            return Err("cannot use `(@witx usize)` in this ABI".to_string());
        }
        Type::Builtin(_) => Ok(()),
    }
}

/// Trait for language implementors to use to generate glue code between native
/// WebAssembly signatures and interface types signatures.
///
/// This is used as an implementation detail in interpreting the ABI between
/// interface types and wasm types. Eventually this will be driven by interface
/// types adapters themselves, but for now the ABI of a function dictates what
/// instructions are fed in.
///
/// Types implementing `Bindgen` are incrementally fed `Instruction` values to
/// generate code for. Instructions operate like a stack machine where each
/// instruction has a list of inputs and a list of outputs (provided by the
/// `emit` function).
pub trait Bindgen {
    /// The intermediate type for fragments of code for this type.
    ///
    /// For most languages `String` is a suitable intermediate type.
    type Operand: Clone;

    /// Emit code to implement the given instruction.
    ///
    /// Each operand is given in `operands` and can be popped off if ownership
    /// is required. It's guaranteed that `operands` has the appropriate length
    /// for the `inst` given, as specified with [`Instruction`].
    ///
    /// Each result variable should be pushed onto `results`. This function must
    /// push the appropriate number of results or binding generation will panic.
    fn emit(
        &mut self,
        inst: &Instruction<'_>,
        operands: &mut Vec<Self::Operand>,
        results: &mut Vec<Self::Operand>,
    );

    /// Allocates temporary space in linear memory for the type `ty`.
    ///
    /// This is called when calling some wasm functions where a return pointer
    /// is needed. Only used for the `Abi::Preview1` ABI.
    ///
    /// Returns an `Operand` which has type `i32` and is the base of the typed
    /// allocation in memory.
    fn allocate_typed_space(&mut self, ty: &NamedType) -> Self::Operand;

    /// Allocates temporary space in linear memory for a fixed number of `i64`
    /// values.
    ///
    /// This is only called in the `Abi::Next` ABI for when a function would
    /// otherwise have multiple results.
    ///
    /// Returns an `Operand` which has type `i32` and points to the base of the
    /// fixed-size-array allocation.
    fn allocate_i64_array(&mut self, amt: usize) -> Self::Operand;

    /// Enters a new block of code to generate code for.
    ///
    /// This is currently exclusively used for constructing variants. When a
    /// variant is constructed a block here will be pushed for each case of a
    /// variant, generating the code necessary to translate a variant case.
    ///
    /// Blocks are completed with `finish_block` below. It's expected that `emit`
    /// will always push code (if necessary) into the "current block", which is
    /// updated by calling this method and `finish_block` below.
    fn push_block(&mut self);

    /// Indicates to the code generator that a block is completed, and the
    /// `operand` specified was the resulting value of the block.
    ///
    /// This method will be used to compute the value of each arm of lifting a
    /// variant. The `operand` will be `None` if the variant case didn't
    /// actually have any type associated with it. Otherwise it will be `Some`
    /// as the last value remaining on the stack representing the value
    /// associated with a variant's `case`.
    ///
    /// It's expected that this will resume code generation in the previous
    /// block before `push_block` was called. This must also save the results
    /// of the current block internally for instructions like `ResultLift` to
    /// use later.
    fn finish_block(&mut self, operand: &mut Vec<Self::Operand>);
}

impl InterfaceFunc {
    /// Get the WebAssembly type signature for this interface function
    ///
    /// The first entry returned is the list of parameters and the second entry
    /// is the list of results for the wasm function signature.
    pub fn wasm_signature(&self) -> WasmSignature {
        let mut params = Vec::new();
        let mut results = Vec::new();
        for param in self.params.iter() {
            match &**param.tref.type_() {
                Type::Builtin(_)
                | Type::Pointer(_)
                | Type::ConstPointer(_)
                | Type::Handle(_)
                | Type::List(_) => {
                    push_wasm(param.tref.type_(), &mut params);
                }
                ty @ Type::Variant(_) => match self.abi {
                    Abi::Preview1 => params.push(WasmType::I32),
                    Abi::Next => push_wasm(ty, &mut params),
                },
                Type::Record(r) => match self.abi {
                    Abi::Preview1 => match r.bitflags_repr() {
                        Some(repr) => params.push(WasmType::from(repr)),
                        None => params.push(WasmType::I32),
                    },
                    Abi::Next => push_wasm(param.tref.type_(), &mut params),
                },
            }
        }

        for param in self.results.iter() {
            match &**param.tref.type_() {
                Type::List(_)
                | Type::Builtin(_)
                | Type::Pointer(_)
                | Type::ConstPointer(_)
                | Type::Record(_)
                | Type::Handle(_) => {
                    push_wasm(param.tref.type_(), &mut results);
                }

                Type::Variant(v) => {
                    match self.abi {
                        Abi::Preview1 => {} // handled below
                        Abi::Next => {
                            push_wasm(param.tref.type_(), &mut results);
                            continue;
                        }
                    }
                    results.push(match v.tag_repr {
                        IntRepr::U64 => WasmType::I64,
                        IntRepr::U32 | IntRepr::U16 | IntRepr::U8 => WasmType::I32,
                    });
                    if v.is_enum() {
                        continue;
                    }
                    // return pointer
                    if let Some(ty) = &v.cases[0].tref {
                        match &**ty.type_() {
                            Type::Record(r) if r.is_tuple() => {
                                for _ in 0..r.members.len() {
                                    params.push(WasmType::I32);
                                }
                            }
                            _ => params.push(WasmType::I32),
                        }
                    }
                }
            }
        }

        // Rust/C don't support multi-value well right now, so if a function
        // would have multiple results then instead truncate it to have 0
        // results and instead insert a return pointer.
        let mut retptr = None;
        if results.len() > 1 {
            params.push(WasmType::I32);
            retptr = Some(mem::take(&mut results));
        }

        WasmSignature {
            params,
            results,
            retptr,
        }
    }

    /// Generates an abstract sequence of instructions which represents this
    /// function being adapted as an imported function.
    ///
    /// The instructions here, when executed, will emulate a language with
    /// interface types calling the concrete wasm implementation. The parameters
    /// for the returned instruction sequence are the language's own
    /// interface-types parameters. One instruction in the instruction stream
    /// will be a `Call` which represents calling the actual raw wasm function
    /// signature.
    ///
    /// This function is useful, for example, if you're building a language
    /// generator for WASI bindings. This will document how to translate
    /// language-specific values into the wasm types to call a WASI function,
    /// and it will also automatically convert the results of the WASI function
    /// back to a language-specific value.
    pub fn call_wasm(&self, module: &Id, bindgen: &mut impl Bindgen) {
        Generator::new(self.abi, bindgen).call_wasm(module, self);
    }

    /// This is the dual of [`InterfaceFunc::call_wasm`], except that instead of
    /// calling a wasm signature it generates code to come from a wasm signature
    /// and call an interface types signature.
    pub fn call_interface(&self, module: &Id, bindgen: &mut impl Bindgen) {
        Generator::new(self.abi, bindgen).call_interface(module, self);
    }
}

struct Generator<'a, B: Bindgen> {
    abi: Abi,
    bindgen: &'a mut B,
    operands: Vec<B::Operand>,
    results: Vec<B::Operand>,
    stack: Vec<B::Operand>,
    return_pointers: Vec<B::Operand>,
}

impl<'a, B: Bindgen> Generator<'a, B> {
    fn new(abi: Abi, bindgen: &'a mut B) -> Generator<'a, B> {
        Generator {
            abi,
            bindgen,
            operands: Vec::new(),
            results: Vec::new(),
            stack: Vec::new(),
            return_pointers: Vec::new(),
        }
    }

    fn call_wasm(&mut self, module: &Id, func: &InterfaceFunc) {
        // Push all parameters for this function onto the stack, and then
        // batch-lower everything all at once.
        for nth in 0..func.params.len() {
            self.emit(&Instruction::GetArg { nth });
        }
        self.lower_all(&func.params, None);

        // If necessary we may need to prepare a return pointer for this ABI.
        // The `Preview1` ABI has most return values returned through pointers,
        // and the `Next` ABI returns more-than-one values through a return
        // pointer.
        let sig = func.wasm_signature();
        self.prep_return_pointer(&sig, &func.results);

        // Now that all the wasm args are prepared we can call the actual wasm
        // function.
        assert_eq!(self.stack.len(), sig.params.len());
        self.emit(&Instruction::CallWasm {
            module: module.as_str(),
            name: func.name.as_str(),
            params: &sig.params,
            results: &sig.results,
        });

        // In the `Next` ABI we model multiple return values by going through
        // memory. Remove that indirection here by loading everything to
        // simulate the function having many return values in our stack
        // discipline.
        if let Some(actual) = &sig.retptr {
            self.load_retptr(actual);
        }

        // Batch-lift all result values now that all the function's return
        // values are on the stack.
        self.lift_all(&func.results, true);

        self.emit(&Instruction::Return {
            amt: func.results.len(),
        });
        assert!(self.stack.is_empty());
    }

    fn load_retptr(&mut self, types: &[WasmType]) {
        assert_eq!(self.return_pointers.len(), 1);
        for (i, ty) in types.iter().enumerate() {
            self.stack.push(self.return_pointers[0].clone());
            let offset = (i * 8) as i32;
            match ty {
                WasmType::I32 => self.emit(&Instruction::I32Load { offset }),
                WasmType::I64 => self.emit(&Instruction::I64Load { offset }),
                WasmType::F32 => self.emit(&Instruction::F32Load { offset }),
                WasmType::F64 => self.emit(&Instruction::F64Load { offset }),
            }
        }
    }

    fn call_interface(&mut self, module: &Id, func: &InterfaceFunc) {
        // Use `GetArg` to push all relevant arguments onto the stack. Note
        // that we can't use the signature of this function directly due to
        // various conversions and return pointers, so we need to somewhat
        // manually calculate all the arguments which are converted as
        // interface types arguments below.
        let sig = func.wasm_signature();
        let nargs = match self.abi {
            Abi::Preview1 => {
                func.params.len()
                    + func
                        .params
                        .iter()
                        .filter(|t| match &**t.tref.type_() {
                            Type::List(_) => true,
                            _ => false,
                        })
                        .count()
            }
            Abi::Next => sig.params.len() - sig.retptr.is_some() as usize,
        };
        for nth in 0..nargs {
            self.emit(&Instruction::GetArg { nth });
        }

        // Once everything is on the stack we can lift all arguments one-by-one
        // into their interface-types equivalent.
        self.lift_all(&func.params, false);

        // ... and that allows us to call the interface types function ...
        self.emit(&Instruction::CallInterface {
            module: module.as_str(),
            func,
        });

        // ... and at the end we lower everything back into return values.
        self.lower_all(&func.results, Some(nargs));

        // Our ABI dictates that a list of returned types are returned through
        // memories, so after we've got all the values on the stack perform
        // all of the stores here.
        if let Some(tys) = &sig.retptr {
            self.store_retptr(tys, sig.params.len() - 1);
        }

        self.emit(&Instruction::Return {
            amt: sig.results.len(),
        });
        assert!(self.stack.is_empty());
    }

    /// Assumes that the wasm values to create `tys` are all located on the
    /// stack.
    ///
    /// Inserts instructions necesesary to lift those types into their
    /// interface types equivalent.
    fn lift_all(&mut self, tys: &[InterfaceFuncParam], is_return: bool) {
        let mut temp = Vec::new();
        let operands = tys
            .iter()
            .rev()
            .map(|ty| {
                let ntys = match self.abi {
                    Abi::Preview1 => match &**ty.tref.type_() {
                        Type::List(_) => 2,
                        _ => 1,
                    },
                    Abi::Next => {
                        temp.truncate(0);
                        push_wasm(ty.tref.type_(), &mut temp);
                        temp.len()
                    }
                };
                self.stack
                    .drain(self.stack.len() - ntys..)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for (operands, ty) in operands.into_iter().rev().zip(tys) {
            self.stack.extend(operands);
            self.lift(&ty.tref, is_return);
        }
    }

    /// Assumes that the value for `tys` is already on the stack, and then
    /// converts all of those values into their wasm types by lowering each
    /// argument in-order.
    fn lower_all(&mut self, tys: &[InterfaceFuncParam], mut nargs: Option<usize>) {
        let operands = self
            .stack
            .drain(self.stack.len() - tys.len()..)
            .collect::<Vec<_>>();
        for (operand, ty) in operands.into_iter().zip(tys) {
            self.stack.push(operand);
            self.lower(&ty.tref, nargs.as_mut());
        }
    }

    /// Assumes `types.len()` values are on the stack and stores them all into
    /// the return pointer of this function, specified in the last argument.
    ///
    /// This is only used with `Abi::Next`.
    fn store_retptr(&mut self, types: &[WasmType], retptr_arg: usize) {
        self.emit(&Instruction::GetArg { nth: retptr_arg });
        let retptr = self.stack.pop().unwrap();
        for (i, ty) in types.iter().enumerate().rev() {
            self.stack.push(retptr.clone());
            let offset = (i * 8) as i32;
            match ty {
                WasmType::I32 => self.emit(&Instruction::I32Store { offset }),
                WasmType::I64 => self.emit(&Instruction::I64Store { offset }),
                WasmType::F32 => self.emit(&Instruction::F32Store { offset }),
                WasmType::F64 => self.emit(&Instruction::F64Store { offset }),
            }
        }
    }

    fn witx(&mut self, instr: &WitxInstruction<'_>) {
        self.emit(&Instruction::Witx { instr });
    }

    fn emit(&mut self, inst: &Instruction<'_>) {
        self.operands.clear();
        self.results.clear();

        let operands_len = inst.operands_len();
        assert!(
            self.stack.len() >= operands_len,
            "not enough operands on stack for {:?}",
            inst
        );
        self.operands
            .extend(self.stack.drain((self.stack.len() - operands_len)..));
        self.results.reserve(inst.results_len());

        self.bindgen
            .emit(inst, &mut self.operands, &mut self.results);

        assert_eq!(
            self.results.len(),
            inst.results_len(),
            "{:?} expected {} results, got {}",
            inst,
            inst.results_len(),
            self.results.len()
        );
        self.stack.extend(self.results.drain(..));
    }

    fn finish_block(&mut self, size: usize) {
        self.operands.clear();
        assert!(
            size <= self.stack.len(),
            "not enough operands on stack for finishing block",
        );
        self.operands
            .extend(self.stack.drain((self.stack.len() - size)..));
        self.bindgen.finish_block(&mut self.operands);
    }

    fn lower(&mut self, ty: &TypeRef, retptr: Option<&mut usize>) {
        use Instruction::*;
        use WitxInstruction::*;

        match &**ty.type_() {
            Type::Builtin(BuiltinType::S8) => self.emit(&I32FromS8),
            Type::Builtin(BuiltinType::U8 { lang_c_char: true }) => self.emit(&I32FromChar8),
            Type::Builtin(BuiltinType::U8 { lang_c_char: false }) => self.emit(&I32FromU8),
            Type::Builtin(BuiltinType::S16) => self.emit(&I32FromS16),
            Type::Builtin(BuiltinType::U16) => self.emit(&I32FromU16),
            Type::Builtin(BuiltinType::S32) => self.emit(&I32FromS32),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: true,
            }) => self.emit(&I32FromUsize),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: false,
            }) => self.emit(&I32FromU32),
            Type::Builtin(BuiltinType::S64) => self.emit(&I64FromS64),
            Type::Builtin(BuiltinType::U64) => self.emit(&I64FromU64),
            Type::Builtin(BuiltinType::Char) => self.emit(&I32FromChar),
            Type::Builtin(BuiltinType::F32) => self.emit(&F32FromIf32),
            Type::Builtin(BuiltinType::F64) => self.emit(&F64FromIf64),
            Type::Pointer(_) => self.witx(&I32FromPointer),
            Type::ConstPointer(_) => self.witx(&I32FromConstPointer),
            Type::Handle(_) => self.emit(&I32FromHandle {
                ty: ty.name().unwrap(),
            }),
            Type::List(element) => match self.abi {
                Abi::Preview1 => self.witx(&ListPointerLength),
                Abi::Next => {
                    let malloc = String::from("witx_malloc");
                    if type_all_bits_valid(element) || is_char(element) {
                        self.emit(&ListCanonLower { element, malloc });
                    } else {
                        self.bindgen.push_block();
                        self.emit(&IterElem);
                        self.emit(&IterBasePointer);
                        let addr = self.stack.pop().unwrap();
                        self.write_to_memory(element, addr, 0);
                        self.finish_block(0);
                        self.emit(&ListLower { element, malloc });
                    }
                }
            },
            Type::Record(r) => {
                if let Some(repr) = r.bitflags_repr() {
                    let ty = ty.name().unwrap();
                    match repr {
                        IntRepr::U64 => return self.witx(&I64FromBitflags { ty }),
                        _ => return self.witx(&I32FromBitflags { ty }),
                    }
                }
                match self.abi {
                    Abi::Preview1 => self.witx(&AddrOf),
                    Abi::Next => {
                        self.emit(&RecordLower {
                            ty: r,
                            name: ty.name(),
                        });
                        let fields = self
                            .stack
                            .drain(self.stack.len() - r.members.len()..)
                            .collect::<Vec<_>>();
                        for (member, field) in r.members.iter().zip(fields) {
                            self.stack.push(field);
                            self.lower(&member.tref, None);
                        }
                    }
                }
            }

            Type::Variant(v) if self.abi == Abi::Preview1 => {
                // Enum-like variants are simply lowered to their discriminant.
                if v.is_enum() {
                    return self.witx(&EnumLower {
                        ty: ty.name().unwrap(),
                    });
                }

                // If this variant is in the return position then it's special,
                // otherwise it's an argument and we just pass the address.
                let retptr = match retptr {
                    Some(ptr) => ptr,
                    None => return self.witx(&AddrOf),
                };

                // For the return position we emit some blocks to lower the
                // ok/err payloads which means that in the ok branch we're
                // storing to out-params and in the err branch we're simply
                // lowering the error enum.
                //
                // Note that this is all very specific to the current WASI ABI.
                let (ok, err) = v.as_expected().unwrap();
                self.bindgen.push_block();
                if let Some(ok) = ok {
                    self.emit(&VariantPayload);
                    let store = |me: &mut Self, ty: &TypeRef, n| {
                        me.emit(&GetArg { nth: *retptr + n });
                        me.witx(&Store {
                            ty: ty.name().unwrap(),
                        });
                    };
                    match &**ok.type_() {
                        Type::Record(r) if r.is_tuple() => {
                            self.witx(&TupleLower {
                                amt: r.members.len(),
                            });
                            // Note that `rev()` is used here due to the order
                            // that tuples are pushed onto the stack and how we
                            // consume the last item first from the stack.
                            for (i, member) in r.members.iter().enumerate().rev() {
                                store(self, &member.tref, i);
                            }
                        }
                        _ => store(self, ok, 0),
                    }
                };
                self.finish_block(0);

                self.bindgen.push_block();
                if let Some(ty) = err {
                    self.emit(&VariantPayload);
                    self.lower(ty, None);
                }
                self.finish_block(err.is_some() as usize);

                self.witx(&ResultLower { ok, err });
            }

            Type::Variant(v) => {
                let mut results = Vec::new();
                let mut temp = Vec::new();
                let mut casts = Vec::new();
                push_wasm(ty.type_(), &mut results);
                for (i, case) in v.cases.iter().enumerate() {
                    self.bindgen.push_block();
                    self.emit(&I32Const { val: i as i32 });
                    let mut pushed = 1;
                    if let Some(ty) = &case.tref {
                        // Using the payload of this block we lower the type to
                        // raw wasm values.
                        self.emit(&VariantPayload);
                        self.lower(ty, None);

                        // Determine the types of all the wasm values we just
                        // pushed, and record how many. If we pushed too few
                        // then we'll need to push some zeros after this.
                        temp.truncate(0);
                        push_wasm(ty.type_(), &mut temp);
                        pushed += temp.len();

                        // For all the types pushed we may need to insert some
                        // bitcasts. This will go through and cast everything
                        // to the right type to ensure all blocks produce the
                        // same set of results.
                        casts.truncate(0);
                        for (actual, expected) in temp.iter().zip(&results[1..]) {
                            casts.push(cast(*actual, *expected));
                        }
                        if casts.iter().any(|c| *c != Bitcast::None) {
                            self.emit(&Bitcasts { casts: &casts });
                        }
                    }

                    // If we haven't pushed enough items in this block to match
                    // what other variants are pushing then we need to push
                    // some zeros.
                    if pushed < results.len() {
                        self.emit(&ConstZero {
                            tys: &results[pushed..],
                        });
                    }
                    self.finish_block(results.len());
                }
                self.emit(&VariantLower {
                    ty: v,
                    nresults: results.len(),
                    name: ty.name(),
                });
            }
        }
    }

    fn prep_return_pointer(&mut self, sig: &WasmSignature, results: &[InterfaceFuncParam]) {
        match self.abi {
            Abi::Preview1 => {
                assert!(results.len() < 2);
                let ty = match results.get(0) {
                    Some(ty) => ty.tref.type_(),
                    None => return,
                };
                // Return pointers are only needed for `Result<T, _>`...
                let variant = match &**ty {
                    Type::Variant(v) => v,
                    _ => return,
                };
                // ... and only if `T` is actually present in `Result<T, _>`
                let ok = match &variant.cases[0].tref {
                    Some(t) => t,
                    None => return,
                };

                // Tuples have each individual item in a separate return pointer while
                // all other types go through a singular return pointer.
                let mut prep = |ty: &TypeRef| {
                    let ptr = self.bindgen.allocate_typed_space(ty.name().unwrap());
                    self.return_pointers.push(ptr.clone());
                    self.stack.push(ptr);
                };
                match &**ok.type_() {
                    Type::Record(r) if r.is_tuple() => {
                        for member in r.members.iter() {
                            prep(&member.tref);
                        }
                    }
                    _ => prep(ok),
                }
            }
            // If a return pointer was automatically injected into this function
            // then we need to allocate a proper amount of space for it and then
            // add it to the stack to get passed to the callee.
            Abi::Next => {
                if let Some(results) = &sig.retptr {
                    let ptr = self.bindgen.allocate_i64_array(results.len());
                    self.return_pointers.push(ptr.clone());
                    self.stack.push(ptr.clone());
                }
            }
        }
    }

    // Note that in general everything in this function is the opposite of the
    // `lower` function above. This is intentional and should be kept this way!
    fn lift(&mut self, ty: &TypeRef, is_return: bool) {
        use Instruction::*;
        use WitxInstruction::*;

        match &**ty.type_() {
            Type::Builtin(BuiltinType::S8) => self.emit(&S8FromI32),
            Type::Builtin(BuiltinType::U8 { lang_c_char: true }) => self.emit(&Char8FromI32),
            Type::Builtin(BuiltinType::U8 { lang_c_char: false }) => self.emit(&U8FromI32),
            Type::Builtin(BuiltinType::S16) => self.emit(&S16FromI32),
            Type::Builtin(BuiltinType::U16) => self.emit(&U16FromI32),
            Type::Builtin(BuiltinType::S32) => self.emit(&S32FromI32),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: true,
            }) => self.emit(&UsizeFromI32),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: false,
            }) => self.emit(&U32FromI32),
            Type::Builtin(BuiltinType::S64) => self.emit(&S64FromI64),
            Type::Builtin(BuiltinType::U64) => self.emit(&U64FromI64),
            Type::Builtin(BuiltinType::Char) => self.emit(&CharFromI32),
            Type::Builtin(BuiltinType::F32) => self.emit(&If32FromF32),
            Type::Builtin(BuiltinType::F64) => self.emit(&If64FromF64),
            Type::Pointer(ty) => self.witx(&PointerFromI32 { ty }),
            Type::ConstPointer(ty) => self.witx(&ConstPointerFromI32 { ty }),
            Type::Handle(_) => self.emit(&HandleFromI32 {
                ty: ty.name().unwrap(),
            }),
            Type::List(element) => match self.abi {
                Abi::Preview1 => self.witx(&ListFromPointerLength { ty: element }),
                Abi::Next => {
                    let free = String::from("witx_free");
                    if type_all_bits_valid(element) || is_char(element) {
                        self.emit(&ListCanonLift { element, free });
                    } else {
                        self.bindgen.push_block();
                        self.emit(&IterBasePointer);
                        let addr = self.stack.pop().unwrap();
                        self.read_from_memory(element, addr, 0);
                        self.finish_block(1);
                        self.emit(&ListLift { element, free });
                    }
                }
            },
            Type::Record(r) => {
                if let Some(repr) = r.bitflags_repr() {
                    let ty = ty.name().unwrap();
                    match repr {
                        IntRepr::U64 => return self.witx(&BitflagsFromI64 { ty }),
                        _ => return self.witx(&BitflagsFromI32 { ty }),
                    }
                }
                match self.abi {
                    Abi::Preview1 => {
                        let ty = ty.name().unwrap();
                        self.witx(&Load { ty })
                    }
                    Abi::Next => {
                        let mut temp = Vec::new();
                        push_wasm(ty.type_(), &mut temp);
                        let mut args = self
                            .stack
                            .drain(self.stack.len() - temp.len()..)
                            .collect::<Vec<_>>();
                        for member in r.members.iter() {
                            temp.truncate(0);
                            push_wasm(member.tref.type_(), &mut temp);
                            self.stack.extend(args.drain(..temp.len()));
                            self.lift(&member.tref, false);
                        }
                        self.emit(&RecordLift {
                            ty: r,
                            name: ty.name(),
                        });
                    }
                }
            }

            Type::Variant(v) if self.abi == Abi::Preview1 => {
                if v.is_enum() {
                    return self.witx(&EnumLift {
                        ty: ty.name().unwrap(),
                    });
                } else if !is_return {
                    return self.witx(&Load {
                        ty: ty.name().unwrap(),
                    });
                }

                let (ok, err) = v.as_expected().unwrap();
                self.bindgen.push_block();
                if let Some(ok) = ok {
                    let mut n = 0;
                    let mut load = |ty: &TypeRef| {
                        self.stack.push(self.return_pointers[n].clone());
                        n += 1;
                        self.witx(&Load {
                            ty: ty.name().unwrap(),
                        });
                    };
                    match &**ok.type_() {
                        Type::Record(r) if r.is_tuple() => {
                            for member in r.members.iter() {
                                load(&member.tref);
                            }
                            self.witx(&TupleLift {
                                amt: r.members.len(),
                            });
                        }
                        _ => load(ok),
                    }
                }
                self.finish_block(ok.is_some() as usize);

                self.bindgen.push_block();
                if let Some(ty) = err {
                    self.witx(&ReuseReturn);
                    self.lift(ty, false);
                }
                self.finish_block(err.is_some() as usize);

                self.witx(&ResultLift);
            }

            Type::Variant(v) => {
                let mut params = Vec::new();
                let mut temp = Vec::new();
                let mut casts = Vec::new();
                push_wasm(ty.type_(), &mut params);
                let block_inputs = self
                    .stack
                    .drain(self.stack.len() - params.len() + 1..)
                    .collect::<Vec<_>>();
                for case in v.cases.iter() {
                    self.bindgen.push_block();
                    if let Some(ty) = &case.tref {
                        // Push only the values we need for this variant onto
                        // the stack.
                        temp.truncate(0);
                        push_wasm(ty.type_(), &mut temp);
                        self.stack
                            .extend(block_inputs[..temp.len()].iter().cloned());

                        // Cast all the types we have on the stack to the actual
                        // types needed for this variant, if necessary.
                        casts.truncate(0);
                        for (actual, expected) in temp.iter().zip(&params[1..]) {
                            casts.push(cast(*expected, *actual));
                        }
                        if casts.iter().any(|c| *c != Bitcast::None) {
                            self.emit(&Bitcasts { casts: &casts });
                        }

                        // Then recursively lift this variant's payload.
                        self.lift(ty, false);
                    }
                    self.finish_block(case.tref.is_some() as usize);
                }
                self.emit(&VariantLift {
                    ty: v,
                    name: ty.name(),
                });
            }
        }
    }

    fn write_to_memory(&mut self, ty: &TypeRef, addr: B::Operand, offset: i32) {
        use Instruction::*;

        match &**ty.type_() {
            // Builtin types need different flavors of storage instructions
            // depending on the size of the value written.
            Type::Builtin(b) => {
                self.lower(ty, None);
                self.stack.push(addr);
                match b {
                    BuiltinType::S8 | BuiltinType::U8 { .. } => self.emit(&I32Store8 { offset }),
                    BuiltinType::S16 | BuiltinType::U16 => self.emit(&I32Store16 { offset }),
                    BuiltinType::S32 | BuiltinType::U32 { .. } | BuiltinType::Char => {
                        self.emit(&I32Store { offset })
                    }
                    BuiltinType::S64 | BuiltinType::U64 => self.emit(&I64Store { offset }),
                    BuiltinType::F32 => self.emit(&F32Store { offset }),
                    BuiltinType::F64 => self.emit(&F64Store { offset }),
                }
            }

            // Lowering all these types produces an `i32` which we can easily
            // store into memory.
            Type::Pointer(_) | Type::ConstPointer(_) | Type::Handle(_) => {
                self.lower(ty, None);
                self.stack.push(addr);
                self.emit(&I32Store { offset })
            }

            // After lowering the list there's two i32 values on the stack
            // which we write into memory, writing the pointer into the low address
            // and the length into the high address.
            Type::List(_) => {
                self.lower(ty, None);
                self.stack.push(addr.clone());
                self.emit(&I32Store { offset: offset + 4 });
                self.stack.push(addr);
                self.emit(&I32Store { offset });
            }

            // Decompose the record into its components and then write all the
            // components into memory one-by-one.
            Type::Record(r) => {
                self.emit(&RecordLower {
                    ty: r,
                    name: ty.name(),
                });
                let fields = self
                    .stack
                    .drain(self.stack.len() - r.members.len()..)
                    .collect::<Vec<_>>();
                for (layout, field) in r.member_layout().iter().zip(fields) {
                    self.stack.push(field);
                    self.write_to_memory(
                        &layout.member.tref,
                        addr.clone(),
                        offset + (layout.offset as i32),
                    );
                }
            }

            // Each case will get its own block, and the first item in each
            // case is writing the discriminant. After that if we have a
            // payload we write the payload after the discriminant, aligned up
            // to the type's alignment.
            Type::Variant(v) => {
                let payload_offset = offset + (v.payload_offset() as i32);
                for (i, case) in v.cases.iter().enumerate() {
                    self.bindgen.push_block();
                    self.emit(&I32Const { val: i as i32 });
                    self.stack.push(addr.clone());
                    self.emit(&I32Store { offset });
                    if let Some(ty) = &case.tref {
                        self.emit(&VariantPayload);
                        self.write_to_memory(ty, addr.clone(), payload_offset);
                    }
                    self.finish_block(0);
                }
                self.emit(&VariantLower {
                    ty: v,
                    nresults: 0,
                    name: ty.name(),
                });
            }
        }
    }

    fn read_from_memory(&mut self, ty: &TypeRef, addr: B::Operand, offset: i32) {
        use Instruction::*;

        match &**ty.type_() {
            // Builtin types need different flavors of load instructions
            // depending on the size of the value written, but then they're all
            // lifted the same way.
            Type::Builtin(b) => {
                self.stack.push(addr);
                match b {
                    BuiltinType::S8 => self.emit(&I32Load8S { offset }),
                    BuiltinType::U8 { .. } => self.emit(&I32Load8U { offset }),
                    BuiltinType::S16 => self.emit(&I32Load16S { offset }),
                    BuiltinType::U16 => self.emit(&I32Load16U { offset }),
                    BuiltinType::S32 | BuiltinType::U32 { .. } | BuiltinType::Char => {
                        self.emit(&I32Load { offset })
                    }
                    BuiltinType::S64 | BuiltinType::U64 => self.emit(&I64Load { offset }),
                    BuiltinType::F32 => self.emit(&F32Load { offset }),
                    BuiltinType::F64 => self.emit(&F64Load { offset }),
                }
                self.lift(ty, false);
            }

            // These types are all easily lifted from an `i32`
            Type::Pointer(_) | Type::ConstPointer(_) | Type::Handle(_) => {
                self.stack.push(addr);
                self.emit(&I32Load { offset });
                self.lift(ty, false);
            }

            // Read the pointer/len and then perform the standard lifting
            // proceses.
            Type::List(_) => {
                self.stack.push(addr.clone());
                self.emit(&I32Load { offset });
                self.stack.push(addr);
                self.emit(&I32Load { offset: offset + 4 });
                self.lift(ty, false);
            }

            // Read and lift each field individually, adjusting the offset
            // as we go along, then aggregate all the fields into the record.
            Type::Record(r) => {
                for layout in r.member_layout() {
                    self.read_from_memory(
                        &layout.member.tref,
                        addr.clone(),
                        offset + (layout.offset as i32),
                    );
                }
                self.emit(&RecordLift {
                    ty: r,
                    name: ty.name(),
                });
            }

            // Each case will get its own block, and we'll dispatch to the
            // right block based on the `i32.load` we initially perform. Each
            // individual block is pretty simple and just reads the payload type
            // from the corresponding offset if one is available.
            Type::Variant(v) => {
                self.stack.push(addr.clone());
                self.emit(&I32Load { offset });
                let payload_offset = offset + (v.payload_offset() as i32);
                for case in v.cases.iter() {
                    self.bindgen.push_block();
                    if let Some(ty) = &case.tref {
                        self.read_from_memory(ty, addr.clone(), payload_offset);
                    }
                    self.finish_block(case.tref.is_some() as usize);
                }
                self.emit(&VariantLift {
                    ty: v,
                    name: ty.name(),
                });
            }
        }
    }
}

fn push_wasm(ty: &Type, result: &mut Vec<WasmType>) {
    match ty {
        Type::Builtin(BuiltinType::S8)
        | Type::Builtin(BuiltinType::U8 { .. })
        | Type::Builtin(BuiltinType::S16)
        | Type::Builtin(BuiltinType::U16)
        | Type::Builtin(BuiltinType::S32)
        | Type::Builtin(BuiltinType::U32 { .. })
        | Type::Builtin(BuiltinType::Char)
        | Type::Pointer(_)
        | Type::ConstPointer(_)
        | Type::Handle(_) => result.push(WasmType::I32),

        Type::Builtin(BuiltinType::U64) | Type::Builtin(BuiltinType::S64) => {
            result.push(WasmType::I64)
        }
        Type::Builtin(BuiltinType::F32) => result.push(WasmType::F32),
        Type::Builtin(BuiltinType::F64) => result.push(WasmType::F64),

        Type::Record(r) => match r.bitflags_repr() {
            Some(repr) => result.push(repr.into()),
            None => {
                for member in r.members.iter() {
                    push_wasm(member.tref.type_(), result);
                }
            }
        },

        Type::List(_) => {
            result.push(WasmType::I32);
            result.push(WasmType::I32);
        }

        Type::Variant(v) => {
            result.push(v.tag_repr.into());
            let start = result.len();
            let mut temp = Vec::new();

            // Push each case's type onto a temporary vector, and then merge
            // that vector into our final list starting at `start`. Note
            // that this requires some degree of "unification" so we can
            // handle things like `Result<i32, f32>` where that turns into
            // `[i32 i32]` where the second `i32` might be the `f32`
            // bitcasted.
            for case in v.cases.iter() {
                let ty = match &case.tref {
                    Some(ty) => ty,
                    None => continue,
                };
                push_wasm(ty.type_(), &mut temp);

                for (i, ty) in temp.drain(..).enumerate() {
                    match result.get_mut(start + i) {
                        Some(prev) => *prev = unify(*prev, ty),
                        None => result.push(ty),
                    }
                }
            }
        }
    }
}

fn unify(a: WasmType, b: WasmType) -> WasmType {
    use WasmType::*;

    match (a, b) {
        (I64, _) | (_, I64) | (I32, F64) | (F64, I32) => I64,

        (I32, I32) | (I32, F32) | (F32, I32) => I32,

        (F32, F32) => F32,
        (F64, F64) | (F32, F64) | (F64, F32) => F64,
    }
}

fn cast(from: WasmType, to: WasmType) -> Bitcast {
    use WasmType::*;

    match (from, to) {
        (I32, I32) | (I64, I64) | (F32, F32) | (F64, F64) => Bitcast::None,

        (I32, I64) => Bitcast::I32ToI64,
        (F32, F64) => Bitcast::F32ToF64,
        (F32, I32) => Bitcast::F32ToI32,
        (F64, I64) => Bitcast::F64ToI64,

        (I64, I32) => Bitcast::I64ToI32,
        (F64, F32) => Bitcast::F64ToF32,
        (I32, F32) => Bitcast::I32ToF32,
        (I64, F64) => Bitcast::I64ToF64,

        (F32, I64) => Bitcast::F32ToI64,
        (I64, F32) => Bitcast::I64ToF32,
        (F64, I32) | (I32, F64) => unreachable!(),
    }
}

fn type_all_bits_valid(ty: &TypeRef) -> bool {
    match &**ty.type_() {
        Type::Record(r) => r.members.iter().all(|t| type_all_bits_valid(&t.tref)),

        Type::Builtin(BuiltinType::Char) | Type::Variant(_) | Type::Handle(_) | Type::List(_) => {
            false
        }

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
fn is_char(ty: &TypeRef) -> bool {
    match &**ty.type_() {
        Type::Builtin(BuiltinType::Char) => true,
        _ => false,
    }
}
