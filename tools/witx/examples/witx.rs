use anyhow::{anyhow, bail, Result};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use structopt::{clap::AppSettings, StructOpt};
use witx::{load, phases, Document, Documentation};

/// Validate and process witx files
#[derive(StructOpt, Debug)]
#[structopt(
    name = "witx",
    version = env!("CARGO_PKG_VERSION"),
    global_settings = &[
        AppSettings::VersionlessSubcommands,
        AppSettings::ColoredHelp
    ]
)]
struct Args {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Output documentation
    Docs {
        /// Path to root of witx document
        #[structopt(number_of_values = 1, value_name = "INPUT", parse(from_os_str))]
        input: Vec<PathBuf>,
        /// Path to generated documentation in Markdown format
        #[structopt(
            short = "o",
            long = "output",
            value_name = "OUTPUT",
            parse(from_os_str)
        )]
        output: Option<PathBuf>,
    },
    /// Update documentation in WASI repository to reflect witx specs
    RepoDocs,
    /// Examine differences between interfaces
    Polyfill {
        /// Path to root of witx document
        #[structopt(
            required = true,
            number_of_values = 1,
            value_name = "INPUT",
            parse(from_os_str)
        )]
        input: Vec<PathBuf>,
        /// Path to root of witx document describing interface to polyfill
        #[structopt(
            required = true,
            number_of_values = 1,
            value_name = "OLDER_INTERFACE",
            parse(from_os_str)
        )]
        older_interface: Vec<PathBuf>,
        /// Module to examine (use newname=oldname syntax if name is different
        /// between new and old interfaces)
        #[structopt(
            short = "m",
            long = "module_mapping",
            required = true,
            number_of_values = 1,
            value_name = "NEWNAME=OLDNAME",
            parse(try_from_str = parse_module_mapping)
        )]
        module_mapping: Vec<(String, String)>,
    },
}

pub fn main() {
    let args = Args::from_args();
    pretty_env_logger::init();
    let verbose = args.verbose;

    match args.cmd {
        Command::Docs { input, output } => {
            let doc = load_witx(&input, "input", verbose);
            if let Some(output) = output {
                write_docs(&doc, output)
            } else {
                println!("{}", doc.to_md())
            }
        }
        Command::RepoDocs => {
            for phase in &[
                phases::snapshot().unwrap(),
                phases::ephemeral().unwrap(),
                phases::old::snapshot_0().unwrap(),
            ] {
                let doc = load(&phase).expect("parse phase");
                write_docs(&doc, phases::docs_path(&phase));
            }
        }
        Command::Polyfill {
            input,
            older_interface,
            module_mapping,
        } => {
            use std::{collections::HashMap, iter::FromIterator};
            use witx::polyfill::Polyfill;

            let doc = load_witx(&input, "input", verbose);
            let older_doc = load_witx(&older_interface, "older_interface", verbose);
            let module_mapping = HashMap::from_iter(module_mapping.into_iter());
            let polyfill = match Polyfill::new(&doc, &older_doc, &module_mapping) {
                Ok(polyfill) => polyfill,
                Err(e) => {
                    eprintln!("couldn't calculate polyfill");
                    if verbose {
                        println!("{:?}", e);
                    }
                    process::exit(1);
                }
            };
            println!("{}", polyfill.to_md());
            if verbose {
                println!("{:?}", polyfill);
            }
        }
    }
}

fn load_witx(input: &[PathBuf], field_name: &str, verbose: bool) -> Document {
    match load(input) {
        Ok(doc) => {
            if verbose {
                println!("{}: {:?}", field_name, doc);
            }
            doc
        }
        Err(e) => {
            eprintln!("{}", e.report());
            if verbose {
                println!("{:?}", e);
            }
            process::exit(1)
        }
    }
}

fn write_docs<P: AsRef<Path>>(document: &Document, path: P) {
    let mut file = File::create(path.as_ref()).expect("create output file");
    file.write_all(document.to_md().as_bytes())
        .expect("write output file");
}

fn parse_module_mapping(m: &str) -> Result<(String, String)> {
    let s: Vec<_> = m.split('=').collect();
    let (n, o) = match s.len() {
        1 => {
            let mname = s
                .get(0)
                .ok_or(anyhow!("module name cannot be an empty string"))?;
            (mname, mname)
        }
        2 => {
            let newname = s
                .get(0)
                .ok_or(anyhow!("new module name cannot be an empty string"))?;
            let oldname = s
                .get(1)
                .ok_or(anyhow!("old module name cannot be an empty string"))?;
            (newname, oldname)
        }
        _ => bail!("invalid module mapping: '{}'", m),
    };
    Ok((n.to_string(), o.to_string()))
}
