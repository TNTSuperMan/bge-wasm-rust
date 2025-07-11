mod bgeimage;
use bgeimage::bin2img;
use wasm_bindgen::prelude::*;

mod memory;
use memory::Memory;

mod framestate;
use framestate::FrameState;

mod wav;
use crate::wav::bin2wav;

#[wasm_bindgen]
pub fn init_panic_hook(){
    console_error_panic_hook::set_once();
}
#[wasm_bindgen]
pub struct Runtime {
    memory: Memory,
    stack: Vec<u8>,
    callstack: Vec<u16>,
    pc: u16,
    ccc: u64,

    do_subframe: bool,
    keystate: u8,
    framestate: FrameState,
    savedata: Vec<u8>
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
            ccc: 0,

            do_subframe: do_subframe,
            keystate: 0,
            framestate: FrameState::new(),
            savedata: Vec::new()
        }
    }
    pub fn emulate_frame(&mut self) -> String{
        for _emucount in 0..1000000 {
            let result = self.emulate();
            match result {
                Ok(r) => {
                    if !r {
                        return String::from("");
                    }
                },
                Err(e) => { return e; }
            }
        }
        String::from("")
    }
    pub fn emulate_one(&mut self) -> String{
        let result = self.emulate();
        match result {
            Ok(_) => String::from(""),
            Err(e) => e
        }
    }
    pub fn load(&self, addr: u16) -> u8{ self.memory.load(addr) }
    pub fn store(&mut self, addr: u16, val: u8){ self.memory.store(addr, val) }
    pub fn set_key_state(&mut self, state: u8){ self.keystate = state }
    pub fn set_save(&mut self, save: Vec<u8>){ self.savedata = save }
    pub fn get_save(&self) -> Vec<u8>{ self.savedata.as_slice().to_vec() }
    pub fn frame_state(&mut self) -> FrameState{
        match self.framestate.do_redraw() {
            true  => self.framestate.pop(),
            false => self.framestate.clone()
        }
    }
    pub fn get_pc(&self) -> u16 { self.pc }
    pub fn get_ccc(&self) -> u64 { self.ccc }
    pub fn get_stack(&self) -> Vec<u8> { self.stack.as_slice().to_vec() }
    pub fn get_callstack(&self) -> Vec<u16> { self.callstack.as_slice().to_vec() }

    fn push(&mut self, val: u8){ self.stack.push(val) }
    fn pop(&mut self) -> Result<u8, String>{
        match self.stack.len() {
            0 => Err(String::from("Stack underflow")),
            _ => Ok(self.stack.pop().unwrap())
        }
    }
    fn pop_addr(&mut self) -> Result<u16, String>{
        let bottom = self.pop()? as u16;
        let top = self.pop()? as u16;
        Ok(bottom | (top << 8))
    }
    fn clear_io(&mut self){
        for i in 0xf000..0xffff {
            self.memory.store(i, 0);
        }
    }
    fn emulate(&mut self) -> Result<bool, String>{
        self.ccc += 1;
        match self.load(self.pc) {
            0x00 => {},
            0x01 => {
                self.pc += 1;
                self.push(self.load(self.pc));
            },
            0x02 => {
                self.pop()?;
            },
            0x03 => {
                self.stack.clear();
            },
            0x04 => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push(v1 + v2)
            },
            0x05 => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push(v1 - v2)
            },
            0x06 => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push(v1 * v2)
            },
            0x07 => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push(v1 / v2)
            },
            0x08 => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push(v1 % v2)
            },
            0x09 => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push(!(v1 & v2))
            },
            0x0a => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push((v1 == v2) as u8);
            },
            0x0b => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push((v1 > v2) as u8);
            },
            0x0c => {
                let addr = self.pop_addr()?;
                if self.pop()? != 0 {
                    self.pc = addr;
                    return Ok(true);
                }
            },
            0x0d => {
                self.pc = self.pop_addr()?;
                return Ok(true);
            },
            0x0e => {
                self.callstack.push(self.pc);
                self.pc = self.pop_addr()?;
                return Ok(true);
            },
            0x0f => {
                let addr = self.callstack.pop();
                
                match addr {
                    Some(to) => {
                        self.pc = to;
                    },
                    None => {
                        return Err(String::from("Callstack underflow"));
                    }
                }
            },
            0x10 => {
                let addr = self.pop_addr()?;
                self.push(self.load(addr))
            },
            0x11 => {
                let addr = self.pop_addr()?;
                let val = self.pop()?;
                self.store(addr, val);
            },
            0x12 => {
                self.push(self.keystate);
            },
            0x13 => {
                self.framestate.redraw();
                self.pc += 1;
                return Ok(false);
            },
            0x14 => {
                let c = self.pop()?;
                let h = self.pop()?;
                let w = self.pop()?;
                let y = self.pop()?;
                let x = self.pop()?;
                self.framestate.push_rect(x,y,w,h,c);
            },
            0x15 => {
                let y = self.pop()?;
                let x = self.pop()?;
                let id= self.pop()?;
                self.framestate.push_graph(x, y, id);
            },
            0x16 => {
                let id = self.pop()?;
                self.framestate.push_sound(id);
            },
            0x17 => {
                self.framestate.stop_sound();
            },
            0x18 => {
                let mode = self.pop()?;
                match mode {
                    0 => {
                        self.framestate.set_img(bin2img(self.memory.get_io())?);
                    },
                    1 => {
                        self.framestate.set_wav(bin2wav(self.memory.get_io())?);
                    },
                    2 => {
                        self.clear_io();
                        for i in 0..self.savedata.len() {
                            if i > 0xFFF {
                                break;
                            }
                            self.memory.store((i + 0x5000) as u16, self.savedata[i]);
                        }
                    },
                    3 => {
                        self.framestate._do_save = true;
                    }
                    4 => { self.clear_io(); }
                    _ => {}
                }
            },
            0x19 => {
                self.pc += 1;
                return Err(String::from("break"));
            },
            _ => {}
        }
        self.pc += 1;
        Ok(!self.do_subframe || self.load(self.pc) != 0x12)
    }
}

