use crate::ast::*;
use heck::ShoutySnakeCase;

const PROLOGUE: &'static str = r#"/*
 * This file describes the WASI interface, consisting of functions, types,
 * and defined values (macros).
 *
 * The interface described here is greatly inspired by [CloudABI]'s clean,
 * thoughtfully-designed, cabability-oriented, POSIX-style API.
 *
 * [CloudABI]: https://github.com/NuxiNL/cloudlibc
 */

#ifndef __wasi_core_h
#define __wasi_core_h

#ifndef __wasi__
#error <wasi/core.h> is only supported on WASI platforms.
#endif

#include <stddef.h>
#include <stdint.h>

_Static_assert(_Alignof(int8_t) == 1, "non-wasi data layout");
_Static_assert(_Alignof(uint8_t) == 1, "non-wasi data layout");
_Static_assert(_Alignof(int16_t) == 2, "non-wasi data layout");
_Static_assert(_Alignof(uint16_t) == 2, "non-wasi data layout");
_Static_assert(_Alignof(int32_t) == 4, "non-wasi data layout");
_Static_assert(_Alignof(uint32_t) == 4, "non-wasi data layout");
_Static_assert(_Alignof(int64_t) == 8, "non-wasi data layout");
_Static_assert(_Alignof(uint64_t) == 8, "non-wasi data layout");

#ifdef __cplusplus
extern "C" {
#endif

"#;

const EPILOGUE: &'static str = r#"#ifdef __cplusplus
}
#endif

#endif"#;

pub fn to_c_header(doc: &Document) -> String {
    let mut ret = String::new();

    ret.push_str(PROLOGUE);

    for d in doc.datatypes() {
        datatype_to_c_header(&mut ret, &*d);
    }

    for m in doc.modules() {
        module_to_c_header(&mut ret, &m);
    }

    ret.push_str(EPILOGUE);

    ret
}

fn datatype_to_c_header(ret: &mut String, d: &Datatype) {
    if !d.docs.is_empty() {
        ret.push_str("/**\n");
        for line in d.docs.lines() {
            ret.push_str(&format!(" * {}\n", line));
        }
        ret.push_str(" */\n");
    }

    match &d.variant {
        DatatypeVariant::Alias(a) => alias_to_c_header(ret, a),
        DatatypeVariant::Enum(e) => enum_to_c_header(ret, e),
        DatatypeVariant::Flags(f) => flags_to_c_header(ret, f),
        DatatypeVariant::Struct(s) => struct_to_c_header(ret, s),
        DatatypeVariant::Union(u) => union_to_c_header(ret, u),
        DatatypeVariant::Handle(h) => handle_to_c_header(ret, h),
    }
}

fn alias_to_c_header(ret: &mut String, a: &AliasDatatype) {
    ret.push_str(&format!(
        "typedef {} __wasi_{}_t;\n",
        datatype_ident_name(&a.to),
        ident_name(&a.name)
    ));
    ret.push_str("\n");
}

fn enum_to_c_header(ret: &mut String, e: &EnumDatatype) {
    ret.push_str(&format!(
        "typedef {} __wasi_{}_t;\n",
        intrepr_name(e.repr),
        ident_name(&e.name)
    ));
    ret.push_str("\n");

    for (index, variant) in e.variants.iter().enumerate() {
        if !variant.docs.is_empty() {
            ret.push_str("/**\n");
            for line in variant.docs.lines() {
                ret.push_str(&format!(" * {}\n", line));
            }
            ret.push_str(" */\n");
        }
        ret.push_str(&format!(
            "#define __WASI_{}_{} ((__wasi_{}_t){})\n",
            ident_name(&e.name).to_shouty_snake_case(),
            ident_name(&variant.name).to_shouty_snake_case(),
            ident_name(&e.name),
            index
        ));
        ret.push_str("\n");
    }
}

fn flags_to_c_header(ret: &mut String, f: &FlagsDatatype) {
    ret.push_str(&format!(
        "typedef {} __wasi_{}_t;\n",
        intrepr_name(f.repr),
        ident_name(&f.name)
    ));
    ret.push_str("\n");

    for (index, flag) in f.flags.iter().enumerate() {
        if !flag.docs.is_empty() {
            ret.push_str("/**\n");
            for line in flag.docs.lines() {
                ret.push_str(&format!(" * {}\n", line));
            }
            ret.push_str(" */\n");
        }
        ret.push_str(&format!(
            "#define __WASI_{} ((__wasi_{}_t){})\n",
            ident_name(&flag.name).to_shouty_snake_case(),
            ident_name(&f.name),
            1u128 << index
        ));
        ret.push_str("\n");
    }
}

fn struct_to_c_header(ret: &mut String, s: &StructDatatype) {
    ret.push_str(&format!(
        "typedef struct __wasi_{} {{\n",
        ident_name(&s.name)
    ));

    for member in &s.members {
        if !member.docs.is_empty() {
            ret.push_str("    /**\n");
            for line in member.docs.lines() {
                ret.push_str(&format!("     * {}\n", line));
            }
            ret.push_str("     */\n");
        }
        ret.push_str(&format!(
            "    {} {};\n",
            datatype_ident_name(&member.type_),
            ident_name(&member.name)
        ));
        ret.push_str("\n");
    }

    ret.push_str(&format!("}} __wasi_{};\n", ident_name(&s.name)));
    ret.push_str("\n");
}

