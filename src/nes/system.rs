use super::eram::ExtendedRam;
use super::erom::ExtendedRom;
use super::prom::ProgramRom;
use super::vram::VideoRam;
use super::wram::WorkRam;

use super::interface::{SystemBus, EmulateControl};

/// Memory Access Dispatcher
pub struct System {
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

impl System {
    /// inesファイルから読み出してメモリ上に展開します
    /// 組み込み環境でRAM展開されていなくても利用できるように、多少パフォーマンスを犠牲にしてもclosure経由で読み出します
    pub fn from_ines_binary(&mut self, read_closure: impl Fn(usize) -> u8) -> bool {
        // header check
        if read_closure(0) != 0x4e { // N
            return false;
        }
        if read_closure(1) != 0x45 { // E
            return false;
        }
        if read_closure(2) != 0x53 { // S
            return false;
        }
        if read_closure(3) != 0x1a { // character break
            return false;
        }
        let prg_rom_size    = usize::from(read_closure(4)) * 0x4000; // * 16KB
        let chr_rom_size    = usize::from(read_closure(5)) * 0x2000; // * 8KB
        let flag6           = read_closure(6);
        let is_mirroring_vertical        = (flag6 & 0x01) == 0x01; // hor: CIRAM A10=PPU A11, vert: CIRAM A10 = PPU A10
        let is_exists_battery_backed_ram = (flag6 & 0x02) == 0x02; // battery-backed PRG RAM 0x6000-0x7fff exists
        let is_exists_trainer            = (flag6 & 0x04) == 0x04; // 512byte trainer at 0x7000-0x71ff
        true
    }
}
impl SystemBus for System {
    fn read_u8(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            // WRAM
            self.wram.read_u8(addr % 0x0800) // mirror support
        } else if addr < 0x4000 {
            // PPU I/O
            // TODO: Mirror
            unimplemented!();
        } else if addr < 0x4020 {
            // APU I/O, PAD
            unimplemented!();
        } else if addr < 0x6000 {
            // Extended ROM
            self.erom.read_u8(addr - 0x4020)
        } else if addr < 0x8000 {
            // Extended RAM
            self.eram.read_u8(addr - 0x6000)
        } else {
            // PRG-ROM
            self.prom.read_u8(addr - 0x6000)
        }
    }
    fn write_u8(&mut self, addr: u16, data: u8) {
        if addr < 0x2000 {
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
            self.eram.write_u8(addr - 0x6000, data);
        } else {
            // PRG-ROM
            panic!("Memory Write Request Error. PRG-ROM. addr:{:x}, data:{:x}", addr, data);
        }
    }
}

impl EmulateControl for System {
    fn reset(&mut self){
        self.vram.reset();
        self.wram.reset();
        self.erom.reset();
        self.eram.reset();
        self.prom.reset();
    }
    fn store(&self, _read_callback: impl Fn(usize, u8)){
    }
    fn restore(&mut self, _write_callback: impl Fn(usize) -> u8){
    }
}