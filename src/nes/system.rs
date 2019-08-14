use super::cpu::Cpu;
use super::eram::ExtendedRam;
use super::erom::ExtendedRom;
use super::prom::ProgramRom;
use super::vram::VideoRam;
use super::wram::WorkRam;

use super::interface::{SystemBus, EmulateControl};

/// Memory Access Dispatcher
pub struct System {
    /// CPU
    pub cpu: Cpu,
    /// Video RAM
    pub vram: VideoRam,

    /// 0x0000 - 0x07ff: WRAM 
    /// 0x0800 - 0x1f7ff: WRAM  Mirror x3
    pub wram: WorkRam,
    //  0x2000 - 0x2007: PPU I/O
    //  0x2008 - 0x3fff: PPU I/O Mirror x1023
    
    //  0x4000 - 0x401f: APU I/O, PAD

    ///  0x4020 - 0x5fff: Extended ROM
    pub erom: ExtendedRom,
    /// 0x6000 - 0x7FFF: Extended RAM
    pub eram: ExtendedRam,
    //  0x8000 - 0xbfff, 0xc000 - 0xffff: PRG-ROM
    pub prom: ProgramRom,
}

impl SystemBus for System {
    fn read_u8(&self, addr: usize) -> u8 {
        if addr < 0x0800 {
            // WRAM
            return self.wram.read_u8((addr) % 0x0800 as usize); // mirror support
        } else if addr < 0x4000 {
            // PPU I/O
            // TODO: Mirror
            unimplemented!();
        } else if addr < 0x4020 {
            // APU I/O, PAD
            unimplemented!();
        } else if addr < 0x6000 {
            // Extended ROM
            return self.erom.read_u8((addr - 0x4020) as usize);
        } else if addr < 0x8000 {
            // Extended RAM
            return self.eram.read_u8((addr - 0x6000) as usize);
        } else if addr < 0x10000 {
            // PRG-ROM
            return self.prom.read_u8((addr - 0x6000) as usize);
        } else {
            panic!("Memory Read Request Error. Out of Index. addr:{:x}", addr);
        }
    }
    fn write_u8(&mut self, addr: usize, data: u8) {
        if addr < 0x0800 {
            self.wram.write_u8(addr, data);
        } else if addr < 0x4000 {
            // PPU I/O
            unimplemented!();
        } else if addr < 0x4020 {
            // APU I/O, PAD
            unimplemented!();
        } else if addr < 0x6000 {
            // Extended ROM
            panic!("Memory Write Request Error. Extended ROM. addr:{:x}, data:{:x}", addr, data);
        } else if addr < 0x8000 {
            // Extended RAM
            self.eram.write_u8((addr - 0x6000) as usize, data);
        } else if addr < 0x10000 {
            // PRG-ROM
            panic!("Memory Write Request Error. PRG-ROM. addr:{:x}, data:{:x}", addr, data);
        } else {
            panic!("Memory Write Request Error. Out of Index. addr:{:x}, data:{:x}", addr, data);
        }
    }
}

impl EmulateControl for System {
    fn reset(&mut self){
        self.cpu.reset();
        self.vram.reset();
        self.wram.reset();
        self.erom.reset();
        self.eram.reset();
        self.prom.reset();
    }
    fn store(&self, _read_callback: fn(usize, u8)){
        unimplemented!();
    }
    fn restore(&mut self, _write_callback: fn(usize) -> u8){
        unimplemented!();
    }
}