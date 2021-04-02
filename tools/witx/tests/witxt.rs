//! You can run this test suite with:
//!
//!     cargo test --test witxt
//!
//! An argument can be passed as well to filter, based on filename, which test
//! to run
//!
//!     cargo test --test witxt foo.witxt

use anyhow::{anyhow, bail, Context, Result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use wast::parser::{self, Cursor, Parse, ParseBuffer, Parser, Peek};
use witx::{Instruction, WasmType};

fn main() {
    let tests = find_tests();
    let filter = std::env::args().nth(1);

    let tests = tests
        .par_iter()
        .filter_map(|test| {
            if let Some(filter) = &filter {
                if let Some(s) = test.to_str() {
                    if !s.contains(filter) {
                        return None;
                    }
                }
            }
            let contents = std::fs::read(test).unwrap();
            Some((test, contents))
        })
        .collect::<Vec<_>>();

    println!("running {} test files\n", tests.len());

    let ntests = AtomicUsize::new(0);
    let errors = tests
        .par_iter()
        .filter_map(|(test, contents)| {
            WitxtRunner {
                ntests: &ntests,
                modules: HashMap::new(),
            }
            .run(test, contents)
            .err()
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for msg in errors.iter() {
            eprintln!("{:?}", msg);
        }

        panic!("{} tests failed", errors.len())
    }

    println!(
        "test result: ok. {} directives passed\n",
        ntests.load(SeqCst)
    );
}

/// Recursively finds all tests in a whitelisted set of directories which we
/// then load up and test in parallel.
fn find_tests() -> Vec<PathBuf> {
    let mut tests = Vec::new();
    find_tests("tests/witxt".as_ref(), &mut tests);
    tests.sort();
    return tests;

    fn find_tests(path: &Path, tests: &mut Vec<PathBuf>) {
        for f in path.read_dir().unwrap() {
            let f = f.unwrap();
            if f.file_type().unwrap().is_dir() {
                find_tests(&f.path(), tests);
                continue;
            }

            match f.path().extension().and_then(|s| s.to_str()) {
                Some("witxt") => {}
                _ => continue,
            }
            tests.push(f.path());
        }
    }
}

struct WitxtRunner<'a> {
    ntests: &'a AtomicUsize,
    modules: HashMap<String, witx::Module>,
}

impl WitxtRunner<'_> {
    fn run(&mut self, test: &Path, contents: &[u8]) -> Result<()> {
        let contents = str::from_utf8(contents)?;
        macro_rules! adjust {
            ($e:expr) => {{
                let mut e = wast::Error::from($e);
                e.set_path(test);
                e.set_text(contents);
                e
            }};
        }
        let buf = ParseBuffer::new(contents).map_err(|e| adjust!(e))?;
        let witxt = parser::parse::<Witxt>(&buf).map_err(|e| adjust!(e))?;

        let errors = witxt
            .directives
            .into_iter()
            .filter_map(|directive| {
                let (line, col) = directive.span().linecol_in(contents);
                self.test_directive(contents, test, directive)
                    .with_context(|| {
                        format!(
                            "failed directive on {}:{}:{}",
                            test.display(),
                            line + 1,
                            col + 1
                        )
                    })
                    .err()
            })
            .collect::<Vec<_>>();
        if errors.is_empty() {
            return Ok(());
        }
        let mut s = format!("{} test failures in {}:", errors.len(), test.display());
        for mut error in errors {
            if let Some(err) = error.downcast_mut::<wast::Error>() {
                err.set_path(test);
                err.set_text(contents);
            }
            s.push_str("\n\n\t--------------------------------\n\n\t");
            s.push_str(&format!("{:?}", error).replace("\n", "\n\t"));
        }
        bail!("{}", s)
    }

    fn test_directive(
        &mut self,
        contents: &str,
        test: &Path,
        directive: WitxtDirective,
    ) -> Result<()> {
        self.bump_ntests();
        match directive {
            WitxtDirective::Witx(witx) => {
                let doc = witx.module(contents, test)?;
                self.assert_md(&doc)?;
                if let Some(name) = witx.id {
                    self.modules.insert(name.name().to_string(), doc);
                }
            }
            WitxtDirective::AssertInvalid { witx, message, .. } => {
                let err = match witx.module(contents, test) {
                    Ok(_) => bail!("witx was valid when it shouldn't be"),
                    Err(e) => format!("{:?}", anyhow::Error::from(e)),
                };
                if !err.contains(message) {
                    bail!("expected error {:?}\nfound error {}", message, err);
                }
            }
            WitxtDirective::AssertEq { eq, t1, t2, .. } => {
                let (t1m, t1t) = t1;
                let (t2m, t2t) = t2;
                let t1d = self
                    .modules
                    .get(t1m.name())
                    .ok_or_else(|| anyhow!("no document named {:?}", t1m.name()))?;
                let t2d = self
                    .modules
                    .get(t2m.name())
                    .ok_or_else(|| anyhow!("no document named {:?}", t2m.name()))?;
                let t1 = t1d
                    .typename(&witx::Id::new(t1t))
                    .ok_or_else(|| anyhow!("no type named {:?}", t1t))?;
                let t2 = t2d
                    .typename(&witx::Id::new(t2t))
                    .ok_or_else(|| anyhow!("no type named {:?}", t2t))?;
                if t1.tref.type_equal(&t2.tref) != eq {
                    bail!("failed comparing {:?} and {:?}", t1, t2);
                }
            }
            WitxtDirective::AssertAbi {
                witx,
                wasm_signatures,
                abis,
                ..
            } => {
                let module = witx.module(contents, test)?;
                let func = module.funcs().next().ok_or_else(|| anyhow!("no funcs"))?;
                for (mode, wasm_params, wasm_results) in wasm_signatures.iter() {
                    let sig = func.wasm_signature(*mode);
                    if sig.params != *wasm_params {
                        bail!("expected params {:?}, found {:?}", wasm_params, sig.params);
                    }
                    if sig.results != *wasm_results {
                        bail!(
                            "expected results {:?}, found {:?}",
                            wasm_results,
                            sig.results
                        );
                    }
                }

                for abi in abis {
                    let mut check = AbiBindgen {
                        abi: abi.instrs.iter(),
                        err: None,
                        contents,
                        next_rp: 0,
                    };
                    func.call(module.name(), abi.mode, &mut check);
                    check.check()?;
                }
            }
        }
        Ok(())
    }

    fn assert_md(&self, m: &witx::Module) -> Result<()> {
        self.bump_ntests();
        witx::document(Some(m));
        Ok(())
    }

    fn bump_ntests(&self) {
        self.ntests.fetch_add(1, SeqCst);
    }
}

