#[macro_use]
extern crate log;
extern crate env_logger;
extern crate infuse;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    // Set up logging
    let mut builder = env_logger::Builder::new();
    builder.filter_level(log::LevelFilter::Trace);

    if env::var("RUST_LOG").is_ok() {
        builder.parse_filters(&env::var("RUST_LOG").unwrap());
    }

    // Initialise both logging and the infuse library
    builder.init();
    infuse::init();

    // Delegate all input files to the infuse library
    let arguments = env::args().skip(1);
    debug!("[args] {} file(s)", arguments.len());
    for argument in arguments {
        debug!("[processing] file name = {}", &argument);
        let file = File::open(&argument)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents: Vec<u8> = vec![];
        buf_reader.read_to_end(&mut contents)?;
        debug!("[processing] reading \"{}\" ended", &argument);

        infuse::process_file_data(contents);
    }

    Ok(())
}
