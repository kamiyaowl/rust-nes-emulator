use super::interface::{SystemBus, EmulateControl};

pub const SIZE: usize = 0x0800;

pub struct VideoRam {
    pub ram: [u8; SIZE],
}

impl SystemBus for VideoRam {
    fn read_u8(&self, addr: usize) -> u8 {
        assert!(addr < self.ram.len());
        self.ram[addr]
    }
    fn write_u8(&mut self, addr: usize, data: u8) {
        assert!(addr < self.ram.len());
        self.ram[addr] = data;
    }
}

impl EmulateControl for VideoRam {
    fn reset(&mut self){
        self.ram = [0; SIZE];
    }
    fn store(&self, read_callback: fn(usize, u8)){
        for i in 0..self.ram.len() {
            read_callback(i, self.ram[i]);
        }
    }
    fn restore(&mut self, write_callback: fn(usize) -> u8){
        for i in 0..self.ram.len() {
            let data = write_callback(i);
             self.ram[i] = data;
        }
    }
}