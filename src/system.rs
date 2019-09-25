use super::cassette::*;
use super::interface::*;
use super::pad::*;
use super::video_system::*;

pub const WRAM_SIZE: usize = 0x0800;
pub const PPU_REG_SIZE: usize = 0x0008;
pub const APU_IO_REG_SIZE: usize = 0x0018;
pub const EROM_SIZE: usize = 0x1FE0;
pub const ERAM_SIZE: usize = 0x2000;
pub const PROM_SIZE: usize = 0x8000; // 32KB

pub const WRAM_BASE_ADDR: u16 = 0x0000;
pub const PPU_REG_BASE_ADDR: u16 = 0x2000;
pub const APU_IO_REG_BASE_ADDR: u16 = 0x4000;
pub const CASSETTE_BASE_ADDR: u16 = 0x4020;

/// Memory Access Dispatcher
#[derive(Clone)]
pub struct System {
    /// 0x0000 - 0x07ff: WRAM
    /// 0x0800 - 0x1f7ff: WRAM  Mirror x3
    pub wram: [u8; WRAM_SIZE],
    //  0x2000 - 0x2007: PPU I/O
    //  0x2008 - 0x3fff: PPU I/O Mirror x1023
    pub ppu_reg: [u8; PPU_REG_SIZE],
    //  0x4000 - 0x401f: APU I/O, PAD
    pub io_reg: [u8; APU_IO_REG_SIZE],

    /// カセットへのR/W要求は呼び出し先でEmulation, 実機を切り替えるようにする
    /// 引数に渡されるaddrは、CPU命令そのままのアドレスを渡す
    ///  0x4020 - 0x5fff: Extended ROM
    ///  0x6000 - 0x7FFF: Extended RAM
    ///  0x8000 - 0xbfff: PRG-ROM switchable
    ///  0xc000 - 0xffff: PRG-ROM fixed to the last bank or switchable
    pub cassette: Cassette,

    /// PPUが描画に使うメモリ空間
    pub video: VideoSystem,

    /// コントローラへのアクセスは以下のモジュールにやらせる
    /// 0x4016, 0x4017
    pub pad1: Pad,
    pub pad2: Pad,

    /* PPUのアドレス空間に対する要求トリガ */
    pub written_oam_data: bool,   // OAM_DATAがかかれた
    pub written_ppu_scroll: bool, // PPU_SCROLLが2回書かれた
    pub written_ppu_addr: bool,   // PPU_ADDRが2回書かれた
    pub written_ppu_data: bool,   // PPU_DATAがかかれた
    pub written_oam_dma: bool,    // OAM_DMAが書かれた
    pub read_oam_data: bool,      // OAM_DATAが読まれた
    pub read_ppu_data: bool,      // PPU_DATAが読まれた

    /* 2回海ができるPPU register対応 */
    /// $2005, $2006は状態を共有する、$2002を読み出すと、どっちを書くかはリセットされる
    pub ppu_is_second_write: bool, // 初期値falseで, 2回目の書き込みが分岐するようにtrueにする
    pub ppu_scroll_y_reg: u8,   // $2005
    pub ppu_addr_lower_reg: u8, // $2006
}

impl Default for System {
    fn default() -> Self {
        Self {
            wram: [0; WRAM_SIZE],
            ppu_reg: [0; PPU_REG_SIZE],
            io_reg: [0; APU_IO_REG_SIZE],

            cassette: Default::default(),
            video: Default::default(),
            pad1: Default::default(),
            pad2: Default::default(),

            written_oam_data: false,
            written_ppu_scroll: false,
            written_ppu_addr: false,
            written_ppu_data: false,
            written_oam_dma: false,
            read_oam_data: false,
            read_ppu_data: false,

            ppu_is_second_write: false,
            ppu_scroll_y_reg: 0,
            ppu_addr_lower_reg: 0,
        }
    }
}

impl EmulateControl for System {
    fn reset(&mut self) {
        self.video.reset();
        self.pad1.reset();
        self.pad2.reset();

        self.wram = [0; WRAM_SIZE];
        self.ppu_reg = [0; PPU_REG_SIZE];
        self.io_reg = [0; APU_IO_REG_SIZE];

        self.written_oam_data = false;
        self.written_ppu_scroll = false;
        self.written_ppu_addr = false;
        self.written_ppu_data = false;
        self.written_oam_dma = false;
        self.read_oam_data = false;
        self.read_ppu_data = false;

        self.ppu_is_second_write = false;
        self.ppu_scroll_y_reg = 0;
        self.ppu_addr_lower_reg = 0;
    }
}

