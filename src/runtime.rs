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
        Runtime {
            rom: rom,
            ram: [0; 0x6000],
            stack: Vec::new(),
            callstack: Vec::new(),
            pc: 0
        }
    }
    pub fn load(&self, addr: u16) -> u8{
        if addr < 0xa000 {
            return self.rom[addr as usize];
        } else {
            return self.ram[(addr - 0xa000) as usize];
        }
    }
    pub fn store(&mut self, addr: u16, val: u8){
        if addr >= 0xa000 {
            self.ram[(addr - 0xa000) as usize] = val;
        }
    }
    fn push(&mut self, val: u8){
        self.stack.push(val)
    }
    fn pop(&mut self) -> u8{
        self.stack.pop().expect("stack underflow")
    }
}

