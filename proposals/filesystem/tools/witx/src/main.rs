use clap::{App, Arg, ArgMatches, SubCommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use witx::{load, phases, Document, Documentation};

pub fn main() {
    let app = App::new("witx")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Validate and process witx files")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("docs")
                .about("Output documentation")
                .arg(
                    Arg::with_name("input")
                        .required(true)
                        .multiple(true)
                        .help("path to root of witx document"),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("polyfill")
                .about("Examine differences between interfaces")
                .arg(
                    Arg::with_name("input")
                        .required(true)
                        .multiple(true)
                        .help("path to root of witx document"),
                )
                .arg(
                    Arg::with_name("older_interface")
                        .required(true)
                        .multiple(true)
                        .help("path to root of witx document describing interface to polyfill"),
                )
                .arg(
                    Arg::with_name("module_mapping")
                        .short("m")
                        .long("module_mapping")
                        .required(false)
                        .takes_value(true)
                        .multiple(true)
                        .help("module to examine. Use newname=oldname syntax if name is different between new and old interfaces"),
                ),
        )
        .subcommand(
            SubCommand::with_name("repo-docs")
                .about("Update documentation in WASI repository to reflect witx specs")
        )
        .get_matches();

    let load_witx = {
        |args: &ArgMatches, field: &str| -> Document {
            let inputs = args
                .values_of(field)
                .expect(&format!("required argument: {}", field))
                .collect::<Vec<_>>();
            match load(&inputs) {
                Ok(doc) => {
                    if app.is_present("verbose") {
                        println!("{}: {:?}", field, doc)
                    }
                    doc
                }
                Err(e) => {
                    println!("{}", e.report());
                    if app.is_present("verbose") {
                        println!("{:?}", e);
                    }
                    process::exit(1)
                }
            }
        }
    };

    pretty_env_logger::init();

    if let Some(docs_args) = app.subcommand_matches("docs") {
        let doc = load_witx(&docs_args, "input");
        if let Some(output) = docs_args.value_of("output") {
            write_docs(&doc, output)
        } else {
            println!("{}", doc.to_md())
        }
    } else if let Some(polyfill_args) = app.subcommand_matches("polyfill") {
        let doc = load_witx(&polyfill_args, "input");
        let older_doc = load_witx(&polyfill_args, "older_interface");

        let module_mapping_args = polyfill_args
            .values_of("module_mapping")
            .expect("at least one module_mapping argument required")
            .collect::<Vec<&str>>();
        let module_mapping = parse_module_mapping(&module_mapping_args);

        let polyfill = witx::polyfill::Polyfill::new(&doc, &older_doc, &module_mapping)
            .expect("calculate polyfill");
        println!("{}", polyfill.to_md());
        if app.is_present("verbose") {
            println!("{:?}", polyfill);
        }
    } else if app.subcommand_matches("repo-docs").is_some() {
        for phase in &[
            phases::snapshot().unwrap(),
            phases::ephemeral().unwrap(),
            phases::old::snapshot_0().unwrap(),
        ] {
            let doc = load(&phase).expect("parse phase");
            let path = phase
                .get(0)
                .expect("at least one path")
                .parent()
                .expect("drop file")
                .join("../docs.md");
            write_docs(&doc, &path);
        }
    }
}

fn write_docs<P: AsRef<Path>>(document: &Document, path: P) {
    let mut file = File::create(path.as_ref()).expect("create output file");
    file.write_all(document.to_md().as_bytes())
        .expect("write output file");
}

fn parse_module_mapping(ms: &[&str]) -> HashMap<String, String> {
    let mut o = HashMap::new();
    for m in ms {
        let s = m.split('=').collect::<Vec<&str>>();
        if s.len() == 1 {
            let mname = s.get(0).unwrap();
            o.insert(mname.to_string(), mname.to_string());
        } else if s.len() == 2 {
            let newname = s.get(0).unwrap();
            let oldname = s.get(1).unwrap();
            o.insert(newname.to_string(), oldname.to_string());
        } else {
            panic!("invalid module mapping: '{}'", m)
        }
    }
    o
}
