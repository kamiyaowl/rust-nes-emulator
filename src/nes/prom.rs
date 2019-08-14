use super::interface::{SystemBus, EmulateControl};

pub const SIZE: usize = 0x8000;

pub struct ProgramRom {
    pub rom: [u8; SIZE],
}

impl SystemBus for ProgramRom {
    fn read_u8(&self, addr: usize) -> u8 {
        assert!(addr < self.rom.len());
        return self.rom[addr];
    }
    fn write_u8(&mut self, addr: usize, data: u8) {
        panic!("Memory Write Request Error. PRG-ROM. addr:{:x}, data:{:x}", addr, data);
    }
}