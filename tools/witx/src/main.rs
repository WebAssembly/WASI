use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process;
use witx::{load, Documentation};

pub fn main() {
    let app = App::new("witx")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Validate and process witx files")
        .arg(
            Arg::with_name("input")
                .required(true)
                .multiple(true)
                .help("path to root of witx document"),
        )
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
        .get_matches();

    let inputs = app
        .values_of("input")
        .expect("at least one input required")
        .collect::<Vec<&str>>();

    let load_docs = {
        |inputs: &[&str]| match load(inputs) {
            Ok(doc) => doc,
            Err(e) => {
                println!("{}", e.report());
                if app.is_present("verbose") {
                    println!("{:?}", e);
                }
                process::exit(1)
            }
        }
    };

    pretty_env_logger::init();

    let doc = load_docs(&inputs);
    if app.is_present("verbose") {
        println!("{:?}", doc)
    }

    if let Some(docs_command) = app.subcommand_matches("docs") {
        let md = doc.to_md();
        if let Some(output) = docs_command.value_of("output") {
            let mut file = File::create(output).expect("create output file");
            file.write_all(md.as_bytes()).expect("write output file");
        } else {
            println!("{}", md)
        }
    } else if let Some(polyfill_command) = app.subcommand_matches("polyfill") {
        let older_inputs = polyfill_command
            .values_of("older_interface")
            .expect("at least one older_interface argument required")
            .collect::<Vec<&str>>();
        let older_doc = load_docs(&older_inputs);

        let module_mapping_args = polyfill_command
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
    }
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
