use super::system::MemoryIo;

const SIZE: usize = 0x8000;

pub struct ProgramRom {
    rom: [u8; SIZE],
}

impl MemoryIo for ProgramRom {
    fn read_u8(&self, addr: usize) -> u8 {
        assert!(addr < self.rom.len());
        return self.rom[addr];
    }
    fn write_u8(&mut self, addr: usize, data: u8) {
        panic!("Memory Write Request Error. PRG-ROM. addr:{:x}, data:{:x}", addr, data);
    }
}