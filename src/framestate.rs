use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Rectangle {
    x: u8,
    y: u8,
    w: u8,
    h: u8,
    r: u8,
    g: u8,
    b: u8
}
impl Clone for Rectangle {
    fn clone(&self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            r: self.r,
            g: self.g,
            b: self.b
        }
    }
}
#[wasm_bindgen]
pub struct Graphic {
    x: u8,
    y: u8,
    id:u8
}
impl Clone for Graphic {
    fn clone(&self) -> Graphic {
        Graphic {
            x: self.x,
            y: self.y,
            id:self.id
        }
    }
}
#[wasm_bindgen]
pub struct FrameState {
    rect: Vec<Rectangle>,
    graph:Vec<Graphic>,
    sound:Vec<u8>,
    imgs: Vec<String>,
    _do_redraw: bool,
    _do_updimg: bool,
    stopsound: bool
}
impl FrameState {
    pub fn new() -> FrameState{
        FrameState {
            rect: Vec::new(),
            graph:Vec::new(),
            imgs: Vec::new(),
            sound:Vec::new(),
            _do_redraw: false,
            _do_updimg: false,
            stopsound: false
        }
    }
    pub fn clone(&self) -> FrameState{
        FrameState {
            rect:  self.rect.as_slice().to_vec(),
            graph: self.graph.as_slice().to_vec(),
            imgs:  self.imgs.as_slice().to_vec(),
            sound: self.sound.as_slice().to_vec(),
            _do_redraw: self._do_redraw,
            _do_updimg: self._do_updimg,
            stopsound: self.stopsound
        }
    }
    pub fn pop(&mut self) -> FrameState{
        let clone = self.clone();

        self.rect = Vec::new();
        self.graph= Vec::new();
        self.sound= Vec::new();
        self._do_redraw = false;
        self.stopsound = false;

        return clone;
    }
    pub fn redraw(&mut self){
        self._do_redraw = true;
    }
    pub fn do_redraw(&self) -> bool{
        return self._do_redraw;
    }
    pub fn push_rect(&mut self, x: u8, y: u8, w: u8, h: u8, c: u8){
        let r = (c & 0b110000) >> 4;
        let g = (c & 0b001100) >> 2;
        let b = (c & 0b000011) >> 0;
        self.rect.push(Rectangle {
            x: x,
            y: y,
            w: w,
            h: h,
            r: r,
            g: g,
            b: b
        })
    }
    pub fn push_graph(&mut self, x: u8, y: u8, id: u8){
        self.graph.push(Graphic {
            x: x,
            y: y,
            id:id
        })
    }
    pub fn push_sound(&mut self, id: u8){
        self.sound.push(id);
    }
    pub fn stop_sound(&mut self){
        self.stopsound = true;
    }
    pub fn set_img(&mut self, imgs: Vec<String>){
        self._do_updimg = true;
        self.imgs = imgs;
    }
}