struct AbiBindgen<'a> {
    abi: std::slice::Iter<'a, (wast::Span, &'a str)>,
    err: Option<anyhow::Error>,
    contents: &'a str,
    next_rp: usize,
}

impl AbiBindgen<'_> {
    fn check(&mut self) -> Result<()> {
        match self.err.take() {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    fn assert(&mut self, name: &str) {
        if self.err.is_some() {
            return;
        }
        match self.abi.next() {
            Some((_, s)) if *s == name => {}
            Some((span, s)) => {
                let (line, col) = span.linecol_in(self.contents);
                self.err = Some(anyhow!(
                    "line {}:{} - expected `{}` but adapter produced `{}`",
                    line + 1,
                    col + 1,
                    s,
                    name,
                ));
            }
            None => {
                self.err = Some(anyhow!(
                    "extra instruction `{}` found when none was expected",
                    name
                ));
            }
        }
    }

    fn assert_op(&mut self, op: &Operand) {
        match op {
            Operand::Op(n) => self.assert(&format!("arg{}", n)),
            Operand::Ret(n) => self.assert(&format!("ret{}", n)),
            Operand::Field(n) => self.assert(&format!("field.{}", n)),
            Operand::Rp(n) => self.assert(&format!("rp{}", n)),
            Operand::InstResult => {}
        }
    }

    fn assert_mem(&mut self, inst: &str, offset: i32) {
        self.assert(inst);
        self.assert(&format!("offset={}", offset));
    }
}

#[derive(Clone, Debug)]
enum Operand {
    Op(usize),
    Ret(usize),
    Field(String),
    Rp(usize),
    InstResult,
}

impl witx::Bindgen for AbiBindgen<'_> {
    type Operand = Operand;
    fn emit(
        &mut self,
        inst: &Instruction<'_>,
        operands: &mut Vec<Self::Operand>,
        results: &mut Vec<Self::Operand>,
    ) {
        use witx::Instruction::*;
        use witx::WitxInstruction::*;
        match inst {
            GetArg { nth } => {
                self.assert(&format!("get-arg{}", nth));
                results.push(Operand::Op(*nth));
            }
            I32FromChar => self.assert("i32.from_char"),
            I64FromU64 => self.assert("i64.from_u64"),
            I64FromS64 => self.assert("i64.from_s64"),
            I32FromU32 => self.assert("i32.from_u32"),
            I32FromS32 => self.assert("i32.from_s32"),
            I32FromUsize => self.assert("i32.from_usize"),
            I32FromU16 => self.assert("i32.from_u16"),
            I32FromS16 => self.assert("i32.from_s16"),
            I32FromU8 => self.assert("i32.from_u8"),
            I32FromS8 => self.assert("i32.from_s8"),
            I32FromChar8 => self.assert("i32.from_char8"),
            I32FromBorrowedHandle { .. } => self.assert("i32.from_borrowed_handle"),
            I32FromOwnedHandle { .. } => self.assert("i32.from_owned_handle"),
            F32FromIf32 => self.assert("f32.from_if32"),
            F64FromIf64 => self.assert("f64.from_if64"),
            CallWasm {
                results: call_results,
                ..
            } => {
                self.assert("call.wasm");
                for i in 0..call_results.len() {
                    results.push(Operand::Ret(i));
                }
            }
            CallInterface { func, .. } => {
                self.assert("call.interface");
                for i in 0..func.results.len() {
                    results.push(Operand::Ret(i));
                }
            }
            S8FromI32 => self.assert("s8.from_i32"),
            U8FromI32 => self.assert("u8.from_i32"),
            S16FromI32 => self.assert("s16.from_i32"),
            U16FromI32 => self.assert("u16.from_i32"),
            S32FromI32 => self.assert("s32.from_i32"),
            U32FromI32 => self.assert("u32.from_i32"),
            S64FromI64 => self.assert("s64.from_i64"),
            U64FromI64 => self.assert("u64.from_i64"),
            CharFromI32 => self.assert("char.from_i32"),
            Char8FromI32 => self.assert("char8.from_i32"),
            UsizeFromI32 => self.assert("usize.from_i32"),
            If32FromF32 => self.assert("if32.from_f32"),
            If64FromF64 => self.assert("if64.from_f64"),
            HandleBorrowedFromI32 { .. } => self.assert("handle.borrowed_from_i32"),
            HandleOwnedFromI32 { .. } => self.assert("handle.owned_from_i32"),
            Return { .. } => self.assert("return"),
            VariantPayload => self.assert("variant-payload"),
            RecordLift { .. } => self.assert("record-lift"),
            RecordLower { ty, .. } => {
                self.assert("record-lower");
                for member in ty.members.iter() {
                    results.push(Operand::Field(member.name.as_str().to_string()));
                }
            }

            I32Const { val } => {
                self.assert("i32.const");
                self.assert(&val.to_string());
            }
            VariantLower { .. } => self.assert("variant-lower"),
            VariantLift { .. } => self.assert("variant-lift"),
            Bitcasts { casts } => {
                for (cast, operand) in casts.iter().zip(operands.drain(..)) {
                    match cast {
                        witx::Bitcast::None => self.assert("nocast"),
                        witx::Bitcast::I32ToI64 => self.assert("i32-to-i64"),
                        witx::Bitcast::F32ToF64 => self.assert("f32-to-f64"),
                        witx::Bitcast::F32ToI32 => self.assert("f32-to-i32"),
                        witx::Bitcast::F64ToI64 => self.assert("f64-to-i64"),
                        witx::Bitcast::I64ToI32 => self.assert("i64-to-i32"),
                        witx::Bitcast::F64ToF32 => self.assert("f64-to-f32"),
                        witx::Bitcast::I32ToF32 => self.assert("i32-to-f32"),
                        witx::Bitcast::I64ToF64 => self.assert("i64-to-f64"),
                        witx::Bitcast::F32ToI64 => self.assert("f32-to-i64"),
                        witx::Bitcast::I64ToF32 => self.assert("i64-to-f32"),
                    }
                    self.assert_op(&operand);
                }
            }
            ConstZero { tys } => {
                for ty in tys.iter() {
                    match ty {
                        witx::WasmType::I32 => self.assert("i32.const"),
                        witx::WasmType::I64 => self.assert("i64.const"),
                        witx::WasmType::F32 => self.assert("f32.const"),
                        witx::WasmType::F64 => self.assert("f64.const"),
                    }
                    self.assert("0");
                }
            }
            I32Load { offset } => self.assert_mem("i32.load", *offset),
            I32Load8U { offset } => self.assert_mem("i32.load8u", *offset),
            I32Load8S { offset } => self.assert_mem("i32.load8s", *offset),
            I32Load16U { offset } => self.assert_mem("i32.load16u", *offset),
            I32Load16S { offset } => self.assert_mem("i32.load16s", *offset),
            I64Load { offset } => self.assert_mem("i64.load", *offset),
            F32Load { offset } => self.assert_mem("f32.load", *offset),
            F64Load { offset } => self.assert_mem("f64.load", *offset),
            I32Store { offset } => self.assert_mem("i32.store", *offset),
            I32Store8 { offset } => self.assert_mem("i32.store8", *offset),
            I32Store16 { offset } => self.assert_mem("i32.store16", *offset),
            I64Store { offset } => self.assert_mem("i64.store", *offset),
            F32Store { offset } => self.assert_mem("f32.store", *offset),
            F64Store { offset } => self.assert_mem("f64.store", *offset),

            ListCanonLower { .. } => self.assert("list.canon_lower"),
            ListLower { .. } => self.assert("list.lower"),
            ListCanonLift { .. } => self.assert("list.canon_lift"),
            ListLift { .. } => self.assert("list.lift"),
            IterElem => self.assert("iter-elem"),
            IterBasePointer => self.assert("iter-base-pointer"),

            I32FromBitflags { .. } => self.assert("i32.from_bitflags"),
            BitflagsFromI32 { .. } => self.assert("bitflags.from_i32"),
            I64FromBitflags { .. } => self.assert("i64.from_bitflags"),
            BitflagsFromI64 { .. } => self.assert("bitflags.from_i64"),

            BufferLowerPtrLen { .. } => self.assert("buffer.lower_ptr_len"),
            BufferLowerHandle { .. } => self.assert("buffer.lower_handle"),
            BufferLiftPtrLen { .. } => self.assert("buffer.lift_ptr_len"),
            BufferLiftHandle { .. } => self.assert("buffer.lift_handle"),

            Witx { instr } => match instr {
                PointerFromI32 { .. } => self.assert("pointer.from_i32"),
                ConstPointerFromI32 { .. } => self.assert("const_pointer.from_i32"),
                ReuseReturn => self.assert("reuse_return"),
                I32FromPointer => self.assert("i32.from_pointer"),
                I32FromConstPointer => self.assert("i32.from_const_pointer"),
                AddrOf => self.assert("addr-of"),
            },
        }

        for op in operands.iter() {
            self.assert_op(op);
        }

        while results.len() < inst.results_len() {
            results.push(Operand::InstResult);
        }
    }

    fn allocate_typed_space(&mut self, _: &witx::NamedType) -> Self::Operand {
        self.next_rp += 1;
        Operand::Rp(self.next_rp - 1)
    }

    fn allocate_i64_array(&mut self, _: usize) -> Self::Operand {
        Operand::Rp(0)
    }

    fn push_block(&mut self) {
        self.assert("block.push");
    }

    fn finish_block(&mut self, _operands: &mut Vec<Self::Operand>) {
        self.assert("block.finish");
    }
}

