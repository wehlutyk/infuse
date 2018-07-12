#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate lopdf;
#[macro_use]
extern crate log;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use std::panic;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use lopdf::Document;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_error(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn log_warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    fn log_info(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    fn log_debug(s: &str);
}

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
struct WasmConsoleLogger;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
impl log::Log for WasmConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let log_type = match record.level() {
                log::Level::Error => log_error,
                log::Level::Warn => log_warn,
                log::Level::Info => log_info,
                log::Level::Debug => log_debug,
                log::Level::Trace => log_debug,
            };
            log_type(&format!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
static WASM_LOGGER: WasmConsoleLogger = WasmConsoleLogger;

#[wasm_bindgen]
pub fn init() {
    let prefix;

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        log::set_logger(&WASM_LOGGER)
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
