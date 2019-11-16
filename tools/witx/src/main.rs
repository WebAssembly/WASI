use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use witx::{load, to_c_header, Documentation};

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
            SubCommand::with_name("c_header")
                .about("Output C header file")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .get_matches();

    let inputs = app
        .values_of("input")
        .expect("at least one input required")
        .into_iter()
        .map(|i| PathBuf::from(i))
        .collect::<Vec<PathBuf>>();

    match load(&inputs) {
        Ok(doc) => {
            if app.is_present("verbose") {
                println!("{:?}", doc)
            }

            if let Some(subcommand) = app.subcommand_matches("docs") {
                let md = doc.to_md();
                if let Some(output) = subcommand.value_of("output") {
                    let mut file = File::create(output).expect("create output file");
                    file.write_all(md.as_bytes()).expect("write output file");
                } else {
                    println!("{}", md)
                }
            } else if let Some(subcommand) = app.subcommand_matches("c_header") {
                let c_header = to_c_header(&doc);
                if let Some(output) = subcommand.value_of("output") {
                    let mut file = File::create(output).expect("create output file");
                    file.write_all(c_header.as_bytes())
                        .expect("write output file");
                } else {
                    println!("{}", c_header)
                }
            }
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
