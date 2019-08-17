use super::interface::{SystemBus, EmulateControl};

pub const SIZE: usize = 0x2000;

pub struct ExtendedRam {
    pub ram: [u8; SIZE],
}

impl SystemBus for ExtendedRam {
    fn read_u8(&self, addr: u16) -> u8 {
        debug_assert!((addr as usize) < self.ram.len());
        self.ram[addr as usize]
    }
    fn write_u8(&mut self, addr: u16, data: u8) {
        debug_assert!((addr as usize) < self.ram.len());
        self.ram[addr as usize] = data;
    }
}

impl EmulateControl for ExtendedRam {
    fn reset(&mut self){
        self.ram = [0; SIZE];
    }
    fn store(&self, read_callback: impl Fn(usize, u8)){
        for i in 0..self.ram.len() {
            read_callback(i, self.ram[i]);
        }
    }
    fn restore(&mut self, write_callback: impl Fn(usize) -> u8){
        for i in 0..self.ram.len() {
            let data = write_callback(i);
             self.ram[i] = data;
        }
    }
}