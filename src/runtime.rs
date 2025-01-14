pub struct Runtime {
    rom: [u8; 0xa000],
    ram: [u8; 0x6000],
    stack: Vec<u8>,
    callstack: Vec<u16>,
    pc: u16
}
impl Runtime {
    pub fn new(rom: [u8; 0xa000]) -> Runtime{
        return Runtime {
            rom: rom,
            ram: [0; 0x6000],
            stack: Vec::new(),
            callstack: Vec::new(),
            pc: 0
        }
    }
}

