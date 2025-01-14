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
    pub fn emulate_frame(&mut self){
        while self.emulate() {}
    }
    pub fn load(&self, addr: u16) -> u8{
        if addr < 0xa000 {
            if (addr as usize) < self.rom.len() {
                return self.rom[addr as usize];
            } else {
                return 0;
            }
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
    fn emulate(&mut self) -> bool{
        match self.load(self.pc) {
            0x00 => {},
            0x01 => {
                self.pc += 1;
                self.push(self.load(self.pc));
            },
            0x02 => {
                self.pop();
            },
            0x03 => {
                self.stack.clear();
            },
            0x04 => {
                let v1 = self.pop();
                let v2 = self.pop();
                self.push(v1 + v2)
            },
            0x05 => {
                let v1 = self.pop();
                let v2 = self.pop();
                self.push(v1 - v2)
            },
            0x06 => {
                let v1 = self.pop();
                let v2 = self.pop();
                self.push(v1 * v2)
            },
            0x07 => {
                let v1 = self.pop();
                let v2 = self.pop();
                self.push(v1 / v2)
            },
            0x08 => {
                let v1 = self.pop();
                let v2 = self.pop();
                self.push(v1 % v2)
            },
            0x09 => {
                let v1 = self.pop();
                let v2 = self.pop();
                self.push(!(v1 & v2))
            }
            _ => {}
        }
        return true;
    }
}

