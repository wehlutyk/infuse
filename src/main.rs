extern crate infuse;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    infuse::init();

    let arguments = env::args().skip(1);
    println!("[args] {} file(s)", arguments.len());
    for argument in arguments {
        println!("[processing] file name = {}", &argument);
        let file = File::open(&argument)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents: Vec<u8> = vec![];
        buf_reader.read_to_end(&mut contents)?;
        println!("[processing] reading \"{}\" ended", &argument);

        infuse::process_file_data(contents);
    }

    Ok(())
}
