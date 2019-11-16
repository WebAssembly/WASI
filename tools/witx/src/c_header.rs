use crate::ast::*;
use heck::ShoutySnakeCase;

const PROLOGUE: &str = r#"/**
 * @file
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

const EPILOGUE: &str = r#"#ifdef __cplusplus
}
#endif

#endif"#;

pub fn to_c_header(doc: &Document) -> String {
    let mut ret = String::new();

    ret.push_str(PROLOGUE);

    for d in doc.datatypes() {
        print_datatype(&mut ret, &*d);
    }

    for m in doc.modules() {
        print_module(&mut ret, doc, &m);
    }

    ret.push_str(EPILOGUE);

    ret
}

fn print_datatype(ret: &mut String, d: &Datatype) {
    if !d.docs.is_empty() {
        ret.push_str("/**\n");
        for line in d.docs.lines() {
            ret.push_str(&format!(" * {}\n", line));
        }
        ret.push_str(" */\n");
    }

    match &d.variant {
        DatatypeVariant::Alias(a) => print_alias(ret, a),
        DatatypeVariant::Enum(e) => print_enum(ret, e),
        DatatypeVariant::Flags(f) => print_flags(ret, f),
        DatatypeVariant::Struct(s) => print_struct(ret, s),
        DatatypeVariant::Union(u) => print_union(ret, u),
        DatatypeVariant::Handle(h) => print_handle(ret, h),
    }
}

fn print_alias(ret: &mut String, a: &AliasDatatype) {
    match a.to {
        DatatypeIdent::Array(_) => {
            // Don't emit arrays as top-level types; instead we special-case
            // them in places like parameter lists so that we can pass them
            // as pointer and length pairs.
        }
        _ => {
            ret.push_str(&format!(
                "typedef {} __wasi_{}_t;\n",
                datatype_ident_name(&a.to),
                ident_name(&a.name)
            ));
            ret.push_str("\n");
        }
    }
}

fn print_enum(ret: &mut String, e: &EnumDatatype) {
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

fn print_flags(ret: &mut String, f: &FlagsDatatype) {
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

fn print_struct(ret: &mut String, s: &StructDatatype) {
    ret.push_str(&format!(
        "typedef struct __wasi_{}_t {{\n",
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

    ret.push_str(&format!("}} __wasi_{}_t;\n", ident_name(&s.name)));
    ret.push_str("\n");
}

fn print_union(ret: &mut String, u: &UnionDatatype) {
    ret.push_str(&format!(
        "typedef union __wasi_{}_t {{\n",
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

    ret.push_str(&format!("}} __wasi_{}_t;\n", ident_name(&u.name)));
    ret.push_str("\n");
}

fn print_handle(ret: &mut String, h: &HandleDatatype) {
    ret.push_str(&format!("typedef int __wasi_{}_t;", ident_name(&h.name)));
}

fn print_module(ret: &mut String, doc: &Document, m: &Module) {
    ret.push_str("/**\n");
    ret.push_str(&format!(" * @defgroup {}\n", ident_name(&m.name),));
    for line in m.docs.lines() {
        ret.push_str(&format!(" * {}\n", line));
    }
    ret.push_str(" * @{\n");
    ret.push_str(" */\n");
    ret.push_str("\n");

    for func in m.funcs() {
        print_func(ret, doc, &func, &m.name);
    }

    ret.push_str("/** @} */\n");
    ret.push_str("\n");
}

fn print_func(ret: &mut String, doc: &Document, func: &InterfaceFunc, module_name: &Id) {
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
    if func.results.is_empty() {
        if func.name.as_str() == "proc_exit" {
            ret.push_str("_Noreturn ");
        }
        ret.push_str("void ");
    } else {
        let first_result = &func.results[0];
        ret.push_str(&format!("{} ", datatype_ident_name(&first_result.type_)));
    }

    ret.push_str(&format!("__wasi_{}(\n", ident_name(&func.name)));

    if func.params.is_empty() && func.results.len() <= 1 {
        ret.push_str("    void\n");
    }
    for (index, param) in func.params.iter().enumerate() {
        if !param.docs.is_empty() {
            ret.push_str("    /**\n");
            for line in param.docs.lines() {
                ret.push_str(&format!("     * {}\n", line));
            }
            ret.push_str("     */\n");
        }
        add_params(ret, doc, &ident_name(&param.name), &param.type_);
        ret.push_str(&format!(
            "{}\n",
            if index + 1 < func.params.len() || func.results.len() > 1 {
                ",\n"
            } else {
                ""
            }
        ));
    }

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

    ret.push_str(") __attribute__((\n");
    ret.push_str(&format!(
        "    __import_module__(\"{}\"),\n",
        ident_name(module_name)
    ));
    ret.push_str(&format!(
        "    __import_name__(\"{}\")",
        ident_name(&func.name)
    ));
    if !func.results.is_empty() {
        ret.push_str(",\n    __warn_unused_result__\n");
    }
    ret.push_str("));\n");
    ret.push_str("\n");
}

fn add_params(ret: &mut String, doc: &Document, name: &str, type_: &DatatypeIdent) {
    match type_ {
        DatatypeIdent::Ident(i) => match &doc.datatype(&i.name).unwrap().as_ref().variant {
            DatatypeVariant::Alias(a) => add_resolved_params(ret, name, &a.to),
            _ => add_resolved_params(ret, name, type_),
        },
        _ => add_resolved_params(ret, name, type_),
    }
}

fn add_resolved_params(ret: &mut String, name: &str, type_: &DatatypeIdent) {
    match type_ {
        DatatypeIdent::Builtin(BuiltinType::String) => {
            ret.push_str(&format!("    const char *{},\n", name));
            ret.push_str("\n");
            ret.push_str("    /**\n");
            ret.push_str(&format!(
                "     * The length of the buffer pointed to by `{}`.\n",
                name,
            ));
            ret.push_str("     */\n");
            ret.push_str(&format!("    size_t {}_len", name));
        }
        DatatypeIdent::Array(element) => {
            ret.push_str(&format!(
                "    const {} *{},\n",
                datatype_ident_name(&element),
                name
            ));
            ret.push_str("\n");
            ret.push_str("    /**\n");
            ret.push_str(&format!(
                "     * The length of the array pointed to by `{}`.\n",
                name,
            ));
            ret.push_str("     */\n");
            ret.push_str(&format!("    size_t {}_len", name));
        }
        _ => {
            ret.push_str(&format!("    {} {}", datatype_ident_name(&type_), name));
        }
    }
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
        DatatypeIdent::Array(_) => unreachable!("arrays should be special-cased"),
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
