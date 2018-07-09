#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate lopdf;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use std::panic;
use std::io::Cursor;

use wasm_bindgen::prelude::*;

use lopdf::Document;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
fn log(s: &str) {
    println!("{}", s);
}

#[wasm_bindgen]
pub fn init() {
    let prefix;

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        prefix = "Wasm";
    }
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    {
        prefix = "Non-wasm";
    }
    log(&format!("{} infuse initialised", &prefix));
}

#[wasm_bindgen]
pub fn process_file_data(data: Vec<u8>) {
    log(&format!("Processing file (size = {}b)", data.len()));

    let cursor = Cursor::new(data);
    let doc = Document::load_from(cursor).unwrap();

    log(&format!("PDF version: {}", doc.version));
    log(&format!("#pages: {}", doc.get_pages().len()));
}
