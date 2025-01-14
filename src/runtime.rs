use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Runtime {
    rom: [u8; 0xa000],
    ram: [u8; 0x6000],
    stack: Vec<u8>,
    callstack: Vec<u16>,
    pc: u16
}
#[wasm_bindgen]
impl Runtime {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: &[u8]) -> Runtime{
        let mut arr_rom: [u8; 0xa000] = [0; 0xa000];
        for (i, &item) in rom.iter().enumerate() {
            arr_rom[i] = item;
        }
        return Runtime {
            rom: arr_rom,
            ram: [0; 0x6000],
            stack: Vec::new(),
            callstack: Vec::new(),
            pc: 0
        }
    }
}

