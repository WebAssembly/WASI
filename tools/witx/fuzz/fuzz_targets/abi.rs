#![no_main]

use arbitrary::{Result, Unstructured};
use libfuzzer_sys::fuzz_target;
use std::rc::Rc;
use witx::*;

fuzz_target!(|data: &[u8]| {
    drop(fuzz(&mut Unstructured::new(data)));
});

fn fuzz(u: &mut Unstructured<'_>) -> Result<()> {
    let results = params("r", u)?;
    let params = params("p", u)?;
    let func = InterfaceFunc {
        abi: abi(u)?,
        docs: String::new(),
        name: Id::new("foo"),
        noreturn: results.len() == 0 && u.arbitrary()?,
        params,
        results,
    };

    if func.abi.validate(&func.params, &func.results).is_ok() {
        func.call(&Id::new("bar"), call_mode(u)?, &mut Bindgen);
    }

    Ok(())
}

fn abi(_: &mut Unstructured<'_>) -> Result<Abi> {
    Ok(Abi::Next)
}

fn call_mode(u: &mut Unstructured<'_>) -> Result<CallMode> {
    match u.int_in_range(0..=3)? {
        0 => Ok(CallMode::DefinedExport),
        1 => Ok(CallMode::DefinedImport),
        2 => Ok(CallMode::DeclaredExport),
        _ => Ok(CallMode::DeclaredImport),
    }
}

fn params(prefix: &str, u: &mut Unstructured<'_>) -> Result<Vec<InterfaceFuncParam>> {
    let len = u.int_in_range(0..=20)?;
    (0..len).map(|i| param(prefix, i, u)).collect()
}

fn param(prefix: &str, i: usize, u: &mut Unstructured<'_>) -> Result<InterfaceFuncParam> {
    Ok(InterfaceFuncParam {
        docs: String::new(),
        name: Id::new(format!("{}{}", prefix, i)),
        tref: tref(&mut 100, u)?,
    })
}

fn tref(fuel: &mut usize, u: &mut Unstructured<'_>) -> Result<TypeRef> {
    let ty = if *fuel == 0 {
        Type::Builtin(BuiltinType::U64)
    } else {
        match u.int_in_range(0..=6)? {
            0 => Type::Builtin(builtin(u)?),
            1 => Type::Handle(HandleDatatype {}),
            2 => {
                *fuel -= 1;
                Type::List(tref(fuel, u)?)
            }
            3 => {
                *fuel -= 1;
                Type::Pointer(tref(fuel, u)?)
            }
            4 => {
                *fuel -= 1;
                Type::ConstPointer(tref(fuel, u)?)
            }
            5 => {
                let (kind, max) = record_kind(u)?;
                let nfields = u.int_in_range(0..=(*fuel).min(20).min(max))?;
                *fuel -= nfields;
                let members = (0..nfields)
                    .map(|i| member(i, fuel, &kind, u))
                    .collect::<Result<Vec<_>>>()?;
                Type::Record(RecordDatatype { kind, members })
            }
            _ => {
                let tag_repr = int_repr(u)?;
                let ncases = u.int_in_range(0..=(*fuel).min(20))?;
                *fuel -= ncases;
                let cases = (0..ncases)
                    .map(|i| case(i, fuel, u))
                    .collect::<Result<Vec<_>>>()?;
                Type::Variant(Variant { tag_repr, cases })
            }
        }
    };
    Ok(TypeRef::Name(Rc::new(NamedType {
        name: Id::new("ty"),
        tref: TypeRef::Value(Rc::new(ty)),
        docs: String::new(),
    })))
}

fn member(
    i: usize,
    fuel: &mut usize,
    kind: &RecordKind,
    u: &mut Unstructured<'_>,
) -> Result<RecordMember> {
    Ok(match kind {
        RecordKind::Bitflags(_) => RecordMember {
            name: Id::new(format!("f{}", i)),
            docs: String::new(),
            tref: TypeRef::Value(Rc::new(Type::Variant(Variant {
                tag_repr: IntRepr::U8,
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
        },
        RecordKind::Tuple => RecordMember {
            name: Id::new(format!("{}", i)),
            docs: String::new(),
            tref: tref(fuel, u)?,
        },
        RecordKind::Other => RecordMember {
            name: Id::new(format!("m{}", i)),
            docs: String::new(),
            tref: tref(fuel, u)?,
        },
    })
}

fn case(i: usize, fuel: &mut usize, u: &mut Unstructured<'_>) -> Result<Case> {
    Ok(Case {
        name: Id::new(format!("m{}", i)),
        docs: String::new(),
        tref: if u.arbitrary()? {
            Some(tref(fuel, u)?)
        } else {
            None
        },
    })
}

fn record_kind(u: &mut Unstructured<'_>) -> Result<(RecordKind, usize)> {
    Ok(match u.int_in_range(0..=10)? {
        0 => (RecordKind::Tuple, usize::max_value()),
        1 => {
            let repr = int_repr(u)?;
            let max = match repr {
                IntRepr::U8 => 8,
                IntRepr::U16 => 16,
                IntRepr::U32 => 32,
                IntRepr::U64 => 64,
            };
            (RecordKind::Bitflags(repr), max)
        }
        _ => (RecordKind::Other, usize::max_value()),
    })
}

fn int_repr(u: &mut Unstructured<'_>) -> Result<IntRepr> {
    Ok(match u.int_in_range(0..=3)? {
        0 => IntRepr::U8,
        1 => IntRepr::U16,
        2 => IntRepr::U32,
        _ => IntRepr::U64,
    })
}

fn builtin(u: &mut Unstructured<'_>) -> Result<BuiltinType> {
    Ok(match u.int_in_range(0..=12)? {
        0 => BuiltinType::Char,
        1 => BuiltinType::U8 { lang_c_char: false },
        2 => BuiltinType::U8 { lang_c_char: true },
        3 => BuiltinType::U16,
        4 => BuiltinType::U32 {
            lang_ptr_size: false,
        },
        5 => BuiltinType::U32 {
            lang_ptr_size: true,
        },
        6 => BuiltinType::U64,
        7 => BuiltinType::S8,
        8 => BuiltinType::S16,
        9 => BuiltinType::S32,
        10 => BuiltinType::S64,
        11 => BuiltinType::F32,
        _ => BuiltinType::F64,
    })
}

struct Bindgen;

impl witx::Bindgen for Bindgen {
    type Operand = ();

    fn emit(
        &mut self,
        inst: &Instruction<'_>,
        _operands: &mut Vec<Self::Operand>,
        results: &mut Vec<Self::Operand>,
    ) {
        for _ in 0..inst.results_len() {
            results.push(());
        }
    }
    fn allocate_typed_space(&mut self, _ty: &NamedType) -> Self::Operand {
        ()
    }
    fn allocate_i64_array(&mut self, _amt: usize) -> Self::Operand {
        ()
    }
    fn push_block(&mut self) {}
    fn finish_block(&mut self, _operand: &mut Vec<Self::Operand>) {}
}
