pub trait MemoryIo {
    fn read_u8(&self, addr: usize) -> u8;
    fn write_u8(&mut self, addr: usize, data: u8);
}

/// Memory Access Dispatcher
pub struct Memory {
    /// 0x0000 - 0x07ff: WRAM 
    wram: [u8; 0x800],
    //  0x0800 - 0x1f7ff: WRAM  Mirror x3
    //  0x2000 - 0x2007: PPU I/O
    //  0x2008 - 0x3fff: PPU I/O Mirror x1023
    //  0x4000 - 0x401f: APU I/O, PAD

    //  0x4020 - 0x5fff: Extended ROM
    /// 0x6000 - 0x7FFF: Extended RAM
    eram: [u8; 0x2000],

    //  0x8000 - 0xbfff, 0xc000 - 0xffff: PRG-ROM

}

impl MemoryIo for Memory {
    fn read_u8(&self, addr: usize) -> u8 {
        if addr < 0x0800 {
            return self.wram[addr];
        } else if addr < 0x4000 {
            // PPU I/O
            unimplemented!();
        } else if addr < 0x4020 {
            // APU I/O, PAD
            unimplemented!();
        } else if addr < 0x6000 {
            // Extended ROM
            unimplemented!();
        } else if addr < 0x8000 {
            // Extended RAM
            return self.eram[(addr - 0x8000) as usize];
        } else if addr < 0x10000 {
            // PRG-ROM
            unimplemented!();
        } else {
            panic!("Memory Read Request Error. addr:{:x}", addr);
        }
    }
    fn write_u8(&mut self, addr: usize, data: u8) {
        if addr < 0x0800 {
            self.wram[addr] = data;
        } else if addr < 0x4000 {
            // PPU I/O
            unimplemented!();
        } else if addr < 0x4020 {
            // APU I/O, PAD
            unimplemented!();
        } else if addr < 0x6000 {
            // Extended ROM
            unimplemented!();
        } else if addr < 0x8000 {
            // Extended RAM
            self.eram[(addr - 0x8000) as usize] = data;
        } else if addr < 0x10000 {
            // PRG-ROM
            unimplemented!();
        } else {
            panic!("Memory Write Request Error. addr:{:x}", addr);
        }
    }
}