mod kw {
    wast::custom_keyword!(assert_invalid);
    wast::custom_keyword!(assert_eq);
    wast::custom_keyword!(assert_ne);
    wast::custom_keyword!(assert_abi);
    wast::custom_keyword!(witx);
    wast::custom_keyword!(load);
    wast::custom_keyword!(call_wasm);
    wast::custom_keyword!(call_interface);
    wast::custom_keyword!(param);
    wast::custom_keyword!(result);
    wast::custom_keyword!(wasm);
    wast::custom_keyword!(i32);
    wast::custom_keyword!(i64);
    wast::custom_keyword!(f32);
    wast::custom_keyword!(f64);
    wast::custom_keyword!(declared_import);
    wast::custom_keyword!(declared_export);
    wast::custom_keyword!(defined_import);
    wast::custom_keyword!(defined_export);
}

struct Witxt<'a> {
    directives: Vec<WitxtDirective<'a>>,
}

impl<'a> Parse<'a> for Witxt<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let _r1 = parser.register_annotation("interface");
        let _r2 = parser.register_annotation("witx");
        let mut directives = Vec::new();
        while !parser.is_empty() {
            directives.push(parser.parens(|p| p.parse())?);
        }
        Ok(Witxt { directives })
    }
}

enum WitxtDirective<'a> {
    Witx(Witx<'a>),
    AssertInvalid {
        span: wast::Span,
        witx: Witx<'a>,
        message: &'a str,
    },
    AssertEq {
        span: wast::Span,
        eq: bool,
        t1: (wast::Id<'a>, &'a str),
        t2: (wast::Id<'a>, &'a str),
    },
    AssertAbi {
        span: wast::Span,
        witx: Witx<'a>,
        wasm_signatures: Vec<(witx::CallMode, Vec<WasmType>, Vec<WasmType>)>,
        abis: Vec<Abi<'a>>,
    },
}

