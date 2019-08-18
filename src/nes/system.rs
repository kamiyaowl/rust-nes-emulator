use super::interface::{SystemBus, EmulateControl};
use super::cassette::Cassette;

pub const VRAM_SIZE: usize           = 0x0800;
pub const WRAM_SIZE: usize           = 0x0800;
pub const PPU_REG_SIZE: usize        = 0x0008;
pub const APU_AND_IO_REG_SIZE: usize = 0x0018;
pub const EROM_SIZE: usize           = 0x1FE0;
pub const ERAM_SIZE: usize           = 0x2000;
pub const PROM_SIZE: usize           = 0x8000; // 32KB

/// Memory Access Dispatcher
pub struct System {
    /// Video RAM
    pub vram: [u8; VRAM_SIZE],

    /// 0x0000 - 0x07ff: WRAM 
    /// 0x0800 - 0x1f7ff: WRAM  Mirror x3
    pub wram: [u8; WRAM_SIZE],
    //  0x2000 - 0x2007: PPU I/O
    //  0x2008 - 0x3fff: PPU I/O Mirror x1023
    pub ppu_reg: [u8; PPU_REG_SIZE],
    //  0x4000 - 0x401f: APU I/O, PAD
    pub io_reg: [u8; APU_AND_IO_REG_SIZE],

    /// カセットへのR/W要求は呼び出し元でEmulation, 実機を切り替えるようにする
    /// 引数に渡されるaddrは、CPU命令そのままのアドレスを渡す
    ///  0x4020 - 0x5fff: Extended ROM
    ///  0x6000 - 0x7FFF: Extended RAM
    ///  0x8000 - 0xbfff: PRG-ROM switchable
    ///  0xc000 - 0xffff: PRG-ROM fixed to the last bank or switchable
    pub cassette_read:  fn(u16) -> u8,
    pub cassette_write: fn(u16, u8),
}

impl SystemBus for System {
    fn read_u8(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            let index = usize::from(addr) % self.wram.len(); // mirror support
            self.wram[index] 
        } else if addr < 0x4000 {
            let index = usize::from(addr - 0x2000) % self.ppu_reg.len(); // mirror support
            self.ppu_reg[index] 
        } else if addr < 0x4020 {
            let index = usize::from(addr - 0x4000);
            self.io_reg[index] 
        } else {
            (self.cassette_read)(addr)
        }
    }
    fn write_u8(&mut self, addr: u16, data: u8) {
        if addr < 0x2000 {
            let index = usize::from(addr) % self.wram.len(); // mirror support
            self.wram[index] = data;
        } else if addr < 0x4000 {
            let index = usize::from(addr - 0x2000) % self.ppu_reg.len(); // mirror support
            self.ppu_reg[index] = data;
        } else if addr < 0x4020 {
            let index = usize::from(addr - 0x4000);
            self.io_reg[index] = data;
        } else {
            (self.cassette_write)(addr, data);
        }
    }
}

impl EmulateControl for System {
    fn reset(&mut self){
        self.vram    = [0; VRAM_SIZE];
        self.wram    = [0; WRAM_SIZE];
        self.ppu_reg = [0; PPU_REG_SIZE];
        self.io_reg  = [0; APU_AND_IO_REG_SIZE];
    }
    fn get_dump_size() -> usize {
        0x4020
    }
    fn dump(&self, read_callback: impl Fn(usize, u8)) {
        // 冗長だけどカセット以外の全領域を書き出す
        for addr in 0..0x4020u16 {
            read_callback(usize::from(addr), self.read_u8(addr));
        }
    }
    fn restore(&mut self, write_callback: impl Fn(usize) -> u8) {
        // storeの内容をそのまま戻す
        for addr in 0..0x4020u16 {
            self.write_u8(addr, write_callback(usize::from(addr)));
        }
    }
}