use wasm_bindgen::prelude::*;

mod runtime;
use runtime::Runtime;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main(rom: &[u8]) -> Runtime {
    let mut arr_rom: [u8; 0xa000] = [0; 0xa000];
    for (i, &item) in rom.iter().enumerate() {
        arr_rom[i] = item;
    }
    return Runtime::new(arr_rom);
}