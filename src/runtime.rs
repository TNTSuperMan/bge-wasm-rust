use wasm_bindgen::prelude::*;

mod memory;
use memory::Memory;

mod framestate;
use framestate::FrameState;

#[wasm_bindgen]
pub struct Runtime {
    memory: Memory,
    stack: Vec<u8>,
    callstack: Vec<u16>,
    pc: u16,

    do_subframe: bool,
    keystate: u8,
    framestate: FrameState
}
#[wasm_bindgen]
impl Runtime {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: Vec<u8>, do_subframe: bool) -> Runtime{
        Runtime {
            memory: Memory::new(rom),
            stack: Vec::new(),
            callstack: Vec::new(),
            pc: 0,

            do_subframe: do_subframe,
            keystate: 0,
            framestate: FrameState::new()
        }
    }
    pub fn emulate_frame(&mut self){ while self.emulate() {} }
    pub fn load(&self, addr: u16) -> u8{ self.memory.load(addr) }
    pub fn store(&mut self, addr: u16, val: u8){ self.memory.store(addr, val) }
    pub fn set_key_state(&mut self, state: u8){ self.keystate = state }
    pub fn frame_state(&mut self) -> FrameState{
        if self.framestate.do_redraw() {
            return self.framestate.pop();
        } else {
            return self.framestate.clone();
        }
    }

    fn push(&mut self, val: u8){ self.stack.push(val) }
    fn pop(&mut self) -> u8{ self.stack.pop().expect("stack underflow") }
    fn pop_addr(&mut self) -> u16{
        let bottom = self.pop() as u16;
        let top = self.pop() as u16;
        return bottom | (top << 8);
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
            },
            0x0a => {
                let v1 = self.pop();
                let v2 = self.pop();
                self.push((v1 > v2) as u8);
            },
            0x0b => {
                let v1 = self.pop();
                let v2 = self.pop();
                self.push((v1 == v2) as u8);
            },
            0x0c => {
                let addr = self.pop_addr();
                if self.pop() != 0 {
                    self.pc = addr;
                    return true;
                }
            },
            0x0d => {
                self.pc = self.pop_addr();
                return true;
            },
            0x0e => {
                let addr = self.pop_addr();
                self.callstack.push(self.pc);
                self.pc = addr;
                return true;
            },
            0x0f => {
                self.pc = self.callstack.pop().expect("Callstack underflow");
            },
            0x10 => {
                let addr = self.pop_addr();
                self.push(self.load(addr))
            },
            0x11 => {
                let addr = self.pop_addr();
                let val = self.pop();
                self.store(addr, val);
            },
            0x12 => {
                self.push(self.keystate);
            },
            _ => {}
        }
        self.pc += 1;
        return !self.do_subframe || self.load(self.pc) != 0x12;
    }
}

