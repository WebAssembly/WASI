use clap::{App, Arg};
use std::path::Path;
use witx_frontend::load;
use std::process;

pub fn main() {
    let app = App::new("witx-validate")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Validate witx file format")
        .arg(
            Arg::with_name("input")
                .required(true)
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

    match load(Path::new(app.value_of("input").expect("required arg"))) {
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