impl WitxtDirective<'_> {
    fn span(&self) -> wast::Span {
        match self {
            WitxtDirective::Witx(w) => w.span,
            WitxtDirective::AssertInvalid { span, .. }
            | WitxtDirective::AssertAbi { span, .. }
            | WitxtDirective::AssertEq { span, .. } => *span,
        }
    }
}

impl<'a> Parse<'a> for WitxtDirective<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::witx>() {
            Ok(WitxtDirective::Witx(parser.parse()?))
        } else if l.peek::<kw::assert_invalid>() {
            let span = parser.parse::<kw::assert_invalid>()?.0;
            Ok(WitxtDirective::AssertInvalid {
                span,
                witx: parser.parens(|p| p.parse())?,
                message: parser.parse()?,
            })
        } else if l.peek::<kw::assert_eq>() {
            let span = parser.parse::<kw::assert_eq>()?.0;
            Ok(WitxtDirective::AssertEq {
                span,
                eq: true,
                t1: (parser.parse()?, parser.parse()?),
                t2: (parser.parse()?, parser.parse()?),
            })
        } else if l.peek::<kw::assert_ne>() {
            let span = parser.parse::<kw::assert_ne>()?.0;
            Ok(WitxtDirective::AssertEq {
                span,
                eq: false,
                t1: (parser.parse()?, parser.parse()?),
                t2: (parser.parse()?, parser.parse()?),
            })
        } else if l.peek::<kw::assert_abi>() {
            let span = parser.parse::<kw::assert_abi>()?.0;
            Ok(WitxtDirective::AssertAbi {
                span,
                witx: parser.parens(|p| p.parse())?,
                wasm_signatures: {
                    let mut signatures = Vec::new();
                    while parser.peek2::<kw::wasm>() {
                        signatures.push(parser.parens(|p| {
                            p.parse::<kw::wasm>()?;
                            let mode = p
                                .parse::<Option<CallMode>>()?
                                .map(|p| p.0)
                                .unwrap_or(witx::CallMode::DeclaredImport);
                            let mut params = Vec::new();
                            let mut results = Vec::new();
                            if p.peek2::<kw::param>() {
                                p.parens(|p| {
                                    p.parse::<kw::param>()?;
                                    while !p.is_empty() {
                                        params.push(parse_wasmtype(p)?);
                                    }
                                    Ok(())
                                })?;
                            }
                            if p.peek2::<kw::result>() {
                                p.parens(|p| {
                                    p.parse::<kw::result>()?;
                                    while !p.is_empty() {
                                        results.push(parse_wasmtype(p)?);
                                    }
                                    Ok(())
                                })?;
                            }
                            Ok((mode, params, results))
                        })?);
                    }
                    signatures
                },
                abis: {
                    let mut abis = Vec::new();
                    while !parser.is_empty() {
                        abis.push(parser.parens(|p| p.parse())?)
                    }
                    abis
                },
            })
        } else {
            Err(l.error())
        }
    }
}

