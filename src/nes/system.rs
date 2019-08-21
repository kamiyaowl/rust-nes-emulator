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

    /// カセットへのR/W要求は呼び出し先でEmulation, 実機を切り替えるようにする
    /// 引数に渡されるaddrは、CPU命令そのままのアドレスを渡す
    ///  0x4020 - 0x5fff: Extended ROM
    ///  0x6000 - 0x7FFF: Extended RAM
    ///  0x8000 - 0xbfff: PRG-ROM switchable
    ///  0xc000 - 0xffff: PRG-ROM fixed to the last bank or switchable
    pub cassette: Cassette,

    /// $2005 2回書き目はyを書く, $2006 2回目がlower。
    /// $2005, $2006は状態を共有する
    /// $2002を読み出すと、どっちを書くかはリセットされる
    pub ppu_is_second_write: bool, // 初期値falseで
    pub ppu_scroll_y_reg: u8,
    pub ppu_addr_lower_reg: u8,

    /// $4014 OAM DMA
    /// reg自体には転送元pageになる(io_reg[0x4014] << 8)
    /// が、DMAのTriggerになる変数を持たないと辛いので
    pub is_trigger_oam_dam: bool, // default: false
}

impl Default for System {
    fn default() -> Self {
        Self {
            vram:    [0; VRAM_SIZE],
            wram:    [0; WRAM_SIZE],
            ppu_reg: [0; PPU_REG_SIZE],
            io_reg:  [0; APU_AND_IO_REG_SIZE],
            cassette: Default::default(),
            ppu_is_second_write: false,
            ppu_scroll_y_reg: 0,
            ppu_addr_lower_reg: 0,
            is_trigger_oam_dam: false,
        }
    }
}

impl SystemBus for System {
    fn read_u8(&self, addr: u16, is_nondestructive: bool) -> u8 {
        if addr < 0x2000 {
            let index = usize::from(addr) % self.wram.len(); // mirror support
            self.wram[index] 
        } else if addr < 0x4000 {
            let index = usize::from(addr - 0x2000) % self.ppu_reg.len(); // mirror support
            // TODO: is_nondestructiveで処理分岐
            self.ppu_reg[index] 
        } else if addr < 0x4020 {
            // TODO: is_nondestructiveで処理分岐
            let index = usize::from(addr - 0x4000);
            self.io_reg[index] 
        } else {
            self.cassette.read_u8(addr, is_nondestructive)
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
            self.cassette.write_u8(addr, data);
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
    fn dump(&self, _read_callback: impl Fn(usize, u8)) {
        // TODO: #14 破壊読み出しのあるレジスタも強引に値を持ってくる
        unimplemented!();
    }
    fn restore(&mut self, _write_callback: impl Fn(usize) -> u8) {
        // TODO: #14
        unimplemented!();
    }
}