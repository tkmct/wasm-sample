extern crate plasma_client;
extern crate wasm_bindgen;
extern crate web_sys;

pub mod browser_storage;

use browser_storage::BrowserStorage;
use plasma_client::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/js/storage.js")]
extern "C" {
    type LocalStorage;

    #[wasm_bindgen(constructor)]
    fn new() -> LocalStorage;

    #[wasm_bindgen(method)]
    fn get(this: &LocalStorage, k: &str) -> String;
    #[wasm_bindgen(method)]
    fn set(this: &LocalStorage, k: &str, v: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    if let Some(storage) = BrowserStorage::new("plasmaDB") {
        storage.set("hello", "world");
        if let Ok(Some(w)) = storage.get("hello") {
            log(&w);
        };
    };

    Ok(())
}