impl SystemBus for System {
    fn read_u8(&mut self, addr: u16, is_nondestructive: bool) -> u8 {
        if addr < PPU_REG_BASE_ADDR {
            // mirror support
            let index = usize::from(addr) % self.wram.len();
            arr_read!(self.wram, index)
        } else if addr < APU_IO_REG_BASE_ADDR {
            // mirror support
            let index = usize::from(addr - PPU_REG_BASE_ADDR) % self.ppu_reg.len();
            debug_assert!(index < 0x9);
            match index {
                // PPU_STATUS 2度書きレジスタの状態をリセット, VBLANKフラグをクリア
                0x02 => {
                    let data = self.ppu_reg[index]; // 先にフェッチしないとあかんやんけ
                    if !is_nondestructive {
                        self.ppu_is_second_write = false;
                        self.write_ppu_is_vblank(false);
                    }
                    data
                }
                // OAM_DATAの読み出しフラグ
                0x04 => {
                    if !is_nondestructive {
                        self.read_oam_data = true;
                    }
                    arr_read!(self.ppu_reg, index)
                }
                // PPU_DATA update/address incrementのためにフラグを立てる
                // バッファが入るので1step遅れで結果が入る
                0x07 => {
                    if !is_nondestructive {
                        self.read_ppu_data = true;
                    }
                    arr_read!(self.ppu_reg, index)
                }
                // default
                _ => arr_read!(self.ppu_reg, index),
            }
        } else if addr < CASSETTE_BASE_ADDR {
            let index = usize::from(addr - APU_IO_REG_BASE_ADDR);
            if !is_nondestructive {
                match index {
                    // TODO: APU
                    0x16 => self.pad1.read_out(), // pad1
                    0x17 => self.pad2.read_out(), // pad2
                    _ => arr_read!(self.io_reg, index),
                }
            } else {
                arr_read!(self.io_reg, index)
            }
        } else {
            self.cassette.read_u8(addr, is_nondestructive)
        }
    }
    fn write_u8(&mut self, addr: u16, data: u8, is_nondestructive: bool) {
        if addr < PPU_REG_BASE_ADDR {
            // mirror support
            let index = usize::from(addr) % self.wram.len();
            arr_write!(self.wram, index, data);
        } else if addr < APU_IO_REG_BASE_ADDR {
            // mirror support
            let index = usize::from(addr - PPU_REG_BASE_ADDR) % self.ppu_reg.len();
            match index {
                // $2004 OAM_DATAに書いたら書き込みフラグを立てる(使わないだろうけど)
                0x04 => {
                    if !is_nondestructive {
                        self.written_oam_data = true
                    }
                    arr_write!(self.ppu_reg, index, data);
                }
                // $2005 PPU_SCROLL 2回書き
                0x05 => {
                    if self.ppu_is_second_write {
                        self.ppu_scroll_y_reg = data;
                        if !is_nondestructive {
                            self.ppu_is_second_write = false;
                            // PPUに通知
                            self.written_ppu_scroll = true;
                        }
                    } else {
                        arr_write!(self.ppu_reg, index, data);
                        if !is_nondestructive {
                            self.ppu_is_second_write = true;
                        }
                    }
                }
                // $2006 PPU_ADDR 2回書き
                0x06 => {
                    if self.ppu_is_second_write {
                        self.ppu_addr_lower_reg = data;
                        if !is_nondestructive {
                            self.ppu_is_second_write = false;
                            // PPUに通知
                            self.written_ppu_addr = true;
                        }
                    } else {
                        arr_write!(self.ppu_reg, index, data);
                        if !is_nondestructive {
                            self.ppu_is_second_write = true;
                        }
                    }
                }
                // $2007 PPU_DATA addr autoincrement
                0x07 => {
                    arr_write!(self.ppu_reg, index, data);
                    if !is_nondestructive {
                        // PPUに書いてもらおう
                        self.written_ppu_data = true;
                    }
                }
                // default
                _ => {
                    arr_write!(self.ppu_reg, index, data);
                }
            };
        } else if addr < CASSETTE_BASE_ADDR {
            let index = usize::from(addr - APU_IO_REG_BASE_ADDR);
            if !is_nondestructive {
                match index {
                    // TODO: APU
                    0x14 => self.written_oam_dma = true, // OAM DMA
                    0x16 => self.pad1.write_strobe((data & 0x01) == 0x01), // pad1
                    0x17 => self.pad2.write_strobe((data & 0x01) == 0x01), // pad2
                    _ => {}
                }
            }
            arr_write!(self.io_reg, index, data);
        } else {
            self.cassette.write_u8(addr, data, is_nondestructive);
        }
    }
}
