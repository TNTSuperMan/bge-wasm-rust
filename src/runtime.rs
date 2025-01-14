use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Runtime {
    rom: Vec<u8>,
    ram: [u8; 0x6000],
    stack: Vec<u8>,
    callstack: Vec<u16>,
    pc: u16
}
#[wasm_bindgen]
impl Runtime {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: Vec<u8>) -> Runtime{
        return Runtime {
            rom: rom,
            ram: [0; 0x6000],
            stack: Vec::new(),
            callstack: Vec::new(),
            pc: 0
        }
    }
}