fn parse_wasmtype(p: Parser<'_>) -> parser::Result<WasmType> {
    let mut l = p.lookahead1();
    if l.peek::<kw::i32>() {
        p.parse::<kw::i32>()?;
        Ok(WasmType::I32)
    } else if l.peek::<kw::i64>() {
        p.parse::<kw::i64>()?;
        Ok(WasmType::I64)
    } else if l.peek::<kw::f32>() {
        p.parse::<kw::f32>()?;
        Ok(WasmType::F32)
    } else if l.peek::<kw::f64>() {
        p.parse::<kw::f64>()?;
        Ok(WasmType::F64)
    } else {
        Err(l.error())
    }
}

struct Witx<'a> {
    span: wast::Span,
    id: Option<wast::Id<'a>>,
    def: WitxDef<'a>,
}

enum WitxDef<'a> {
    Fs(&'a str),
    Inline(witx::parser::TopLevelModule<'a>),
}

impl Witx<'_> {
    fn module(&self, contents: &str, file: &Path) -> Result<witx::Module> {
        use witx::parser::TopLevelSyntax;

        match &self.def {
            WitxDef::Inline(doc) => {
                let mut validator = witx::ModuleValidation::new(contents, file);
                for t in doc.decls.iter() {
                    match &t.item {
                        TopLevelSyntax::Decl(d) => validator
                            .validate_decl(&d, &t.comments)
                            .map_err(|e| anyhow::anyhow!("{}", e.report()))?,
                        TopLevelSyntax::Use(_) => unimplemented!(),
                    }
                }
                for f in doc.functions.iter() {
                    validator
                        .validate_function(&f.item, &f.comments)
                        .map_err(|e| anyhow::anyhow!("{}", e.report()))?;
                }
                Ok(validator.into_module())
            }
            WitxDef::Fs(path) => {
                let parent = file.parent().unwrap();
                let path = parent.join(path);
                Ok(witx::load(&path).map_err(|e| anyhow::anyhow!("{}", e.report()))?)
            }
        }
    }
}

