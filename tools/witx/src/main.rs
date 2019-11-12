use clap::{App, Arg};
use std::path::PathBuf;
use std::process;
use witx::load;

pub fn main() {
    let app = App::new("witx")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Validate witx file format")
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
