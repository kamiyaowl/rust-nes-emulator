use super::interface::SystemBus;

const SIZE: usize = 0x0800;

pub struct WorkRam {
    ram: [u8; SIZE],
}

impl SystemBus for WorkRam {
    fn read_u8(&self, addr: usize) -> u8 {
        assert!(addr < self.ram.len());
        return self.ram[addr];
    }
    fn write_u8(&mut self, addr: usize, data: u8) {
        assert!(addr < self.ram.len());
        self.ram[addr] = data;
    }
}