impl<'a> Parse<'a> for Witx<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let span = parser.parse::<kw::witx>()?.0;
        let id = parser.parse()?;

        let def = if parser.peek2::<kw::load>() {
            parser.parens(|p| {
                p.parse::<kw::load>()?;
                Ok(WitxDef::Fs(p.parse()?))
            })?
        } else {
            WitxDef::Inline(parser.parse()?)
        };
        Ok(Witx { id, span, def })
    }
}

struct Abi<'a> {
    mode: witx::CallMode,
    instrs: Vec<(wast::Span, &'a str)>,
}

impl<'a> Parse<'a> for Abi<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let mut l = parser.lookahead1();
        let mode = if l.peek::<kw::declared_import>() {
            parser.parse::<kw::declared_import>()?;
            witx::CallMode::DeclaredImport
        } else if l.peek::<kw::declared_export>() {
            parser.parse::<kw::declared_export>()?;
            witx::CallMode::DeclaredExport
        } else if l.peek::<kw::defined_import>() {
            parser.parse::<kw::defined_import>()?;
            witx::CallMode::DefinedImport
        } else if l.peek::<kw::defined_export>() {
            parser.parse::<kw::defined_export>()?;
            witx::CallMode::DefinedExport
        } else {
            return Err(l.error());
        };
        let mut instrs = Vec::new();
        while !parser.is_empty() {
            instrs.push(parser.step(|cursor| {
                let (kw, next) = match cursor.keyword() {
                    Some(pair) => pair,
                    None => match cursor.integer() {
                        Some((i, next)) => (i.src(), next),
                        None => return Err(cursor.error("expected keyword or integer")),
                    },
                };
                Ok(((cursor.cur_span(), kw), next))
            })?);
        }
        Ok(Abi { mode, instrs })
    }
}

struct CallMode(witx::CallMode);

impl<'a> Parse<'a> for CallMode {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let mut l = parser.lookahead1();
        Ok(CallMode(if l.peek::<kw::declared_import>() {
            parser.parse::<kw::declared_import>()?;
            witx::CallMode::DeclaredImport
        } else if l.peek::<kw::declared_export>() {
            parser.parse::<kw::declared_export>()?;
            witx::CallMode::DeclaredExport
        } else if l.peek::<kw::defined_import>() {
            parser.parse::<kw::defined_import>()?;
            witx::CallMode::DefinedImport
        } else if l.peek::<kw::defined_export>() {
            parser.parse::<kw::defined_export>()?;
            witx::CallMode::DefinedExport
        } else {
            return Err(l.error());
        }))
    }
}

impl Peek for CallMode {
    fn peek(cursor: Cursor<'_>) -> bool {
        kw::declared_import::peek(cursor)
            || kw::declared_export::peek(cursor)
            || kw::defined_import::peek(cursor)
            || kw::defined_export::peek(cursor)
    }

    fn display() -> &'static str {
        "call mode"
    }
}
