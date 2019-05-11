extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::IdbFactory;
use web_sys::IdbDatabase;
use web_sys::IdbObjectStore;
use web_sys::IdbRequest;
use web_sys::IdbOpenDbRequest;
use web_sys::IdbCursor;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

trait Storage {
    fn get(&self, k: String) -> String;
    fn set(&self, k: String, v: String) -> String;
}


struct BrowserStorage {
    db: IdbDatabase
}

impl BrowserStorage {
    fn new(db: IdbDatabase) -> BrowserStorage {
        BrowserStorage { db: db, }
    }
}

impl Storage for BrowserStorage {
    fn get(&self, k: String) -> String {
        k
    }

    fn set(&self, k: String, v: String) -> String {
        v
    }
}



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
    set_panic_hook();

    let factory = IdbFactory::from();
    let open_request = IdbFactory::open("storage", 1).unwrap();
    open_request.set_onsuccess(|| {
        let db = open_request.result();
    });


    Ok(())
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
