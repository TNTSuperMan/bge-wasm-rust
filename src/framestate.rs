use crate::bgeimage::toimg::Bin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Display {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: bool,

    pub is_graph: bool,
    pub gid: u8
}
impl Clone for Display {
    fn clone(&self) -> Display {
        Display {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
            is_graph: self.is_graph,
            gid: self.gid
        }
    }
}
#[wasm_bindgen]
pub struct FrameState {
    disps: Vec<Display>,
    sound:Vec<u8>,
    imgs: Vec<Bin>,
    wavs: Vec<Bin>,
    pub _do_redraw: bool,
    pub _do_updimg: bool,
    pub _do_updwav: bool,
    pub stopsound: bool,
    pub _do_save: bool
}
#[wasm_bindgen]
impl FrameState {
    pub fn new() -> FrameState{
        FrameState {
            disps: Vec::new(),
            imgs: Vec::new(),
            sound:Vec::new(),
            wavs: Vec::new(),
            _do_redraw: false,
            _do_updimg: false,
            _do_updwav: false,
            stopsound: false,
            _do_save: false
        }
    }
    pub fn clone(&self) -> FrameState{
        FrameState {
            disps:  self.disps.as_slice().to_vec(),
            sound: self.sound.as_slice().to_vec(),
            imgs:  self.imgs.as_slice().to_vec(),
            wavs: self.wavs.as_slice().to_vec(),
            _do_redraw: self._do_redraw,
            _do_updimg: self._do_updimg,
            _do_updwav: self._do_updwav,
            stopsound: self.stopsound,
            _do_save: false
        }
    }
    pub fn pop(&mut self) -> FrameState{
        let clone = self.clone();

        self.disps = Vec::new();
        self.imgs = Vec::new();
        self.sound= Vec::new();
        self._do_redraw = false;
        self._do_updimg = false;
        self.stopsound = false;
        self._do_save = false;

        return clone;
    }
    pub fn redraw(&mut self){
        self._do_redraw = true;
    }
    pub fn do_redraw(&self) -> bool{
        return self._do_redraw;
    }
    pub fn push_rect(&mut self, x: u8, y: u8, w: u8, h: u8, c: u8){
        let a = (c & 0b11000000) >> 6;
        let r = (c & 0b110000) >> 4;
        let g = (c & 0b001100) >> 2;
        let b = (c & 0b000011) >> 0;
        self.disps.push(Display {
            x: x,
            y: y,
            w: w,
            h: h,
            r: r * 85,
            g: g * 85,
            b: b * 85,
            a: a == 0,
            is_graph: false,
            gid: 0
        })
    }
    pub fn push_graph(&mut self, x: u8, y: u8, id: u8){
        self.disps.push(Display {
            x: x,
            y: y,
            w: 0, h: 0, r: 0, g: 0, b: 0, a: true,
            is_graph: true,
            gid:id
        })
    }
    pub fn push_sound(&mut self, id: u8){
        self.sound.push(id);
    }
    pub fn stop_sound(&mut self){
        self.stopsound = true;
    }
    pub fn set_img(&mut self, imgs: Vec<Bin>){
        self._do_updimg = true;
        self.imgs = imgs;
    }
    pub fn set_wav(&mut self, wavs: Vec<Bin>){
        self._do_updwav = true;
        self.wavs = wavs;
    }
    pub fn get_disps(&self)-> Vec<Display>  { self.disps.as_slice().to_vec() }
    pub fn get_sound(&self)-> Vec<u8>       { self.sound.as_slice().to_vec() }
    pub fn get_imgs(&self) -> Vec<Bin>      { self.imgs .as_slice().to_vec() }
    pub fn get_wavs(&self) -> Vec<Bin>      { self.wavs .as_slice().to_vec() }
}
