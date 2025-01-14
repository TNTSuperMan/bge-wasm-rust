use wasm_bindgen::prelude::*;

mod runtime;
use runtime::Runtime;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main(rom: Vec<u8>) -> Runtime {
    return Runtime::new(rom);
}