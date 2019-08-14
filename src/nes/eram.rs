use super::system::MemoryIo;

const SIZE: usize = 0x2000;

pub struct ExtendedRam {
    ram: [u8; SIZE],
}

impl MemoryIo for ExtendedRam {
    fn read_u8(&self, addr: usize) -> u8 {
        assert!(addr < self.ram.len());
        return self.ram[addr];
    }
    fn write_u8(&mut self, addr: usize, data: u8) {
        assert!(addr < self.ram.len());
        self.ram[addr] = data;
    }
}