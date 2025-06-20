pub struct Memory {
    rom: Vec<u8>,
    ram: [u8; 0x6000]
}
impl Memory {
    pub fn new(rom: Vec<u8>) -> Memory{
        Memory {
            rom: rom,
            ram: [0; 0x6000]
        }
    }
    pub fn load(&self, addr: u16) -> u8{
        if addr < 0xa000 {
            if (addr as usize) < self.rom.len() {
                self.rom[addr as usize]
            } else {
                0
            }
        } else {
            self.ram[(addr - 0xa000) as usize]
        }
    }
    pub fn store(&mut self, addr: u16, val: u8){
        if addr >= 0xa000 {
            self.ram[(addr - 0xa000) as usize] = val;
        }
    }
    pub fn get_io(&self) -> &[u8]{ &self.ram[0x5000..0x6000] }
}
