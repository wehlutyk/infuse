#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate lopdf;
#[macro_use]
extern crate log;

pub mod logging;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use std::panic;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use lopdf::Document;

#[wasm_bindgen]
pub fn init() {
    let prefix;

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        log::set_logger(&logging::WASM_LOGGER)
            .map(|()| log::set_max_level(log::LevelFilter::Trace)).unwrap();
        prefix = "Wasm";
    }
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    {
        prefix = "Non-wasm";
    }
    info!("{} infuse initialised", &prefix);
}

#[wasm_bindgen]
pub fn process_file_data(data: Vec<u8>) {
    info!("Processing file (size = {}b)", data.len());

    let cursor = Cursor::new(data);
    let doc = Document::load_from(cursor).unwrap();

    info!("PDF version: {}", doc.version);
    info!("#pages: {}", doc.get_pages().len());
}
