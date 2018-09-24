#![cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]

use log::{Level, Log, Metadata, Record};
use wasm_bindgen::prelude::*;

// Use web-sys once it's published (rustwasm/wasm-bindgen#613)
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
}

impl Log for WasmConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_fn = match record.level() {
                Level::Error => error,
                Level::Warn => warn,
                Level::Info => info,
                Level::Debug => debug,
                Level::Trace => debug,
            };
            log_fn(&format!(
                "{}: {}: {}",
                record.level(),
                record.module_path().unwrap_or("<no module>"),
                record.args()
            ));
        }
    }

    fn flush(&self) {}
}

pub struct WasmConsoleLogger;

pub static WASM_LOGGER: WasmConsoleLogger = WasmConsoleLogger;