fn union_to_c_header(ret: &mut String, u: &UnionDatatype) {
    ret.push_str(&format!(
        "typedef union __wasi_{} {{\n",
        ident_name(&u.name)
    ));

    for variant in &u.variants {
        if !variant.docs.is_empty() {
            ret.push_str("    /**\n");
            for line in variant.docs.lines() {
                ret.push_str(&format!("     * {}\n", line));
            }
            ret.push_str("     */\n");
        }
        ret.push_str(&format!(
            "    {} {};\n",
            datatype_ident_name(&variant.type_),
            ident_name(&variant.name)
        ));
        ret.push_str("\n");
    }

    ret.push_str(&format!("}} __wasi_{};\n", ident_name(&u.name)));
    ret.push_str("\n");
}

fn handle_to_c_header(ret: &mut String, h: &HandleDatatype) {
    ret.push_str(&format!("typedef int __wasi_{}_t;", ident_name(&h.name)));
}

fn module_to_c_header(ret: &mut String, m: &Module) {
    ret.push_str("/**\n");
    ret.push_str(&format!(
        " * @defgroup {} {}\n",
        ident_name(&m.name),
        m.docs.lines().next().unwrap_or("(not documented yet)")
    ));
    for line in m.docs.lines() {
        ret.push_str(&format!(" * {}\n", line));
    }
    ret.push_str(" * @{\n");
    ret.push_str(" */\n");
    ret.push_str("\n");

    for func in m.funcs() {
        func_to_c_header(ret, &func, &m.name);
    }

    ret.push_str("/** @} */\n");
    ret.push_str("\n");
}

fn func_to_c_header(ret: &mut String, func: &InterfaceFunc, module_name: &Id) {
    if !func.docs.is_empty() {
        ret.push_str("/**\n");
        for line in func.docs.lines() {
            ret.push_str(&format!(" * {}\n", line));
        }
        if !func.results.is_empty() {
            let first_result = &func.results[0];
            if !first_result.docs.is_empty() {
                ret.push_str(" * @return\n");
                for line in first_result.docs.lines() {
                    ret.push_str(&format!(" * {}", line));
                }
            }
        }
        ret.push_str(" */\n");
    }
    if !func.results.is_empty() {
        let first_result = &func.results[0];
        ret.push_str(&format!("{} ", datatype_ident_name(&first_result.type_)));
    }

    ret.push_str(&format!("__wasi_{}_t(\n", ident_name(&func.name)));

    for (index, param) in func.params.iter().enumerate() {
        if !param.docs.is_empty() {
            ret.push_str("    /**\n");
            for line in param.docs.lines() {
                ret.push_str(&format!("     * {}\n", line));
            }
            ret.push_str("     */\n");
        }
        ret.push_str(&format!(
            "    {} {}{}\n",
            datatype_ident_name(&param.type_),
            ident_name(&param.name),
            if index + 1 < func.params.len() || func.results.len() > 1 {
                ","
            } else {
                ""
            }
        ));
    }

    if !func.results.is_empty() {
        for (index, result) in func.results.iter().enumerate() {
            if index == 0 {
                // The first result is returned by value above.
                continue;
            }
            if !result.docs.is_empty() {
                ret.push_str("    /**\n");
                for line in result.docs.lines() {
                    ret.push_str(&format!("     * {}\n", line));
                }
                ret.push_str("     */\n");
            }
            ret.push_str(&format!(
                "    {} *{}{}\n",
                datatype_ident_name(&result.type_),
                ident_name(&result.name),
                if index + 1 < func.results.len() {
                    ","
                } else {
                    ""
                }
            ));
        }
    }

    ret.push_str(&format!(
        ") __attribute__((__import_module__(\"{}\"), __import_name__(\"{}\")))",
        ident_name(module_name),
        ident_name(&func.name)
    ));
    if !func.results.is_empty() {
        ret.push_str(" __attribute__((__warn_unused_result__))");
    }
    ret.push_str(";\n");
    ret.push_str("\n");
}

fn ident_name(i: &Id) -> String {
    i.as_str().to_string()
}

fn builtin_type_name(b: BuiltinType) -> &'static str {
    match b {
        BuiltinType::String => "string",
        BuiltinType::U8 => "uint8_t",
        BuiltinType::U16 => "uint16_t",
        BuiltinType::U32 => "uint32_t",
        BuiltinType::U64 => "uint64_t",
        BuiltinType::S8 => "int8_t",
        BuiltinType::S16 => "int16_t",
        BuiltinType::S32 => "int32_t",
        BuiltinType::S64 => "int64_t",
        BuiltinType::F32 => "float",
        BuiltinType::F64 => "double",
    }
}

fn datatype_ident_name(data_ty: &DatatypeIdent) -> String {
    match data_ty {
        DatatypeIdent::Builtin(b) => builtin_type_name(*b).to_string(),
        DatatypeIdent::Array(a) => format!("Array<{}>", datatype_ident_name(&*a)),
        DatatypeIdent::Pointer(p) => format!("{} *", datatype_ident_name(&*p)),
        DatatypeIdent::ConstPointer(p) => format!("const {} *", datatype_ident_name(&*p)),
        DatatypeIdent::Ident(i) => format!("__wasi_{}_t", i.name.as_str()),
    }
}

fn intrepr_name(i: IntRepr) -> &'static str {
    match i {
        IntRepr::U8 => "uint8_t",
        IntRepr::U16 => "uint16_t",
        IntRepr::U32 => "uint32_t",
        IntRepr::U64 => "uint64_t",
    }
}
