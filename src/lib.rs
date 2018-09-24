extern crate console_error_panic_hook;
extern crate lopdf;
extern crate wasm_bindgen;
#[macro_use]
extern crate log;

pub mod logging;

use lopdf::Document;
use std::io::Cursor;
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    let prefix;

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        log::set_logger(&logging::WASM_LOGGER)
            .map(|()| log::set_max_level(log::LevelFilter::Trace))
            .unwrap();
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
    info!("Trailer: {:#?}", &doc.trailer);
    info!("Objects: {:#?}", &doc.objects);
    info!("Pages: {:#?}", &doc.get_pages());
    info!("Pages contents...");
    for (page_number, object_id) in doc.get_pages() {
        info!("Page {}", &page_number);
        info!("Content streams: {:#?}", &doc.get_page_contents(object_id));
        info!("Contents: {:#?}", &doc.get_page_content(object_id));
        info!("Decoded contents: {:#?}", &doc.get_and_decode_page_content(object_id));
        info!("Resources: {:#?}", &doc.get_page_resources(object_id));
        info!("Fonts: {:#?}", &doc.get_page_fonts(object_id));
    }
}
