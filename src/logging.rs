#![cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]

use log::{Level, Record, Metadata, Log};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_error(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn log_warn(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    fn log_info(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    fn log_debug(s: &str);
}

impl Log for WasmConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_type = match record.level() {
                Level::Error => log_error,
                Level::Warn => log_warn,
                Level::Info => log_info,
                Level::Debug => log_debug,
                Level::Trace => log_debug,
            };
            log_type(&format!("{}: {}: {}", record.level(),
                              record.module_path().unwrap_or("<no module>"), record.args()));
        }
    }

    fn flush(&self) {}
}

pub struct WasmConsoleLogger;

pub static WASM_LOGGER: WasmConsoleLogger = WasmConsoleLogger;
